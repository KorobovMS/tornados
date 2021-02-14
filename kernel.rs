#![no_std]
#![feature(lang_items)]
#![feature(core_panic)]
#![feature(panic_info_message)]
#![feature(fmt_as_str)]

mod idt;
mod memops;
mod panic;
mod vga;

use vga::Vga;

extern "C" {
    fn enable_interrupts();
}

#[no_mangle]
pub fn kernel_main() {
    idt::setup_idt();
    unsafe { enable_interrupts(); }
    let vga = Vga::new();
    vga.clear_screen();
}
