#![no_std]
#![feature(const_fn)]
#![feature(ptr_internals)]
#![feature(alloc)]
#![feature(allocator_api)]
#![feature(global_allocator)]

extern crate alloc;
#[macro_use]
extern crate bitflags;
#[macro_use]
extern crate vga;
extern crate x86_64;
extern crate multiboot2;

use multiboot2::BootInformation;

mod area_frame_allocator;
mod paging;
pub mod frame;
pub mod heap_allocator;
mod stack_allocator;

pub use area_frame_allocator::AreaFrameAllocator;
pub use paging::remap_the_kernel;
pub use stack_allocator::Stack;
pub use frame::{FrameAllocator, Frame, FrameIter};
use paging::PhysicalAddress;

pub const PAGE_SIZE: usize = 4096;

pub fn init(boot_info: &BootInformation, heap_start: usize, heap_size: usize) -> MemoryController {
    //assert_has_not_been_called!("memory::init must be called only once");

    let memory_map_tag = boot_info.memory_map_tag().expect("Memory map tag required");
    let elf_sections_tag = boot_info
        .elf_sections_tag()
        .expect("Elf sections tag required");

    let kernel_start = elf_sections_tag
        .sections()
        .filter(|s| s.is_allocated())
        .map(|s| s.addr)
        .min()
        .unwrap();
    let kernel_end = elf_sections_tag
        .sections()
        .filter(|s| s.is_allocated())
        .map(|s| s.addr + s.size)
        .max()
        .unwrap();

    kprintln!(
        "kernel start: {:#x}, kernel end: {:#x}",
        kernel_start,
        kernel_end
    );
    kprintln!(
        "multiboot start: {:#x}, multiboot end: {:#x}",
        boot_info.start_address(),
        boot_info.end_address()
    );

    let mut frame_allocator = AreaFrameAllocator::new(
        kernel_start as usize,
        kernel_end as usize,
        boot_info.start_address(),
        boot_info.end_address(),
        memory_map_tag.memory_areas(),
    );

    let mut active_table = paging::remap_the_kernel(&mut frame_allocator, boot_info);

    use self::paging::Page;

    let heap_start_page = Page::containing_address(heap_start);
    let heap_end_page = Page::containing_address(heap_start + heap_size - 1);

    for page in Page::range_inclusive(heap_start_page, heap_end_page) {
        active_table.map(page, paging::WRITABLE, &mut frame_allocator);
    }

    let stack_allocator = {
        let stack_alloc_start = heap_end_page + 1;
        let stack_alloc_end = stack_alloc_start + 100;
        let stack_alloc_range = Page::range_inclusive(stack_alloc_start, stack_alloc_end);
        stack_allocator::StackAllocator::new(stack_alloc_range)
    };

    MemoryController {
        active_table: active_table,
        frame_allocator: frame_allocator,
        stack_allocator: stack_allocator,
    }
}

pub struct MemoryController {
    active_table: paging::ActivePageTable,
    frame_allocator: AreaFrameAllocator,
    stack_allocator: stack_allocator::StackAllocator,
}

impl MemoryController {
    pub fn alloc_stack(&mut self, size_in_pages: usize) -> Option<Stack> {
        let &mut MemoryController {
            ref mut active_table,
            ref mut frame_allocator,
            ref mut stack_allocator,
        } = self;
        stack_allocator.alloc_stack(active_table, frame_allocator, size_in_pages)
    }
}

