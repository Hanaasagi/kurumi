#![no_std]
#![feature(asm)]
#![feature(lang_items)]

#[lang = "eh_personality"]
#[no_mangle]
pub extern fn eh_personality() {
}

#[lang = "panic_fmt"]
#[no_mangle]
pub extern fn rust_begin_panic() -> ! {
    loop {}
}

extern crate rlibc;
#[macro_use]
extern crate vga;

extern crate device;
use device::pic;

extern crate interrupts;

#[no_mangle]
pub extern fn kmain() -> ! {
    vga::clear_screen();
    kprintln!("booting ...");
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
