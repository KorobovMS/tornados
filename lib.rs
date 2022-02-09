#![no_std]
#![feature(core_panic)]
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
mod sched;
mod vga;
