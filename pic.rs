use ioport::Port;

static PIC1_CMD: Port = Port::new(0x20);
static PIC1_DATA: Port = Port::new(0x21);
static PIC2_CMD: Port = Port::new(0xa0);
static PIC2_DATA: Port = Port::new(0xa1);

pub fn disable_pic() {
    PIC1_DATA.out8(0xff);
    PIC2_DATA.out8(0xff);
}
