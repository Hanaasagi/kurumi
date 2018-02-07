#![no_std]
#![feature(asm)]
#![feature(const_fn)]

pub mod io;
pub mod pic;
pub mod keyboard;
pub mod tty;
pub mod disk;
pub mod ata;

#[macro_use]
extern crate vga;

#[macro_use]
extern crate bitflags;
extern crate spin;
