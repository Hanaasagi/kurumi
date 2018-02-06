#![no_std]
#![feature(asm)]
#![feature(alloc)]
#![feature(lang_items)]
#![feature(global_allocator)]

#[macro_use]
extern crate vga;
extern crate interrupt;
extern crate device;
extern crate memory;

#[macro_use]
extern crate alloc; /* format */
extern crate rlibc;
extern crate multiboot2;
extern crate linked_list_allocator;
extern crate x86_64;

use device::pic;
use linked_list_allocator::LockedHeap;

const HEAP_START: usize = 0o_000_001_000_000_0000;
const HEAP_SIZE:  usize = 100 * 1024; // 100 KiB

#[global_allocator]
static HEAP_ALLOCATOR: LockedHeap = LockedHeap::empty();

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

    //show_sys_info(multiboot_info_addr);

    enable_nxe_bit();
    enable_write_protect_bit();

    let boot_info = unsafe{ multiboot2::load(multiboot_info_addr) };
    memory::init(boot_info, HEAP_START, HEAP_SIZE);
    unsafe { HEAP_ALLOCATOR.lock().init(HEAP_START, HEAP_SIZE); }
    for _ in 0..10000 {
        format!("Some String");
    }

    kprint!("$ ");
    loop {}
}

fn enable_nxe_bit() {
    use x86_64::registers::msr::{IA32_EFER, rdmsr, wrmsr};

    let nxe_bit = 1 << 11;
    unsafe {
        let efer = rdmsr(IA32_EFER);
        wrmsr(IA32_EFER, efer | nxe_bit);
    }
}

fn enable_write_protect_bit() {
    use x86_64::registers::control_regs::{cr0, cr0_write, Cr0};

    unsafe { cr0_write(cr0() | Cr0::WRITE_PROTECT) };
}

#[allow(dead_code)]
fn show_sys_info(multiboot_info_addr: usize) {
    for _ in 0..80 { kprint!("="); }
    let boot_info = unsafe{ multiboot2::load(multiboot_info_addr) };
    let memory_map_tag = boot_info.memory_map_tag()
        .expect("Memory map tag required");

    kprintln!("memory areas:");
    for area in memory_map_tag.memory_areas() {
        kprintln!("    start: 0x{:08x}, length: 0x{:08x}",
                  area.base_addr, area.length);
    }

    let elf_sections_tag = boot_info.elf_sections_tag()
        .expect("Elf-sections tag required");

    kprintln!("kernel sections:");
    for section in elf_sections_tag.sections() {
        kprintln!("    addr: 0x{:08x}, size: 0x{:08x}, flags: 0x{:08x}",
                  section.addr, section.size, section.flags);
    }

    let kernel_start = elf_sections_tag.sections().map(|s| s.addr)
        .min().unwrap();
    let kernel_end = elf_sections_tag.sections().map(|s| s.addr + s.size)
        .max().unwrap();

    let multiboot_start = multiboot_info_addr;
    let multiboot_end = multiboot_start + (boot_info.total_size as usize);

    kprintln!("kernel starts at 0x{:08x}, ends at 0x{:08x}",
              kernel_start, kernel_end);
    kprintln!("multiboot starts at 0x{:08x}, ends at 0x{:08x}",
              multiboot_start, multiboot_end);

    let mut frame_allocator = memory::AreaFrameAllocator::new(
        kernel_start as usize, kernel_end as usize, multiboot_start,
        multiboot_end, memory_map_tag.memory_areas());
    use memory::frame::FrameAllocator;
    kprintln!("{:?}", frame_allocator.allocate_frame().unwrap());
    kprintln!("{:?}", frame_allocator.allocate_frame().unwrap());

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
