#![no_std]
#![feature(asm)]
#![feature(naked_functions)]
#![feature(const_fn)]
#![feature(core_intrinsics)]

#[macro_use]
extern crate bitflags;
extern crate spin;

#[macro_use]
extern crate vga;
extern crate device;

#[macro_use]
pub mod macros;
pub mod idt;
mod dtables;

use spin::Mutex;
use core::intrinsics;

use idt::IdtEntry;
use dtables::DescriptorTablePointer;
use device::pic;
use device::keyboard;
use device::io::inb;

// The Interrupt Descriptor Table
// The CPU will look at this table to find the appropriate interrupt handler.
pub static IDT: Mutex<[IdtEntry; 256]> = Mutex::new([IdtEntry::new(); 256]);


pub fn init() {

    let ptr: DescriptorTablePointer =
        DescriptorTablePointer::new_idtp(&IDT.lock()[..]);

    unsafe { dtables::lidt(&ptr) };

    interrupt!(isr32, {
        pic::send_eoi(32);
    });

    interrupt!(isr33, {
        let scancode = unsafe { inb(0x60) };

        if let Some(c) = keyboard::from_scancode(scancode as usize) {
            kprint!("{}", c);
        }

        pic::send_eoi(33);
    });

    // IDT Table
    IDT.lock()[32].set_func(isr32);
    IDT.lock()[33].set_func(isr33);

    unsafe { sti!() }
}
