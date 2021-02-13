#![no_std]
#![feature(lang_items)]
#![feature(core_panic)]
#![feature(panic_info_message)]
#![feature(fmt_as_str)]

mod memops;
mod vga;

use vga::Vga;

extern "C" {
    fn hang() -> !;
}

#[no_mangle]
pub fn kernel_main() {
    let vga = Vga::new();
    vga.clear_screen();
    core::panicking::panic("PANIC!!!!!");
}

#[panic_handler]
fn panic_impl(_info: &core::panic::PanicInfo) -> ! {
    let mut vga = Vga::new();
    if let Some(arg) = _info.message() {
        if let Some(msg) = arg.as_str() {
            vga.write(msg);
        } else {
            vga.write("unknown msg");
        }
    } else {
        vga.write("unknown arg");
    }
    unsafe {
        hang();
    }
}

#[lang = "eh_personality"]
fn eh_personality() {}
