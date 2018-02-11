#![no_std]
#![feature(const_fn)]
#![feature(alloc)]

#[macro_use]
extern crate vga;
extern crate device;

#[macro_use]
extern crate alloc;
#[macro_use]
extern crate bitflags;

mod fat32;
mod file;

use alloc::Vec;
use device::ata::ata;
use device::disk::Disk;
use file::{File, FileMode, FilePointer, FileDescriptor};

trait FileSystem {
    type FileType: File;

    fn open_file(&self, drive: &Disk, file_name: &str) -> Option<FilePointer<Self::FileType>>;
    fn read_file(&self, drive: &Disk, file_pointer: &FilePointer<Self::FileType>, buffer: &mut [u8]) -> Option<usize>;
}

struct FsManager<'a, T: 'a + FileSystem> {
    filesystem: &'a T,
    drive: &'a Disk,
    descriptors: Vec<FileDescriptor<T::FileType>>,
}

impl <'a, T: FileSystem>  FsManager<'a, T> {
    fn open_file(&mut self, file_name: &str) -> Option<u16> {
        if let Some(file_pointer) = self.filesystem.open_file(self.drive, file_name) {
            let mut min_index = self.descriptors.len();
            for (index, descriptor) in self.descriptors.iter().enumerate() {
                if descriptor.get_id() as usize > index {
                    min_index = index;
                    break;
                }
            }

            let descriptor = FileDescriptor::new(
                min_index as u16,
                FileMode::ReadWrite,
                file_pointer
            );

            self.descriptors.insert(min_index, descriptor);
            return Some(min_index as u16);
        }
        None
    }

    fn close_descriptor(&mut self, descriptor: u16) {
        let result = self.descriptors.iter().position(|x| x.get_id() == descriptor);
        if let Some(index) = result {
            self.descriptors.remove(index);
        } else {
            kprintln!("Descriptor {} is not open.", descriptor);
        }
    }

    // TODO read folder

    fn read_file(&mut self, descriptor: u16, buffer: &mut [u8]) -> Option<usize> {
        if let Some(descriptor) = self.descriptors.iter_mut().find(|x| x.get_id() == descriptor) {
            let file_pointer = descriptor.get_pointer_mut();
            if let Some(size) = self.filesystem.read_file(self.drive, file_pointer, buffer) {
                file_pointer.advance_pointer(size);
                return Some(size);
            } else {
                kprintln!("Unable to read file.");
            }
        } else {
            kprintln!("Descriptor {} is not open.", descriptor);
        }
        None
    }
}

pub fn test_read() {
    let fat32 = unsafe { fat32::Fat32::new(&ata) };
    kprintln!("{:?}", fat32.ebpb);
    let mut fat = FsManager {
        filesystem: &fat32,
        drive: &ata,
        descriptors: Vec::new(),
    };

    let path = "README";
    kprintln!("Opening file \"{}\".", path);
    if let Some(opened_descriptor) = fat.open_file(path) {
        //kprintln!("Printing contents of file:");

        let mut buffer = [0u8; 512];
        let size = fat.read_file(opened_descriptor, &mut buffer).unwrap_or(0);
        let buffer = &buffer[0..size];
        //kprintln!("file size {}", size);
        use core::str;
        kprintln!("{}", str::from_utf8(&buffer).expect("Invalid UTF-8."));
        fat.close_descriptor(opened_descriptor);
    }
}
