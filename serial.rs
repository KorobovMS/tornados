use ioport::Port;

const SERIAL_BASE: u16 = 0x3f8;

static SERIAL_DR: Port = Port::new(SERIAL_BASE + 0);
static SERIAL_DLAB_DIV_LSB: Port = Port::new(SERIAL_BASE + 0);
static SERIAL_IER: Port = Port::new(SERIAL_BASE + 1);
static SERIAL_DLAB_DIV_MSB: Port = Port::new(SERIAL_BASE + 1);
static SERIAL_II: Port = Port::new(SERIAL_BASE + 2);
static SERIAL_LCR: Port = Port::new(SERIAL_BASE + 3);
static SERIAL_MCR: Port = Port::new(SERIAL_BASE + 4);
static SERIAL_LSR: Port = Port::new(SERIAL_BASE + 5);
static SERIAL_MSR: Port = Port::new(SERIAL_BASE + 6);
static SERIAL_SR: Port = Port::new(SERIAL_BASE + 7);

pub fn serial_init() {
    SERIAL_IER.out8(0x00);
    SERIAL_LCR.out8(0x80);
    SERIAL_DLAB_DIV_LSB.out8(0x01);
    SERIAL_DLAB_DIV_MSB.out8(0x00);
    SERIAL_LCR.out8(0x03);
    SERIAL_II.out8(0xC7);
    SERIAL_MCR.out8(0x0B);
    SERIAL_MCR.out8(0x1E);
    SERIAL_DR.out8(0xAE);
    if SERIAL_DR.in8() != 0xAE {
        panic!("cannot initialize serial");
    }
    SERIAL_MCR.out8(0x0F);
}

pub fn write_str(s: &str) {
    for b in s.bytes() {
        while (SERIAL_LSR.in8() & 0x20) == 0 {}
        SERIAL_DR.out8(b);
    }
}
