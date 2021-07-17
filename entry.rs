use gdt;
use idt;
use pic;
use serial;
use vga::Vga;
use sched;

fn busy_wait() {
    let mut i = 0;
    while i < 1000000 {
        i += 1;
    }
}

fn thread1_proc()
{
    loop {
        serial::write_str("1");
        busy_wait();
    }
}

fn thread2_proc()
{
    loop {
        serial::write_str("2");
        busy_wait();
    }
}


fn thread3_proc()
{
    loop {
        serial::write_str("3");
        busy_wait();
    }
}

fn thread4_proc()
{
    let mut x: u8 = 0;
    loop {
        x = (x + 1) % 3;
        if x == 0 {
            sched::resume_thread(4);
        } else {
            sched::suspend_thread(4);
        }

        serial::write_str("4");
        busy_wait();
    }
}

fn thread5_proc()
{
    let mut vga = Vga::new();
    loop {
        vga.write("123");
        busy_wait();
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
    sched::create_kernel_thread(thread1_proc as *const ());
    sched::create_kernel_thread(thread2_proc as *const ());
    sched::create_kernel_thread(thread3_proc as *const ());
    sched::create_kernel_thread(thread4_proc as *const ());
    sched::create_user_thread(thread5_proc as *const ());
    sched::start_scheduler();
}
