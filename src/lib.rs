#![feature(lang_items)]
#![no_std]

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

extern crate drivers;
use drivers::pic;

#[no_mangle]
pub extern fn kmain() -> ! {
    pic::remap();
    vga::clear_screen();
    kprintln!("Welcome to Japari Park");
    kprintln!("{}", "test format");
    loop {}
}
