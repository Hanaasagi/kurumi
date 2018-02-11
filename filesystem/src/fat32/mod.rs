mod bpb;
mod directory;
mod cluster;

use core::slice;
use core::str::Split;
use alloc::Vec;
use alloc::string::String;

use self::directory::{Directory, LongFileName, FatDirectory};
use self::bpb::{Bpb, Ebpb};
use self::cluster::{Cluster, ClusterChain};
use super::{File, FilePointer, FileSystem};
use device::disk::Disk;

// TODO some img file START_SECTOR is not zero
const START_SECTOR: u64 = 0;

pub struct Fat32 {
    pub ebpb: Ebpb,
}

// transfer C code from os-dev wiki https://wiki.osdev.org/FAT#Programming_Guide
impl Fat32 {
    pub unsafe fn new(disk: &Disk) -> Self {
        let mut boot_record = [0u8; 512];
        disk.read(START_SECTOR, &mut boot_record)
            .expect("EBPB Read Error");
        let ebpb = (*(boot_record.as_ptr() as *const Ebpb));
        Fat32 {
            ebpb: ebpb,
        }
    }

    #[inline]
    fn get_fat_size(&self) -> u32 {
        if self.ebpb.bpb.sectors_per_fat == 0 {
            return self.ebpb.sectors_per_fat;
        }
        self.ebpb.bpb.sectors_per_fat as u32
    }

    // always be zero in FAT32
    #[inline]
    fn get_root_dir_sector(&self) -> u32 {
        //((self.ebpb.bpb.root_entry_count * 32) + (self.ebpb.bpb.bytes_per_sector - 1)) / self.ebpb.bpb.bytes_per_sector
        0
    }

    #[inline]
    fn get_first_data_sector(&self) -> u64 {
        self.ebpb.bpb.reserved_sectors_count as u64 +
            (self.ebpb.bpb.table_count as u32 * self.get_fat_size()) as u64 +
            self.get_root_dir_sector() as u64
    }

    #[inline]
    fn get_bytes_in_cluster(&self) -> u32 {
        self.ebpb.bpb.sectors_per_cluster as u32 * self.ebpb.bpb.bytes_per_sector as u32
    }

    #[inline]
    fn first_sector_of_cluster(&self, cluster: u32) -> u64 {
        self.get_first_data_sector() + ((cluster-2) * (self.ebpb.bpb.sectors_per_cluster as u32)) as u64
    }

    #[inline]
    fn get_total_clusters(&self) -> u32 {
        let data_sectors = self.ebpb.bpb.total_sectors_large as usize - (self.ebpb.bpb.reserved_sectors_count as usize + self.ebpb.bpb.table_count as usize *32);
        data_sectors as u32 / self.ebpb.bpb.sectors_per_cluster as u32
    }

    // see https://wiki.osdev.org/FAT#Programming_Guide、0
    fn read_directories_from_cluster(&self, drive: &Disk, cluster: Cluster, directories: &mut Vec<Directory>) {
        let mut temp_name: Option<String> = None;
        let mut buffer = vec![0u8; self.get_bytes_in_cluster() as usize];

        let sector = self.first_sector_of_cluster(cluster);
        kprintln!("sector {:?}", sector);
        let sectors_read = unsafe {
            drive.read(sector, &mut buffer)
        }.expect("Disk Read Error") as usize;
        //kprintln!("{:?}", &buffer[0..3]);

        //for i in 0..1000000000{
            //if i % 512 != 0 {
                //continue;
            //}
            //let mut buffer = vec![0u8; self.get_bytes_in_cluster() as usize];
            //let sectors_read = unsafe {
                //drive.read(i, &mut buffer)
            //}.expect("Error reading from disk.") as usize;
            //if buffer[0] == 0 {
                //continue;
            //}
            //kprintln!("{} {:?} ", i, &buffer[0..3]);
        //}
        //panic!();
        let directories_slice = unsafe {
            slice::from_raw_parts(buffer.as_ptr() as *const FatDirectory,
            (sectors_read * self.ebpb.bpb.bytes_per_sector as usize / 32) as usize)
        };
        kprintln!("len {:?}", directories_slice.len());
        //kprintln!("slice {:?}", directories_slice);

        for directory in directories_slice {
            // If the first byte of the directory entry is 0, there are no more directories.
            if directory.name[0] == 0 {
                break;
            }
            // If the first byte of the entry is equal to 0xE5 then the entry is unused.
            if directory.name[0] == 0xE5 {
                continue;
            }

            //kprintln!("name {:?}", directory.name);

            if directory.is_lfn() {
                let lfn_directory = unsafe {
                    *(directory as *const _ as *const LongFileName)
                };
                let long_file_name = lfn_directory.get_name();
                if temp_name != None {
                    // If a long file name is in the buffer and the current directory is another long file name,
                    // apply it to the previously stored file name.
                    temp_name = Some(format!("{}{}", long_file_name, temp_name.unwrap()));
                } else {
                    temp_name = Some(long_file_name);
                }
            } else {
                if let Some(stored_name) = temp_name {
                    directories.push(Directory::new(stored_name, *directory));
                    temp_name = None;
                } else {
                    directories.push(Directory::new(directory.get_short_name(), *directory));
                }
            }
        }
    }

