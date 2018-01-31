#![no_std]
#![feature(asm)]
#![feature(const_fn)]

pub mod io;
pub mod pic;
pub mod keyboard;

#[macro_use]
extern crate bitflags;
extern crate spin;
