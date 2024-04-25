use crate::sched;
use crate::serial;
use crate::pic;
use crate::vga::Vga;
use core::arch::asm;
use core::fmt::Write;
use core::ptr::addr_of_mut;

static mut VGA: Vga = Vga::new();

extern "C" {
    static mut _idt: u64;
}

pub const X86_INT_STATE_EBP: u32 = 0;
pub const X86_INT_STATE_EDI: u32 = 1;
pub const X86_INT_STATE_ESI: u32 = 2;
pub const X86_INT_STATE_EDX: u32 = 3;
pub const X86_INT_STATE_ECX: u32 = 4;
pub const X86_INT_STATE_EBX: u32 = 5;
pub const X86_INT_STATE_EAX: u32 = 6;
pub const X86_INT_STATE_VEC: u32 = 7;
pub const X86_INT_STATE_ERR: u32 = 8;
pub const X86_INT_STATE_EIP: u32 = 9;
pub const X86_INT_STATE_CS: u32 = 10;
pub const X86_INT_STATE_EFLAGS: u32 = 11;
pub const X86_INT_STATE_ESP: u32 = 12;
pub const X86_INT_STATE_SS: u32 = 13;

macro_rules! interrupt_handler_with_code {
    ($isr_entry:ident $vec_num:expr) => {
        #[naked]
        extern "C" fn $isr_entry() {
            unsafe {
                asm!(
                    // Exception's error code was pushed by CPU
                    // Push vector's number
                    concat!("push ", stringify!($vec_num)),
                    // Push CPU's state
                    "push eax",
                    "push ebx",
                    "push ecx",
                    "push edx",
                    "push esi",
                    "push edi",
                    "push ebp",
                    "push esp",
                    "call handle_interrupt",
                    // On return just restore previous CPU state
                    "pop esp",
                    "pop ebp",
                    "pop edi",
                    "pop esi",
                    "pop edx",
                    "pop ecx",
                    "pop ebx",
                    "pop eax",
                    // Pop both error code and vector's number
                    "add esp, 8",
                    "iretd",
                    options(noreturn),
                    );
            }
        }
    }
}

macro_rules! interrupt_handler_without_code {
    ($isr_entry:ident $vec_num:expr) => {
        #[naked]
        extern "C" fn $isr_entry() {
            unsafe {
                asm!(
                    // Zero for interrupt stack layout consistency
                    "push 0",
                    // Push vector's number
                    concat!("push ", stringify!($vec_num)),
                    // Push CPU's state
                    "push eax",
                    "push ebx",
                    "push ecx",
                    "push edx",
                    "push esi",
                    "push edi",
                    "push ebp",
                    "push esp",
                    "call handle_interrupt",
                    // On return just restore previous CPU state
                    "pop esp",
                    "pop ebp",
                    "pop edi",
                    "pop esi",
                    "pop edx",
                    "pop ecx",
                    "pop ebx",
                    "pop eax",
                    // Pop both zero and vector's number
                    "add esp, 8",
                    "iretd",
                    options(noreturn),
                    );
            }
        }
    }
}

const X86_EXC_DIVIDE_ERROR: u32 = 0;
const X86_EXC_DEBUG: u32 = 1;
const X86_EXC_NMI: u32 = 2;
const X86_EXC_BREAKPOINT: u32 = 3;
const X86_EXC_OVERFLOW: u32 = 4;
const X86_EXC_BOUND_RANGE_EXCEEDED: u32 = 5;
const X86_EXC_INVALID_OPCODE: u32 = 6;
const X86_EXC_DEVICE_NOT_AVAILABLE: u32 = 7;
const X86_EXC_DOUBLE_FAULT: u32 = 8;
const X86_EXC_COPROCESSOR_SEGMENT_OVERRUN: u32 = 9;
const X86_EXC_INVALID_TSS: u32 = 10;
const X86_EXC_SEGMENT_NOT_PRESENT: u32 = 11;
const X86_EXC_STACK_FAULT: u32 = 12;
const X86_EXC_GENERAL_PROTECTION: u32 = 13;
const X86_EXC_PAGE_FAULT: u32 = 14;
// no 15
const X86_EXC_X87_FPU_FLOATING_POINT_ERROR: u32 = 16;
const X86_EXC_ALIGNMENT_CHECK: u32 = 17;
const X86_EXC_MACHINE_CHECK: u32 = 18;
const X86_EXC_SIMD_FLOATING_POINT: u32 = 19;
const X86_EXC_VIRTUALIZATION: u32 = 20;
const X86_EXC_CONTROL_PROTECTION: u32 = 21;

