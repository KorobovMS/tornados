#![no_std]
#![feature(lang_items)]
#![feature(core_panic)]
#![feature(panic_info_message)]
#![feature(fmt_as_str)]

mod memops;

extern "C" {
    fn hang() -> !;
}

struct Vga {
    vga: *mut u16,
    row: u8,
    col: u8,
    width: u8,
    height: u8,
}

impl Vga {
    fn new() -> Vga {
        Vga {
            vga: 0xB8000 as *mut u16,
            row: 0u8,
            col: 0u8,
            width: 80,
            height: 25,
        }
    }

    fn set_char(&self, row: u8, col: u8, ch: u8, fg: u8, bg: u8) {
        let fg = fg as u16;
        let bg = bg as u16;
        let ch = ch as u16;
        let row = row as isize;
        let col = col as isize;
        let width = self.width as isize;
        let entry = ch | (fg | (bg << 4)) << 8;
        unsafe {
            core::ptr::write_volatile(self.vga.offset(width * row + col), entry);
        }
    }

    fn clear_screen(&self) {
        for r in 0..self.height {
            for c in 0..self.width {
                self.set_char(r, c, b' ', 0, 0);
            }
        }
    }

    fn write(&mut self, s: &str) {
        for b in s.bytes() {
            if b == b'\n' {
                self.row = self.row + 1;
                self.col = 0;
            } else {
                self.set_char(self.row, self.col, b, 0x0E, 0x01);
                self.col = self.col + 1;
            }
        }
    }
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
