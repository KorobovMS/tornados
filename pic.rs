use ioport::Port;

static PIC1_COMMAND: Port = Port { port: 0x20 };
static PIC1_DATA: Port = Port { port: 0x21 };
static PIC2_COMMAND: Port = Port { port: 0xa0 };
static PIC2_DATA: Port = Port { port: 0xa1 };

pub fn disable_pic() {
    PIC1_DATA.out8(0xff);
    PIC2_DATA.out8(0xff);
}