#[no_mangle]
extern "C" fn handle_interrupt(int_state: *const u32) {
    let vec: u32 = unsafe { *int_state.offset(7) };
    let err: u32 = unsafe { *int_state.offset(8) };
    match vec {
        0x20 => {
            sched::save_current_state(int_state);
            pic::end_of_interrupt(0);
            sched::invoke_scheduler();
        },
        0x21 => {
            serial::write_str("k");
            pic::end_of_interrupt(1);
        },
        0x24 => {
            let b = serial::get_byte();
            unsafe { write!(VGA, "{}", b as char).unwrap(); }
            serial::write_str("4");
            pic::end_of_interrupt(4);
        },
        _ => {
            let thread = sched::current();
            panic!("\
                interrupt {}, error {}\n\
                thread:\n\
                {}",
                vec, err, thread);
        },
    }
}

pub fn setup_irq_handler(idx: u8, handler: *const ()) {
    let handler = handler as u64;
    let lo = handler & 0xFFFF;
    let hi = ((handler >> 16) & 0xFFFF) << 48;
    let kernel_cs = 0x8u16;
    let cs = (kernel_cs as u64) << 16;
    let fl = 0x8E00u64 << 32;
    let idt_desc: u64 = lo | cs | fl | hi;
    unsafe {
        let idt = addr_of_mut!(_idt) as *mut u64;
        *idt.offset(idx as isize) = idt_desc;
    }
}

