#![no_std]
#![feature(const_fn)]
#![feature(alloc)]

mod fat32;

#[macro_use]
extern crate vga;
extern crate device;
extern crate alloc;

use device::ata::ata;

pub fn detect() {
    let fat32 = unsafe { fat32::Fat32::new(&ata) };
    kprintln!("{:?}", fat32.ebpb);
}
