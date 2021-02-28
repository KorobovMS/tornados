#![no_std]
#![feature(asm)]
#![feature(lang_items)]
#![feature(core_panic)]
#![feature(panic_info_message)]
#![feature(fmt_as_str)]
#![feature(naked_functions)]

mod idt;
mod memops;
mod panic;
mod vga;
mod ioport;
mod serial;
mod pic;
mod entry;
