use gdt;
use idt;
use pic;
use serial;
use vga::Vga;

#[no_mangle]
pub fn kernel_main() {
    gdt::setup_gdt();
    idt::setup_idt();
    pic::remap(0x20, 0x28);
    pic::mask(0xEC, 0xFF);
    idt::enable_interrupts();
    serial::serial_init();
    serial::write_str("Booting kernel...\n");
    let vga = Vga::new();
    vga.clear_screen();
    idt::hang();
}
