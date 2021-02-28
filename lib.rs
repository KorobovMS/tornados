#![no_std]
#![feature(asm)]
#![feature(core_panic)]
#![feature(fmt_as_str)]
#![feature(lang_items)]
#![feature(naked_functions)]
#![feature(panic_info_message)]

mod entry;
mod idt;
mod ioport;
mod memops;
mod panic;
mod pic;
mod serial;
mod vga;
