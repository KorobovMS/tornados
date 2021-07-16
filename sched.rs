use core::option::Option;
use core::marker::Copy;
use core::clone::Clone;

#[repr(C)]
pub struct InterruptState {
    ebp: u32,
    edi: u32,
    esi: u32,
    edx: u32,
    ecx: u32,
    ebx: u32,
    eax: u32,
    eip: u32,
    cs: u32,
    eflags: u32,
    esp: u32,
    ss: u32,
}

#[derive(Copy, Clone)]
struct Thread {
    eax: u32,
    ebx: u32,
    ecx: u32,
    edx: u32,
    esi: u32,
    edi: u32,
    ebp: u32,
    esp: u32,
    eip: u32,
    eflags: u32,
    cs: u32,
    ss: u32,
}

unsafe fn save_interrupt_state(int_state: *const InterruptState, thread: &mut Thread) {
    thread.eax = (*int_state).eax;
    thread.ebx = (*int_state).ebx;
    thread.ecx = (*int_state).ecx;
    thread.edx = (*int_state).edx;
    thread.esi = (*int_state).esi;
    thread.edi = (*int_state).edi;
    thread.ebp = (*int_state).ebp;
    thread.eip = (*int_state).eip;
    thread.eflags = (*int_state).eflags;
    thread.cs = (*int_state).cs;
    if thread.cs & 0b11 == 0b11 {
        // interrupted user-mode
        thread.esp = (*int_state).esp;
        thread.ss = (*int_state).ss;
    } else {
        // interrupted kernel-mode
        thread.esp = (int_state as *const u32).offset(10) as u32;
        thread.ss = KERNEL_DS
    }
}

const MAX_THREADS: usize = 5;
const STACK_SIZE: usize = 16*1024;
static mut THREADS: [Option<Thread>; MAX_THREADS] = [None; MAX_THREADS];
static mut STACKS: [[u8; STACK_SIZE]; MAX_THREADS] = [[0; STACK_SIZE]; MAX_THREADS];
static mut CURRENT_THREAD_IDX: usize = 0;
static mut CURRENT_THREAD_COUNT: usize = 0;

const X86_EFLAGS_BASE: u32 = 0b10;
const X86_EFLAGS_CF: u32 = 1 << 0;
const X86_EFLAGS_PF: u32 = 1 << 2;
const X86_EFLAGS_AF: u32 = 1 << 4;
const X86_EFLAGS_ZF: u32 = 1 << 6;
const X86_EFLAGS_SF: u32 = 1 << 7;
const X86_EFLAGS_TF: u32 = 1 << 8;
const X86_EFLAGS_IF: u32 = 1 << 9;
const X86_EFLAGS_DF: u32 = 1 << 10;
const X86_EFLAGS_OF: u32 = 1 << 11;
const X86_EFLAGS_IOPL0: u32 = 0 << 12;
const X86_EFLAGS_IOPL1: u32 = 1 << 12;
const X86_EFLAGS_IOPL2: u32 = 2 << 12;
const X86_EFLAGS_IOPL3: u32 = 3 << 12;
const X86_EFLAGS_NT: u32 = 1 << 14;
const X86_EFLAGS_RF: u32 = 1 << 16;
const X86_EFLAGS_VM: u32 = 1 << 17;
const X86_EFLAGS_AC: u32 = 1 << 18;
const X86_EFLAGS_VIF: u32 = 1 << 19;
const X86_EFLAGS_VIP: u32 = 1 << 20;
const X86_EFLAGS_ID: u32 = 1 << 21;

const KERNEL_CS: u32 = 0x8;
const KERNEL_DS: u32 = 0x10;
const USER_CS: u32 = 0x1B;
const USER_DS: u32 = 0x23;

pub fn create_kernel_thread(entry: *const ()) {
    unsafe {
        if CURRENT_THREAD_COUNT == MAX_THREADS {
            panic!("No more space for threads");
        }
        let stack = &STACKS[CURRENT_THREAD_COUNT] as *const u8;
        let thread = Thread {
            eax: 0,
            ebx: 0,
            ecx: 0,
            edx: 0,
            esi: 0,
            edi: 0,
            ebp: 0,
            esp: stack.offset(STACK_SIZE as isize) as u32,
            eip: entry as u32,
            eflags: X86_EFLAGS_BASE | X86_EFLAGS_IF,
            cs: KERNEL_CS,
            ss: KERNEL_DS,
        };
        THREADS[CURRENT_THREAD_COUNT] = Some(thread);
        CURRENT_THREAD_COUNT += 1;
    }
}

pub fn create_user_thread(entry: *const ()) {
    unsafe {
        if CURRENT_THREAD_COUNT == MAX_THREADS {
            panic!("No more space for threads");
        }
        let stack = &STACKS[CURRENT_THREAD_COUNT] as *const u8;
        let thread = Thread {
            eax: 0,
            ebx: 0,
            ecx: 0,
            edx: 0,
            esi: 0,
            edi: 0,
            ebp: 0,
            esp: stack.offset(STACK_SIZE as isize) as u32,
            eip: entry as u32,
            eflags: X86_EFLAGS_BASE | X86_EFLAGS_IF,
            cs: USER_CS,
            ss: USER_DS,
        };
        THREADS[CURRENT_THREAD_COUNT] = Some(thread);
        CURRENT_THREAD_COUNT += 1;
    }
}

unsafe fn next_idx(current_idx: usize) -> (usize, &'static Thread) {
    let mut idx = current_idx;
    loop {
        idx = (idx + 1) % MAX_THREADS;
        if let Some(thread) = &THREADS[idx] {
            return (idx, thread);
        }
    }
}

#[no_mangle]
pub extern "C" fn save_current_state(int_state: *mut InterruptState) {
    unsafe {
        if let Some(ref mut thread) = THREADS[CURRENT_THREAD_IDX] {
            save_interrupt_state(int_state, thread);
            for _ in 0..10 {}
        }
    }
}

extern "C" {
    fn restore_thread(eax: u32, ebx: u32, ecx: u32, edx: u32,
                      esi: u32, edi: u32, ebp: u32, esp: u32,
                      eip: u32, eflags: u32, cs: u32, ss: u32) -> !;
}

fn switch_to_thread(t: &Thread) -> ! {
    unsafe {
        restore_thread(t.eax, t.ebx, t.ecx, t.edx,
                       t.esi, t.edi, t.ebp, t.esp,
                       t.eip, t.eflags, t.cs, t.ss);
    }
}

#[no_mangle]
pub fn invoke_scheduler() -> ! {
    unsafe {
        let (idx, thread) = next_idx(CURRENT_THREAD_IDX);
        CURRENT_THREAD_IDX = idx;
        switch_to_thread(thread);
    }
}

pub fn start_scheduler() -> ! {
    unsafe {
        if let Some(ref thread) = THREADS[0] {
            switch_to_thread(thread);
        } else {
            panic!("No threads to run");
        }
    }
}
