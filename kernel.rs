#![no_std]
#![feature(lang_items)]
#![feature(core_panic)]
#![feature(panic_info_message)]
#![feature(fmt_as_str)]

mod memops;
mod panic;
mod vga;

use vga::Vga;

#[no_mangle]
pub fn kernel_main() {
    let vga = Vga::new();
    vga.clear_screen();
    core::panicking::panic("PANIC!!!!!");
}
