// some code was from http://os.phil-opp.com/allocating-frames.html
#![no_std]

extern crate multiboot2;

pub mod frame;
pub mod bump;

pub use bump::BumpAllocator;
