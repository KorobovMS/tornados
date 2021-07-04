use ioport::Port;

static PIC1_CMD: Port = Port::new(0x20);
static PIC1_DATA: Port = Port::new(0x21);
static PIC2_CMD: Port = Port::new(0xa0);
static PIC2_DATA: Port = Port::new(0xa1);

const PIC_EOI: u8 = 0x20;

const ICW1_ICW4: u8 = 0x01;
const ICW1_SINGLE: u8 = 0x02;
const ICW1_INTERVAL4: u8 = 0x04;
const ICW1_LEVEL: u8 = 0x08;
const ICW1_INIT: u8 = 0x10;

const ICW4_8086: u8 = 0x01;
const ICW4_AUTO: u8 = 0x02;
const ICW4_BUF_SLAVE: u8 = 0x08;
const ICW4_BUF_MASTER: u8 = 0x0C;
const ICW4_SFNM: u8 = 0x10;

pub fn mask(mask1: u8, mask2: u8) {
    PIC1_DATA.out8(mask1);
    PIC2_DATA.out8(mask2);
}

pub fn end_of_interrupt(irq: u8) {
    if irq >= 8 {
        PIC2_CMD.out8(PIC_EOI);
    }
    PIC1_CMD.out8(PIC_EOI);
}

#[no_mangle]
pub extern "C" fn end_of_timer_interrupt() {
    end_of_interrupt(0);
}

pub fn remap(off1: u8, off2: u8) {
    let mask1 = PIC1_DATA.in8();
    let mask2 = PIC2_DATA.in8();
    PIC1_CMD.out8(ICW1_INIT | ICW1_ICW4);
    PIC2_CMD.out8(ICW1_INIT | ICW1_ICW4);
    PIC1_DATA.out8(off1);
    PIC2_DATA.out8(off2);
    PIC1_DATA.out8(4);
    PIC2_DATA.out8(2);
    PIC1_DATA.out8(ICW4_8086);
    PIC2_DATA.out8(ICW4_8086);
    PIC1_DATA.out8(mask1);
    PIC2_DATA.out8(mask2);
}
