pub struct Port {
    port: u16,
}

impl Port {
    pub const fn new(port: u16) -> Port {
        Port {
            port
        }
    }

    pub fn out8(&self, val: u8) {
        unsafe {
            asm!("out dx, al", in("dx") self.port, in("al") val);
        }
    }

    pub fn out16(&self, val: u16) {
        unsafe {
            asm!("out dx, ax", in("dx") self.port, in("ax") val);
        }
    }

    pub fn out32(&self, val: u32) {
        unsafe {
            asm!("out dx, eax", in("dx") self.port, in("eax") val);
        }
    }

    pub fn in8(&self) -> u8 {
        let val: u8;
        unsafe {
            asm!("in al, dx", out("al") val, in("dx") self.port);
        }
        val
    }
    
    pub fn in16(&self) -> u16 {
        let val: u16;
        unsafe {
            asm!("in ax, dx", out("ax") val, in("dx") self.port);
        }
        val
    }

    pub fn in32(&self) -> u32 {
        let val: u32;
        unsafe {
            asm!("in eax, dx", out("eax") val, in("dx") self.port);
        }
        val
    }
}
