// some code was from http://os.phil-opp.com/allocating-frames.html
#![no_std]
#![feature(const_fn)]

#[macro_use]
extern crate bitflags;
extern crate multiboot2;

pub mod frame;
pub mod bump;
pub mod paging;

pub use bump::BumpAllocator;
pub use frame::Frame;

pub type PhysicalAddress = usize;
pub type VirtualAddress  = usize;
