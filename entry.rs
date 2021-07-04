use gdt;
use idt;
use pic;
use serial;
use vga::Vga;
use sched::create_kernel_thread;

fn thread1_proc()
{
    loop {
        idt::disable_interrupts();
        serial::write_str("1");
        idt::enable_interrupts();
        for _ in 0..1000 {}
    }
}

fn thread2_proc()
{
    loop {
        idt::disable_interrupts();
        serial::write_str("2");
        idt::enable_interrupts();
        for _ in 0..1000 {}
    }
}

fn thread3_proc()
{
    loop {
        idt::disable_interrupts();
        serial::write_str("3");
        idt::enable_interrupts();
        for _ in 0..1000 {}
    }
}

#[no_mangle]
pub fn kernel_main() {
    gdt::setup_gdt();
    idt::setup_idt();
    pic::remap(0x20, 0x28);
    pic::mask(0xEC, 0xFF);
    serial::serial_init();
    serial::write_str("Booting kernel...\n");
    let vga = Vga::new();
    vga.clear_screen();
    create_kernel_thread(thread1_proc as *const ());
    create_kernel_thread(thread2_proc as *const ());
    create_kernel_thread(thread3_proc as *const ());
    idt::enable_interrupts();
    idt::hang();
}
