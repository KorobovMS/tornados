use serial;
use pic;
use vga::Vga;
use core::fmt::Write;

static mut VGA: Vga = Vga::new();

extern "C" {
    static mut _idt: u64;
}

macro_rules! interrupt_handler {
    ($isr_entry:ident $isr_name:ident $isr_body:block) => {
        #[naked]
        extern "C" fn $isr_entry() {
            unsafe {
                asm!("pusha",
                     concat!("call ", stringify!($isr_name)),
                     "popa",
                     "iretd",
                     options(noreturn)); }
        }

        #[no_mangle]
        fn $isr_name()
            $isr_body
    };
}

interrupt_handler!{isr_0 divide_error {
    core::panicking::panic("divide error");
}}

interrupt_handler!{isr_1 debug {
    core::panicking::panic("debug");
}}

interrupt_handler!{isr_2 nmi {
    core::panicking::panic("nmi");
}}

interrupt_handler!{isr_3 breakpoint {
    core::panicking::panic("breakpoint");
}}

interrupt_handler!{isr_4 overflow {
    core::panicking::panic("overflow");
}}

interrupt_handler!{isr_5 bound_range_exceeded {
    core::panicking::panic("bound range exceeded");
}}

interrupt_handler!{isr_6 invalid_opcode {
    core::panicking::panic("invalid opcode");
}}

interrupt_handler!{isr_7 device_not_available {
    core::panicking::panic("device not available");
}}

interrupt_handler!{isr_8 double_fault {
    core::panicking::panic("double fault");
}}

interrupt_handler!{isr_9 coprocessor_segment_overrun {
    core::panicking::panic("coprocessor segment overrun");
}}

interrupt_handler!{isr_10 invalid_tss {
    core::panicking::panic("invalid tss");
}}

interrupt_handler!{isr_11 segment_not_present {
    core::panicking::panic("segment not present");
}}

interrupt_handler!{isr_12 stack_fault {
    core::panicking::panic("stack fault");
}}

interrupt_handler!{isr_13 general_protection {
    core::panicking::panic("general protection");
}}

interrupt_handler!{isr_14 page_fault {
    core::panicking::panic("page fault");
}}

interrupt_handler!{isr_16 x87_fpu_floating_point_error {
    core::panicking::panic("x87 fpu floating point error");
}}

interrupt_handler!{isr_17 alignment_check {
    core::panicking::panic("alignment check");
}}

interrupt_handler!{isr_18 machine_check {
    core::panicking::panic("machine check");
}}

interrupt_handler!{isr_19 simd_floating_point {
    core::panicking::panic("simd floating point");
}}

interrupt_handler!{isr_20 virtualization {
    core::panicking::panic("virtualization");
}}

interrupt_handler!{isr_32 timer {
    serial::write_str("!");
    pic::end_of_interrupt(0);
}}

interrupt_handler!{isr_33 keyboard {
    serial::write_str("k");
    pic::end_of_interrupt(1);
}}

interrupt_handler!{isr_36 com1 {
    let b = serial::get_byte();
    unsafe { write!(&mut VGA, "{}", b as char).unwrap(); }
    serial::write_str("4");
    pic::end_of_interrupt(4);
}}

fn setup_idt_descriptor(idt: *mut u64, idx: u8, handler: *const ()) {
    let handler = handler as u64;
    let lo = handler & 0xFFFF;
    let hi = ((handler >> 16) & 0xFFFF) << 48;
    let kernel_cs = 0x8 as u16;
    let cs = (kernel_cs as u64) << 16;
    let fl = 0x8E00u64 << 32;
    let idt_desc: u64 = lo | cs | fl | hi;
    unsafe {
        *idt.offset(idx as isize) = idt_desc;
    }
}

pub fn setup_idt() {
    unsafe {
        let idt = &mut _idt as *mut u64;
        setup_idt_descriptor(idt, 0, isr_0 as *const ());
        setup_idt_descriptor(idt, 1, isr_1 as *const ());
        setup_idt_descriptor(idt, 2, isr_2 as *const ());
        setup_idt_descriptor(idt, 3, isr_3 as *const ());
        setup_idt_descriptor(idt, 4, isr_4 as *const ());
        setup_idt_descriptor(idt, 5, isr_5 as *const ());
        setup_idt_descriptor(idt, 6, isr_6 as *const ());
        setup_idt_descriptor(idt, 7, isr_7 as *const ());
        setup_idt_descriptor(idt, 8, isr_8 as *const ());
        setup_idt_descriptor(idt, 9, isr_9 as *const ());
        setup_idt_descriptor(idt, 10, isr_10 as *const ());
        setup_idt_descriptor(idt, 11, isr_11 as *const ());
        setup_idt_descriptor(idt, 12, isr_12 as *const ());
        setup_idt_descriptor(idt, 13, isr_13 as *const ());
        setup_idt_descriptor(idt, 14, isr_14 as *const ());
        // No interrupt 15
        setup_idt_descriptor(idt, 16, isr_16 as *const ());
        setup_idt_descriptor(idt, 17, isr_17 as *const ());
        setup_idt_descriptor(idt, 18, isr_18 as *const ());
        setup_idt_descriptor(idt, 19, isr_19 as *const ());
        setup_idt_descriptor(idt, 20, isr_20 as *const ());
        setup_idt_descriptor(idt, 0x20, isr_32 as *const ());
        setup_idt_descriptor(idt, 0x21, isr_33 as *const ());
        setup_idt_descriptor(idt, 0x24, isr_36 as *const ());
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

pub fn hang() -> ! {
    loop {
        halt();
    }
}
