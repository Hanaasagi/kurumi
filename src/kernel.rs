#![no_std]
#![feature(asm)]
#![feature(lang_items)]

#[macro_use]
extern crate vga;
extern crate interrupt;
extern crate device;
extern crate rlibc;
extern crate multiboot2;

use device::pic;

#[no_mangle]
pub extern fn kmain(multiboot_info_addr: usize) -> ! {
    vga::clear_screen();
    kprintln!("Booting ...");
    pic::remap();                   kprintln!("PIC INIT        {:>64}", "[ok]");
    interrupt::init();              kprintln!("INTERRUPT INIT  {:>64}", "[ok]");
    kprintln!(r"
| | ___   _ _ __ _   _ _ __ ___ (_)
| |/ | | | | '__| | | | '_ ` _ \| |
|   <| |_| | |  | |_| | | | | | | |
|_|\_\\__,_|_|   \__,_|_| |_| |_|_|
    ");

    show_sys_info(multiboot_info_addr);

    kprint!("$ ");
    loop {}
}

fn show_sys_info(multiboot_info_addr: usize) {
    for _ in 0..80 { kprint!("="); }
    let boot_info = unsafe{ multiboot2::load(multiboot_info_addr) };
    let memory_map_tag = boot_info.memory_map_tag()
        .expect("Memory map tag required");

    kprintln!("memory areas:");
    for area in memory_map_tag.memory_areas() {
        kprintln!("    start: 0x{:x}, length: 0x{:x}",
                  area.base_addr, area.length);
    }

    let elf_sections_tag = boot_info.elf_sections_tag()
        .expect("Elf-sections tag required");

    kprintln!("kernel sections:");
    for section in elf_sections_tag.sections() {
        kprintln!("    addr: 0x{:x}, size: 0x{:x}, flags: 0x{:x}",
                  section.addr, section.size, section.flags);
    }

    let kernel_start = elf_sections_tag.sections().map(|s| s.addr)
        .min().unwrap();
    let kernel_end = elf_sections_tag.sections().map(|s| s.addr + s.size)
        .max().unwrap();

    let multiboot_start = multiboot_info_addr;
    let multiboot_end = multiboot_start + (boot_info.total_size as usize);

    kprintln!("kernel starts at 0x{:x}, ends at 0x{:x}",
              kernel_start, kernel_end);
    kprintln!("multiboot starts at 0x{:x}, ends at 0x{:x}",
              multiboot_start, multiboot_end);

    for _ in 0..80 { kprint!("="); }
}


#[lang = "eh_personality"]
#[no_mangle]
pub extern fn eh_personality() {

}

#[lang = "panic_fmt"]
#[no_mangle]
pub extern fn rust_begin_panic(msg: core::fmt::Arguments,
                               file: &'static str, line: u32) -> ! {
    kprintln!(r"
Kernel Panic in file {} at Line {}:
{}", file, line, msg);
    loop {}
}
