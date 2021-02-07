#![no_std]
#![feature(lang_items)]

extern "C" {
    fn hang() -> !;
}

fn vga_entry(ch: u8, fg: u8, bg: u8) -> u16 {
    let fg = fg as u16;
    let bg = bg as u16;
    let ch = ch as u16;
    ch | (fg | (bg << 4)) << 8
}

#[no_mangle]
pub fn kernel_main() {
    unsafe {
        let vga: *mut u16 = 0xB8000 as *mut u16;
        *vga = vga_entry(0x4B, 1, 2);
    }
}

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    unsafe {
        let vga: *mut u16 = 0xB8000 as *mut u16;
        *vga = 0x4321;
        hang();
    }
}

#[lang = "eh_personality"]
fn eh_personality()
{}
