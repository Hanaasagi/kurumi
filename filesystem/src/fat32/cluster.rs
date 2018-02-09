use super::{Fat32, Disk};

pub type Cluster = u32;

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum FatEntry {
    Node(Cluster),
    End,
    BadBlock,
}

pub struct ClusterChain<'a> {
    current_entry: FatEntry,
    fat: &'a Fat32,
    drive: &'a Disk,
}

impl <'a> ClusterChain<'a> {
    pub const fn new(cluster: Cluster, fat: &'a Fat32, drive: &'a Disk) -> Self {
        ClusterChain {
            current_entry: FatEntry::Node(cluster),
            fat: fat,
            drive: drive,
        }
    }

    fn check_end(&self, cluster: Cluster) -> bool {
         if cluster >= self.fat.get_total_clusters() {
             return true;
         }
         false
    }

    // follow https://wiki.osdev.org/FAT#FAT_32_3
    fn read_entry(&self, current: Cluster) -> FatEntry {
         if self.check_end(current) {
             return FatEntry::End;
         }

        let first_fat_sector = self.fat.ebpb.bpb.reserved_sectors_count as u32;
        let sector_size = self.fat.ebpb.bpb.bytes_per_sector as usize;
        let mut fat_table = vec![0u8; sector_size];
        let fat_offset = current * 4;
        let fat_sector = first_fat_sector + (fat_offset / sector_size as u32);
        let ent_offset = fat_offset % sector_size as u32;

        // at this point you need to read from sector "fat_sector" on the disk into "FAT_table".
        let table_value = unsafe {
            self.drive.read(fat_sector as u64, &mut fat_table)
                .expect("Disk Read Error");
            let table_reference = &fat_table[ent_offset as usize] as *const u8 as *const u32;
            // ignore the high 4 bits.
            *table_reference & 0x0FFFFFFF
        };

        // If "table_value" is greater than or equal to (>=) 0x0FFFFFF8 then there are no more
        // clusters in the chain. This means that the whole file has been read. If "table_value"
        // equals (==) 0x0FFFFFF7 then this cluster has been marked as "bad". "Bad" clusters are
        // prone to errors and should be avoided. If "table_value" is not one of the above cases
        // then it is the cluster number of the next cluster in the file.
        if table_value >= 0x0FFFFFF8 {
            return FatEntry::End;
        } else if table_value == 0x0FFFFFF7 {
            return FatEntry::BadBlock;
        }

        FatEntry::Node(table_value)
    }
}

impl <'a> Iterator for ClusterChain <'a> {
    type Item = Cluster;

    fn next(&mut self) -> Option<Self::Item> {
        let current_index = match self.current_entry {
            FatEntry::Node(current_cluster) => current_cluster,
            _ => return None,
        };
        self.current_entry = self.read_entry(current_index);

        Some(current_index)
    }
}
