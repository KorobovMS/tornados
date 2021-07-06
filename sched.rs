use core::option::Option;
use core::marker::Copy;
use core::clone::Clone;

#[derive(Copy, Clone)]
struct Thread {
    esp: u32,
}

impl Thread {
    fn new() -> Self {
        Self {
            esp: 0,
        }
    }
}

const MAX_THREADS: usize = 5;
const STACK_SIZE: usize = 16*1024;
static mut THREADS: [Option<Thread>; MAX_THREADS] = [None; MAX_THREADS];
static mut STACKS: [[u8; STACK_SIZE]; MAX_THREADS] = [[0; STACK_SIZE]; MAX_THREADS];
static mut CURRENT_THREAD_IDX: usize = MAX_THREADS - 1;
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

pub fn create_kernel_thread(entry: *const ()) {
    unsafe {
        if CURRENT_THREAD_COUNT == MAX_THREADS {
            panic!("No more space for threads");
        }
        let mut thread = Thread::new();
        let stack = &STACKS[CURRENT_THREAD_COUNT] as *const u8;
        let stack = stack as *mut u32;
        let size = (STACK_SIZE / 4) as isize;
        *stack.offset(size - 1) = X86_EFLAGS_BASE | X86_EFLAGS_IF; // EFLAGS
        *stack.offset(size - 2) = 0x8; // KERNEL_CS
        *stack.offset(size - 3) = entry as u32; // EIP

        thread.esp = stack.offset(size - 10) as u32;
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
pub extern "C" fn get_next_stack() -> u32 {
   unsafe {
       let (new_idx, next) = next_idx(CURRENT_THREAD_IDX);
       CURRENT_THREAD_IDX = new_idx;
       next.esp
   }
}

#[no_mangle]
pub extern "C" fn save_stack_ptr(esp: u32) {
    unsafe {
        if let Some(thread) = &mut THREADS[CURRENT_THREAD_IDX] {
            thread.esp = esp;
        }
    }
}

