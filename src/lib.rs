#![no_std]
#![feature(asm)]
#![feature(lang_items)]

#[macro_use]
extern crate vga;
extern crate interrupts;
extern crate device;
extern crate rlibc;

use device::pic;

#[no_mangle]
pub extern fn kmain() -> ! {
    vga::clear_screen();
    kprintln!("Booting ...");
    pic::remap();                   kprintln!("PIC INIT        {:>64}", "[ok]");
    interrupts::init();             kprintln!("INTERRUPT INIT  {:>64}", "[ok]");
    kprintln!(r"
| | ___   _ _ __ _   _ _ __ ___ (_)
| |/ | | | | '__| | | | '_ ` _ \| |
|   <| |_| | |  | |_| | | | | | | |
|_|\_\\__,_|_|   \__,_|_| |_| |_|_|
    ");

    kprint!("$ ");
    loop {}
}

#[lang = "eh_personality"]
#[no_mangle]
pub extern fn eh_personality() {
}

#[lang = "panic_fmt"]
#[no_mangle]
pub extern fn rust_begin_panic() -> ! {
    loop {}
}