    fn read_cluster_chain(&self, drive: &Disk, first_cluster: u32, directories: &mut Vec<Directory>) {
        let cluster_chain = ClusterChain::new(first_cluster, self, drive);
        for cluster in cluster_chain {
            self.read_directories_from_cluster(drive, cluster, directories);
        }
    }

    fn read_folder(&self, drive: &Disk, cluster: u32) -> Vec<Directory> {
        let mut directories: Vec<Directory> = Vec::new();
        self.read_cluster_chain(drive, cluster, &mut directories);
        directories
    }

    fn find_file(&self, drive: &Disk, cluster: u32, path: &mut Split<&str>) -> Option<Directory> {
        if let Some(part) = path.next() {
            let current_dirs = self.read_folder(drive, cluster);
            //kprintln!("current_dirs: {:?}", current_dirs);
            //kprintln!("path part : {:?}", part);
            let dir: Directory = current_dirs
                    .iter()
                    .find(|dir| dir.get_name() == part)
                    .expect(&format!("Folder {} not found.", part))
                    .clone();
            if dir.get_fat_dir().is_folder() {
                return self.find_file(drive, dir.get_fat_dir().get_cluster(), path);
            }
            return Some(dir);
        }
        None
    }
}

impl FileSystem for Fat32 {
    type FileType = Directory;

    fn open_file(&self, drive: &Disk, file_name: &str) -> Option<FilePointer<Directory>> {
        let mut path_pattern = file_name.split("/");
        if let Some(file) = self.find_file(drive, self.ebpb.root_dir_cluster, &mut path_pattern) {
           return Some(FilePointer::new(file, 0));
        }
        None
    }

    fn read_file(&self, drive: &Disk, file_pointer: &FilePointer<Self::FileType>, buffer: &mut [u8]) -> Option<usize> {
        let read_length = buffer.len();
        let file = file_pointer.get_file();
        let file_size = file.get_size();
        let read_start = file_pointer.get_pos();
        let cluster_size = self.get_bytes_in_cluster() as usize;

        if read_length % cluster_size == 0 {
            let starting_cluster = file.get_fat_dir().get_cluster();
            let mut cluster_chain = ClusterChain::new(starting_cluster, self, drive);
            let mut current_cluster_index = read_start/cluster_size;

            if let Some(mut current_cluster) = cluster_chain.nth(current_cluster_index) {
                let mut part = 0;
                while (read_start + part*cluster_size < file_size) || (part*cluster_size < read_length) {
                    let mut temp_buffer = vec![0u8; cluster_size];
                    unsafe {
                        drive.read(self.first_sector_of_cluster(current_cluster), &mut temp_buffer);
                    }

                    buffer[part*cluster_size..(part+1)*cluster_size].clone_from_slice(&temp_buffer);
                    part += 1;
                    if let Some(next_cluster) = cluster_chain.next() {
                        current_cluster = next_cluster;
                    } else {
                        break;
                    }
                }
                // TODO
                return Some(file_size - read_start);
            }
        }
        None
    }
}
