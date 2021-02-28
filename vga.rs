pub struct Vga {
    vga: *mut u16,
    row: u8,
    col: u8,
    width: u8,
    height: u8,
}

impl Vga {
    pub const fn new() -> Vga {
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

    pub fn clear_screen(&self) {
        for r in 0..self.height {
            for c in 0..self.width {
                self.set_char(r, c, b' ', 0, 0);
            }
        }
    }

    pub fn write(&mut self, s: &str) {
        for b in s.bytes() {
            if b == b'\n' {
                self.row = self.row + 1;
                self.col = 0;
            } else {
                self.set_char(self.row, self.col, b, 0x0E, 0x01);
                self.col = self.col + 1;
            }
            if self.col == self.width {
                self.col = 0;
                self.row = self.row + 1;
            }
            if self.row == self.height {
                self.row = 0;
            }
        }
    }
}

impl core::fmt::Write for Vga {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        self.write(s);
        Ok(())
    }
}

