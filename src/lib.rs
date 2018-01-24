#![feature(lang_items)]
#![no_std]

#[lang = "eh_personality"]
extern fn eh_personality() {
}

#[lang = "panic_fmt"]
extern fn rust_begin_panic() -> ! {
    loop {}
}

extern crate rlibc;
extern crate vga;

#[no_mangle]
pub extern fn kmain() -> ! {

    // clear screen
    let mut buffer = [0x1fu8; 80*50];
    for i in 0..(80*25) {
        buffer[i*2] = 0x00u8;
    }
    let buffer_ptr = (0xb8000) as *mut _;
    unsafe { *buffer_ptr = buffer};

    // print guest string
    let prompt = b"Welcome to Japari Park"; // length 22

    let mut prompt_buf = [0x1fu8; 44];
    for (i, char_byte) in prompt.into_iter().enumerate() {
        prompt_buf[i*2] = *char_byte;
    }
    let buffer_ptr = (0xb8000) as *mut _;
    unsafe { *buffer_ptr = prompt_buf };

    vga::print_something();
    loop {}
}
