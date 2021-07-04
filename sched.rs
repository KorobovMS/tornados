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

pub fn create_kernel_thread(entry: *const ()) {
    unsafe {
        if CURRENT_THREAD_COUNT == MAX_THREADS {
            panic!("No more space for threads");
        }
        let mut thread = Thread::new();
        let stack = &STACKS[CURRENT_THREAD_COUNT] as *const u8;
        let stack = stack as *mut u32;
        let size = (STACK_SIZE / 4) as isize;
        *stack.offset(size - 1) = 0x6; // EFLAGS
        *stack.offset(size - 2) = 0x8; // CS
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
       return next.esp;
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

