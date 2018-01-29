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

pub mod idt;
mod dtables;

use spin::Mutex;
use core::intrinsics;

use dtables::DescriptorTablePointer;
use idt::IdtEntry;
use device::pic;
use device::keyboard;
use device::io::inb;

#[macro_export]
macro_rules! scratch_push {
    () => (asm!(
        "push rax
        push rcx
        push rdx
        push rdi
        push rsi
        push r8
        push r9
        push r10
        push r11"
        : : : : "intel", "volatile"
    ));
}

#[macro_export]
macro_rules! scratch_pop {
    () => (asm!(
        "pop r11
        pop r10
        pop r9
        pop r8
        pop rsi
        pop rdi
        pop rdx
        pop rcx
        pop rax"
        : : : : "intel", "volatile"
    ));
}

#[macro_export]
macro_rules! preserved_push {
    () => (asm!(
        "push rbx
        push rbp
        push r12
        push r13
        push r14
        push r15"
        : : : : "intel", "volatile"
    ));
}

#[macro_export]
macro_rules! preserved_pop {
    () => (asm!(
        "pop r15
        pop r14
        pop r13
        pop r12
        pop rbp
        pop rbx"
        : : : : "intel", "volatile"
    ));
}


#[macro_export]
macro_rules! iret {
    () => (asm!(
        "iretq"
        : : : : "intel", "volatile"
    ));
}

#[macro_export]
macro_rules! make_idt_entry {
    ($index:expr, $name:ident, $body:expr) => {{
        fn body() {
            $body
        }

        #[naked]
        unsafe extern fn $name() {
            scratch_push!();
            preserved_push!();
            asm!("mov rsi, rsp
                  push rsi

                  cli

                  call $0

                  sti

                  add rsp, 8" :: "s"(body as fn()) :: "volatile", "intel");
            preserved_pop!();
            scratch_pop!();
            iret!();

            intrinsics::unreachable();
        }

        IDT.lock()[$index].set_func($name as usize);
    }};
}

// The Interrupt Descriptor Table
// The CPU will look at this table to find the appropriate interrupt handler.
pub  static IDT: Mutex<[IdtEntry; 256]> = Mutex::new([IdtEntry::new(); 256]);

// Enable Interrupts.
pub unsafe fn enable() {
    asm!("sti");
}

// Disable Interrupts.
pub unsafe fn disable() {
    asm!("cli");
}

pub fn init() {

    let ptr: DescriptorTablePointer =
        DescriptorTablePointer::new_idtp(&IDT.lock()[..]);

    unsafe { dtables::lidt(&ptr) };

    make_idt_entry!(32, isr32, {
        pic::send_eoi(32);
    });

    make_idt_entry!(33, isr33, {
        let scancode = unsafe { inb(0x60) };

        if let Some(c) = keyboard::from_scancode(scancode as usize) {
            kprint!("{}", c);
        }

        pic::send_eoi(33);
    });

    unsafe { enable(); }
}
