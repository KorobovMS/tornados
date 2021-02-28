use serial;
use pic;

extern "C" {
    static mut _idt: u64;
}

fn divide_error() {
    core::panicking::panic("divide error");
}

fn debug() {
    core::panicking::panic("debug");
}

fn nmi() {
    core::panicking::panic("nmi");
}

fn breakpoint() {
    core::panicking::panic("breakpoint");
}

fn overflow() {
    core::panicking::panic("overflow");
}

fn bound_range_exceeded() {
    core::panicking::panic("bound range exceeded");
}

fn invalid_opcode() {
    core::panicking::panic("invalid opcode");
}

fn device_not_available() {
    core::panicking::panic("device not available");
}

fn double_fault() {
    core::panicking::panic("double fault");
}

fn coprocessor_segment_overrun() {
    core::panicking::panic("coprocessor segment overrun");
}

fn invalid_tss() {
    core::panicking::panic("invalid tss");
}

fn segment_not_present() {
    core::panicking::panic("segment not present");
}

fn stack_fault() {
    core::panicking::panic("stack fault");
}

fn general_protection() {
    core::panicking::panic("general protection");
}

fn page_fault() {
    core::panicking::panic("page fault");
}

fn x87_fpu_floating_point_error() {
    core::panicking::panic("x87 fpu floating point error");
}

fn alignment_check() {
    core::panicking::panic("alignment check");
}

fn machine_check() {
    core::panicking::panic("machine check");
}

fn simd_floating_point() {
    core::panicking::panic("simd floating point");
}

fn virtualization() {
    core::panicking::panic("virtualization");
}

#[naked]
extern "C" fn timer() {
    unsafe {
        asm!("pusha",
             "call _timer",
             "popa",
             "iretd",
             options(noreturn)); }
}

#[no_mangle]
fn _timer() {
    serial::write_str(".");
    pic::end_of_interrupt(0);
}

#[naked]
extern "C" fn keyboard() {
    unsafe {
        asm!("pusha",
             "call _keyboard",
             "popa",
             "iretd",
             options(noreturn)); }
}

#[no_mangle]
fn _keyboard() {
    serial::write_str("k");
    pic::end_of_interrupt(1);
}

fn setup_idt_descriptor(idt: *mut u64, idx: u8, handler: *const ()) {
    let handler = handler as u64;
    let lo = handler & 0xFFFF;
    let hi = ((handler >> 16) & 0xFFFF) << 48;
    let cs = (get_kernel_cs() as u64) << 16;
    let fl = 0x8E00u64 << 32;
    let idt_desc: u64 = lo | cs | fl | hi;
    unsafe {
        *idt.offset(idx as isize) = idt_desc;
    }
}

pub fn setup_idt() {
    unsafe {
        let idt = &mut _idt as *mut u64;
        setup_idt_descriptor(idt, 0, divide_error as *const ());
        setup_idt_descriptor(idt, 1, debug as *const ());
        setup_idt_descriptor(idt, 2, nmi as *const ());
        setup_idt_descriptor(idt, 3, breakpoint as *const ());
        setup_idt_descriptor(idt, 4, overflow as *const ());
        setup_idt_descriptor(idt, 5, bound_range_exceeded as *const ());
        setup_idt_descriptor(idt, 6, invalid_opcode as *const ());
        setup_idt_descriptor(idt, 7, device_not_available as *const ());
        setup_idt_descriptor(idt, 8, double_fault as *const ());
        setup_idt_descriptor(idt, 9, coprocessor_segment_overrun as *const ());
        setup_idt_descriptor(idt, 10, invalid_tss as *const ());
        setup_idt_descriptor(idt, 11, segment_not_present as *const ());
        setup_idt_descriptor(idt, 12, stack_fault as *const ());
        setup_idt_descriptor(idt, 13, general_protection as *const ());
        setup_idt_descriptor(idt, 14, page_fault as *const ());
        // No interrupt 15
        setup_idt_descriptor(idt, 16, x87_fpu_floating_point_error as *const ());
        setup_idt_descriptor(idt, 17, alignment_check as *const ());
        setup_idt_descriptor(idt, 18, machine_check as *const ());
        setup_idt_descriptor(idt, 19, simd_floating_point as *const ());
        setup_idt_descriptor(idt, 20, virtualization as *const ());
        setup_idt_descriptor(idt, 0x20, timer as *const ());
        setup_idt_descriptor(idt, 0x21, keyboard as *const ());
    }
}

pub fn enable_interrupts() {
    unsafe { asm!("sti"); }
}

pub fn disable_interrupts() {
    unsafe { asm!("cli"); }
}

fn halt() {
    unsafe { asm!("hlt"); }
}

fn get_kernel_cs() -> u16 {
    let val: u16;
    unsafe { asm!("mov {0:x}, cs", out(reg) val); }
    val
}

pub fn hang() -> ! {
    loop {
        halt();
    }
}