pub fn setup_idt() {
    setup_irq_handler(0, isr_0 as *const ());
    setup_irq_handler(1, isr_1 as *const ());
    setup_irq_handler(2, isr_2 as *const ());
    setup_irq_handler(3, isr_3 as *const ());
    setup_irq_handler(4, isr_4 as *const ());
    setup_irq_handler(5, isr_5 as *const ());
    setup_irq_handler(6, isr_6 as *const ());
    setup_irq_handler(7, isr_7 as *const ());
    setup_irq_handler(8, isr_8 as *const ());
    setup_irq_handler(9, isr_9 as *const ());
    setup_irq_handler(10, isr_10 as *const ());
    setup_irq_handler(11, isr_11 as *const ());
    setup_irq_handler(12, isr_12 as *const ());
    setup_irq_handler(13, isr_13 as *const ());
    setup_irq_handler(14, isr_14 as *const ());
    // No interrupt 15
    setup_irq_handler(16, isr_16 as *const ());
    setup_irq_handler(17, isr_17 as *const ());
    setup_irq_handler(18, isr_18 as *const ());
    setup_irq_handler(19, isr_19 as *const ());
    setup_irq_handler(20, isr_20 as *const ());
    setup_irq_handler(21, isr_21 as *const ());
    setup_irq_handler(0x20, isr_32 as *const ());
    setup_irq_handler(0x21, isr_33 as *const ());
    setup_irq_handler(0x24, isr_36 as *const ());
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

interrupt_handler_without_code!(isr_0 0);
interrupt_handler_without_code!(isr_1 1);
interrupt_handler_without_code!(isr_2 2);
interrupt_handler_without_code!(isr_3 3);
interrupt_handler_without_code!(isr_4 4);
interrupt_handler_without_code!(isr_5 5);
interrupt_handler_without_code!(isr_6 6);
interrupt_handler_without_code!(isr_7 7);
interrupt_handler_with_code!(isr_8 8);
interrupt_handler_without_code!(isr_9 9);
interrupt_handler_with_code!(isr_10 10);
interrupt_handler_with_code!(isr_11 11);
interrupt_handler_with_code!(isr_12 12);
interrupt_handler_with_code!(isr_13 13);
interrupt_handler_with_code!(isr_14 14);
// no isr 15
interrupt_handler_without_code!(isr_16 16);
interrupt_handler_with_code!(isr_17 17);
interrupt_handler_without_code!(isr_18 18);
interrupt_handler_without_code!(isr_19 19);
interrupt_handler_without_code!(isr_20 20);
interrupt_handler_without_code!(isr_21 21);
interrupt_handler_without_code!(isr_22 22);
interrupt_handler_without_code!(isr_23 23);
interrupt_handler_without_code!(isr_24 24);
interrupt_handler_without_code!(isr_25 25);
interrupt_handler_without_code!(isr_26 26);
interrupt_handler_without_code!(isr_27 27);
interrupt_handler_without_code!(isr_28 28);
interrupt_handler_without_code!(isr_29 29);
interrupt_handler_without_code!(isr_30 30);
interrupt_handler_without_code!(isr_31 31);
interrupt_handler_without_code!(isr_32 32);
interrupt_handler_without_code!(isr_33 33);
interrupt_handler_without_code!(isr_34 34);
interrupt_handler_without_code!(isr_35 35);
interrupt_handler_without_code!(isr_36 36);
interrupt_handler_without_code!(isr_37 37);
interrupt_handler_without_code!(isr_38 38);
interrupt_handler_without_code!(isr_39 39);
interrupt_handler_without_code!(isr_40 40);
interrupt_handler_without_code!(isr_41 41);
interrupt_handler_without_code!(isr_42 42);
interrupt_handler_without_code!(isr_43 43);
interrupt_handler_without_code!(isr_44 44);
interrupt_handler_without_code!(isr_45 45);
interrupt_handler_without_code!(isr_46 46);
interrupt_handler_without_code!(isr_47 47);
interrupt_handler_without_code!(isr_48 48);
interrupt_handler_without_code!(isr_49 49);
interrupt_handler_without_code!(isr_50 50);
interrupt_handler_without_code!(isr_51 51);
interrupt_handler_without_code!(isr_52 52);
interrupt_handler_without_code!(isr_53 53);
interrupt_handler_without_code!(isr_54 54);
interrupt_handler_without_code!(isr_55 55);
interrupt_handler_without_code!(isr_56 56);
interrupt_handler_without_code!(isr_57 57);
interrupt_handler_without_code!(isr_58 58);
interrupt_handler_without_code!(isr_59 59);
interrupt_handler_without_code!(isr_60 60);
interrupt_handler_without_code!(isr_61 61);
interrupt_handler_without_code!(isr_62 62);
interrupt_handler_without_code!(isr_63 63);
interrupt_handler_without_code!(isr_64 64);
interrupt_handler_without_code!(isr_65 65);
interrupt_handler_without_code!(isr_66 66);
interrupt_handler_without_code!(isr_67 67);
interrupt_handler_without_code!(isr_68 68);
interrupt_handler_without_code!(isr_69 69);
interrupt_handler_without_code!(isr_70 70);
interrupt_handler_without_code!(isr_71 71);
interrupt_handler_without_code!(isr_72 72);
interrupt_handler_without_code!(isr_73 73);
interrupt_handler_without_code!(isr_74 74);
interrupt_handler_without_code!(isr_75 75);
interrupt_handler_without_code!(isr_76 76);
interrupt_handler_without_code!(isr_77 77);
interrupt_handler_without_code!(isr_78 78);
interrupt_handler_without_code!(isr_79 79);
interrupt_handler_without_code!(isr_80 80);
interrupt_handler_without_code!(isr_81 81);
interrupt_handler_without_code!(isr_82 82);
interrupt_handler_without_code!(isr_83 83);
interrupt_handler_without_code!(isr_84 84);
interrupt_handler_without_code!(isr_85 85);
interrupt_handler_without_code!(isr_86 86);
interrupt_handler_without_code!(isr_87 87);
interrupt_handler_without_code!(isr_88 88);
interrupt_handler_without_code!(isr_89 89);
interrupt_handler_without_code!(isr_90 90);
interrupt_handler_without_code!(isr_91 91);
interrupt_handler_without_code!(isr_92 92);
interrupt_handler_without_code!(isr_93 93);
interrupt_handler_without_code!(isr_94 94);
interrupt_handler_without_code!(isr_95 95);
interrupt_handler_without_code!(isr_96 96);
interrupt_handler_without_code!(isr_97 97);
interrupt_handler_without_code!(isr_98 98);
interrupt_handler_without_code!(isr_99 99);
interrupt_handler_without_code!(isr_100 100);
interrupt_handler_without_code!(isr_101 101);
interrupt_handler_without_code!(isr_102 102);
interrupt_handler_without_code!(isr_103 103);
interrupt_handler_without_code!(isr_104 104);
interrupt_handler_without_code!(isr_105 105);
interrupt_handler_without_code!(isr_106 106);
interrupt_handler_without_code!(isr_107 107);
interrupt_handler_without_code!(isr_108 108);
interrupt_handler_without_code!(isr_109 109);
interrupt_handler_without_code!(isr_110 110);
interrupt_handler_without_code!(isr_111 111);
interrupt_handler_without_code!(isr_112 112);
interrupt_handler_without_code!(isr_113 113);
interrupt_handler_without_code!(isr_114 114);
interrupt_handler_without_code!(isr_115 115);
interrupt_handler_without_code!(isr_116 116);
interrupt_handler_without_code!(isr_117 117);
interrupt_handler_without_code!(isr_118 118);
interrupt_handler_without_code!(isr_119 119);
interrupt_handler_without_code!(isr_120 120);
interrupt_handler_without_code!(isr_121 121);
interrupt_handler_without_code!(isr_122 122);
interrupt_handler_without_code!(isr_123 123);
interrupt_handler_without_code!(isr_124 124);
interrupt_handler_without_code!(isr_125 125);
interrupt_handler_without_code!(isr_126 126);
interrupt_handler_without_code!(isr_127 127);
interrupt_handler_without_code!(isr_128 128);
interrupt_handler_without_code!(isr_129 129);
interrupt_handler_without_code!(isr_130 130);
interrupt_handler_without_code!(isr_131 131);
interrupt_handler_without_code!(isr_132 132);
interrupt_handler_without_code!(isr_133 133);
interrupt_handler_without_code!(isr_134 134);
interrupt_handler_without_code!(isr_135 135);
interrupt_handler_without_code!(isr_136 136);
interrupt_handler_without_code!(isr_137 137);
interrupt_handler_without_code!(isr_138 138);
interrupt_handler_without_code!(isr_139 139);
interrupt_handler_without_code!(isr_140 140);
interrupt_handler_without_code!(isr_141 141);
interrupt_handler_without_code!(isr_142 142);
interrupt_handler_without_code!(isr_143 143);
interrupt_handler_without_code!(isr_144 144);
interrupt_handler_without_code!(isr_145 145);
interrupt_handler_without_code!(isr_146 146);
interrupt_handler_without_code!(isr_147 147);
interrupt_handler_without_code!(isr_148 148);
interrupt_handler_without_code!(isr_149 149);
interrupt_handler_without_code!(isr_150 150);
interrupt_handler_without_code!(isr_151 151);
interrupt_handler_without_code!(isr_152 152);
interrupt_handler_without_code!(isr_153 153);
interrupt_handler_without_code!(isr_154 154);
interrupt_handler_without_code!(isr_155 155);
interrupt_handler_without_code!(isr_156 156);
interrupt_handler_without_code!(isr_157 157);
interrupt_handler_without_code!(isr_158 158);
interrupt_handler_without_code!(isr_159 159);
interrupt_handler_without_code!(isr_160 160);
interrupt_handler_without_code!(isr_161 161);
interrupt_handler_without_code!(isr_162 162);
interrupt_handler_without_code!(isr_163 163);
interrupt_handler_without_code!(isr_164 164);
interrupt_handler_without_code!(isr_165 165);
interrupt_handler_without_code!(isr_166 166);
interrupt_handler_without_code!(isr_167 167);
interrupt_handler_without_code!(isr_168 168);
interrupt_handler_without_code!(isr_169 169);
interrupt_handler_without_code!(isr_170 170);
interrupt_handler_without_code!(isr_171 171);
interrupt_handler_without_code!(isr_172 172);
interrupt_handler_without_code!(isr_173 173);
interrupt_handler_without_code!(isr_174 174);
interrupt_handler_without_code!(isr_175 175);
interrupt_handler_without_code!(isr_176 176);
interrupt_handler_without_code!(isr_177 177);
interrupt_handler_without_code!(isr_178 178);
interrupt_handler_without_code!(isr_179 179);
interrupt_handler_without_code!(isr_180 180);
interrupt_handler_without_code!(isr_181 181);
interrupt_handler_without_code!(isr_182 182);
interrupt_handler_without_code!(isr_183 183);
interrupt_handler_without_code!(isr_184 184);
interrupt_handler_without_code!(isr_185 185);
interrupt_handler_without_code!(isr_186 186);
interrupt_handler_without_code!(isr_187 187);
interrupt_handler_without_code!(isr_188 188);
interrupt_handler_without_code!(isr_189 189);
interrupt_handler_without_code!(isr_190 190);
interrupt_handler_without_code!(isr_191 191);
interrupt_handler_without_code!(isr_192 192);
interrupt_handler_without_code!(isr_193 193);
interrupt_handler_without_code!(isr_194 194);
interrupt_handler_without_code!(isr_195 195);
interrupt_handler_without_code!(isr_196 196);
interrupt_handler_without_code!(isr_197 197);
interrupt_handler_without_code!(isr_198 198);
interrupt_handler_without_code!(isr_199 199);
interrupt_handler_without_code!(isr_200 200);
interrupt_handler_without_code!(isr_201 201);
interrupt_handler_without_code!(isr_202 202);
interrupt_handler_without_code!(isr_203 203);
interrupt_handler_without_code!(isr_204 204);
interrupt_handler_without_code!(isr_205 205);
interrupt_handler_without_code!(isr_206 206);
interrupt_handler_without_code!(isr_207 207);
interrupt_handler_without_code!(isr_208 208);
interrupt_handler_without_code!(isr_209 209);
interrupt_handler_without_code!(isr_210 210);
interrupt_handler_without_code!(isr_211 211);
interrupt_handler_without_code!(isr_212 212);
interrupt_handler_without_code!(isr_213 213);
interrupt_handler_without_code!(isr_214 214);
interrupt_handler_without_code!(isr_215 215);
interrupt_handler_without_code!(isr_216 216);
interrupt_handler_without_code!(isr_217 217);
interrupt_handler_without_code!(isr_218 218);
interrupt_handler_without_code!(isr_219 219);
interrupt_handler_without_code!(isr_220 220);
interrupt_handler_without_code!(isr_221 221);
interrupt_handler_without_code!(isr_222 222);
interrupt_handler_without_code!(isr_223 223);
interrupt_handler_without_code!(isr_224 224);
interrupt_handler_without_code!(isr_225 225);
interrupt_handler_without_code!(isr_226 226);
interrupt_handler_without_code!(isr_227 227);
interrupt_handler_without_code!(isr_228 228);
interrupt_handler_without_code!(isr_229 229);
interrupt_handler_without_code!(isr_230 230);
interrupt_handler_without_code!(isr_231 231);
interrupt_handler_without_code!(isr_232 232);
interrupt_handler_without_code!(isr_233 233);
interrupt_handler_without_code!(isr_234 234);
interrupt_handler_without_code!(isr_235 235);
interrupt_handler_without_code!(isr_236 236);
interrupt_handler_without_code!(isr_237 237);
interrupt_handler_without_code!(isr_238 238);
interrupt_handler_without_code!(isr_239 239);
interrupt_handler_without_code!(isr_240 240);
interrupt_handler_without_code!(isr_241 241);
interrupt_handler_without_code!(isr_242 242);
interrupt_handler_without_code!(isr_243 243);
interrupt_handler_without_code!(isr_244 244);
interrupt_handler_without_code!(isr_245 245);
interrupt_handler_without_code!(isr_246 246);
interrupt_handler_without_code!(isr_247 247);
interrupt_handler_without_code!(isr_248 248);
interrupt_handler_without_code!(isr_249 249);
interrupt_handler_without_code!(isr_250 250);
interrupt_handler_without_code!(isr_251 251);
interrupt_handler_without_code!(isr_252 252);
interrupt_handler_without_code!(isr_253 253);
interrupt_handler_without_code!(isr_254 254);
interrupt_handler_without_code!(isr_255 255);
