#![feature(lang_items)]
#![feature(no_std)]
#![feature(asm)]
#![no_std]

extern crate volatile;

pub mod cpu;
pub mod boards;
pub mod kernel;

pub fn start() -> ! {
    loop { }
}
