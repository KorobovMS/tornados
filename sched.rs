use core::arch::asm;
use core::marker::Copy;
use core::clone::Clone;
use core::fmt::{Display, Formatter, Result};
use core::ptr::addr_of;

#[derive(Copy, Clone)]
enum ThreadState {
    Running,
    Waiting,
    Stopped,
}

#[derive(Copy, Clone)]
pub struct Thread {
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

    state: ThreadState,
}

impl Display for Thread {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "\
            eax 0x{:08X} ebx 0x{:08X} ecx 0x{:08X} edx 0x{:08X}\n\
            esi 0x{:08X} edi 0x{:08X} ebp 0x{:08X} esp 0x{:08X}\n\
            eip 0x{:08X} efl 0x{:08X} cs  0x{:08X} ss  0x{:08X}\n",
            self.eax, self.ebx, self.ecx, self.edx,
            self.esi, self.edi, self.ebp, self.esp,
            self.eip, self.eflags, self.cs, self.ss)
    }
}

#[naked]
extern "C" fn idle_proc() -> ! {
    unsafe {
        asm!(
            "2:",
            "hlt",
            "jmp 2b",
            options(noreturn));
    }
}

const MAX_THREADS: usize = 5;
const STACK_SIZE: usize = 16*1024;
static mut THREADS: [Option<Thread>; MAX_THREADS] = [None; MAX_THREADS];
static mut STACKS: [[u8; STACK_SIZE]; MAX_THREADS] = [[0; STACK_SIZE]; MAX_THREADS];
static mut CURRENT_THREAD_IDX: usize = 0;
static mut CURRENT_THREAD_COUNT: usize = 0;
static mut IDLE_THREAD: Thread = Thread {
    eax: 0,
    ebx: 0,
    ecx: 0,
    edx: 0,
    esi: 0,
    edi: 0,
    ebp: 0,
    esp: 0,
    eip: 0,
    eflags: X86_EFLAGS_BASE | X86_EFLAGS_IF,
    cs: KERNEL_CS,
    ss: KERNEL_DS,

    state: ThreadState::Running,
};
const IDLE_STACK_SIZE: usize = 4*1024;
static mut IDLE_STACK: [u8; IDLE_STACK_SIZE] = [0; IDLE_STACK_SIZE];

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
            esp: stack.add(STACK_SIZE) as u32,
            eip: entry as u32,
            eflags: X86_EFLAGS_BASE | X86_EFLAGS_IF,
            cs: KERNEL_CS,
            ss: KERNEL_DS,

            state: ThreadState::Running,
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
            esp: stack.add(STACK_SIZE) as u32,
            eip: entry as u32,
            eflags: X86_EFLAGS_BASE | X86_EFLAGS_IF,
            cs: USER_CS,
            ss: USER_DS,

            state: ThreadState::Running,
        };
        THREADS[CURRENT_THREAD_COUNT] = Some(thread);
        CURRENT_THREAD_COUNT += 1;
    }
}

fn set_thread_state(idx: usize, state: ThreadState) {
    unsafe {
        if idx < CURRENT_THREAD_COUNT {
            if let Some(ref mut thread) = THREADS[idx] {
                match thread.state {
                    ThreadState::Stopped => {},
                    ThreadState::Waiting |
                    ThreadState::Running => thread.state = state,
                }
            }
        }
    }
}

pub fn stop_thread(i: usize) {
    set_thread_state(i, ThreadState::Stopped);
}

pub fn suspend_thread(i: usize) {
    set_thread_state(i, ThreadState::Waiting);
}

pub fn resume_thread(i: usize) {
    set_thread_state(i, ThreadState::Running);
}

pub fn current() -> &'static Thread {
    unsafe { THREADS[CURRENT_THREAD_IDX].as_ref().unwrap() }
}

unsafe fn next_idx(current_idx: usize) -> (usize, *const Thread) {
    let mut idx = current_idx;
    loop {
        idx = (idx + 1) % MAX_THREADS;
        if let Some(thread) = &THREADS[idx] {
            if let ThreadState::Running = thread.state {
                return (idx, thread);
            }
        }
        if idx == current_idx {
            return (MAX_THREADS, addr_of!(IDLE_THREAD));
        }
    }
}

pub fn save_current_state(int_state: *const u32) {
    unsafe {
        if CURRENT_THREAD_IDX == MAX_THREADS {
            return;
        }
        if let Some(ref mut thread) = THREADS[CURRENT_THREAD_IDX] {
            thread.ebp = *int_state.offset(0);
            thread.edi = *int_state.offset(1);
            thread.esi = *int_state.offset(2);
            thread.edx = *int_state.offset(3);
            thread.ecx = *int_state.offset(4);
            thread.ebx = *int_state.offset(5);
            thread.eax = *int_state.offset(6);
            thread.eip = *int_state.offset(9);
            thread.cs = *int_state.offset(10);
            thread.eflags = *int_state.offset(11);
            if thread.cs & 0b11 == 0b11 {
                // interrupted user-mode
                thread.esp = *int_state.offset(12);
                thread.ss = *int_state.offset(13);
            } else {
                // interrupted kernel-mode
                thread.esp = int_state.offset(12) as u32;
                thread.ss = KERNEL_DS;
            }
        }
    }
}

extern "C" {
    fn restore_thread(eax: u32, ebx: u32, ecx: u32, edx: u32,
                      esi: u32, edi: u32, ebp: u32, esp: u32,
                      eip: u32, eflags: u32, cs: u32, ss: u32) -> !;
}

fn switch_to_thread(t: *const Thread) -> ! {
    unsafe {
        restore_thread((*t).eax, (*t).ebx, (*t).ecx, (*t).edx,
                       (*t).esi, (*t).edi, (*t).ebp, (*t).esp,
                       (*t).eip, (*t).eflags, (*t).cs, (*t).ss);
    }
}

pub fn invoke_scheduler() -> ! {
    unsafe {
        let start_idx = if CURRENT_THREAD_IDX == MAX_THREADS { 0 } else { CURRENT_THREAD_IDX };
        let (idx, thread) = next_idx(start_idx);
        CURRENT_THREAD_IDX = idx;
        switch_to_thread(thread);
    }
}

pub fn start_scheduler() -> ! {
    unsafe {
        if let Some(ref thread) = THREADS[0] {
            switch_to_thread(thread);
        } else {
            switch_to_thread(addr_of!(IDLE_THREAD));
        }
    }
}

pub fn init_scheduler() {
    unsafe {
        IDLE_THREAD.eip = idle_proc as usize as u32;
        let stack = addr_of!(IDLE_STACK) as *const u8;
        IDLE_THREAD.esp = stack.add(IDLE_STACK_SIZE) as u32;
    }
}
