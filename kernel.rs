#![no_std]
#![feature(asm)]
#![feature(lang_items)]
#![feature(core_panic)]
#![feature(panic_info_message)]
#![feature(fmt_as_str)]

mod idt;
mod memops;
mod panic;
mod vga;

use vga::Vga;

#[no_mangle]
pub fn kernel_main() {
    idt::setup_idt();
    idt::enable_interrupts();
    let vga = Vga::new();
    vga.clear_screen();
    idt::hang();
}
