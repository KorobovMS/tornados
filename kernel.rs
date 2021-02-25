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
mod ioport;
mod serial;
mod pic;

use vga::Vga;

#[no_mangle]
pub fn kernel_main() {
    idt::setup_idt();
    pic::remap(0x20, 0x28);
    serial::serial_init();
    serial::write_str("Booting kernel...\n");
    idt::enable_interrupts();
    let vga = Vga::new();
    vga.clear_screen();
    idt::hang();
}
