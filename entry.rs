use core::fmt::{Write, Debug, Formatter, Result};
use core::mem;
use crate::idt;
use crate::pic;
use crate::serial;
use crate::vga::Vga;
use crate::sched;

extern "C" {
    static _multiboot_info: &'static MultibootInformation;
}

#[repr(C)]
pub struct MultibootInformation {
    flags: u32,
    mem_lower: u32,
    mem_upper: u32,
    boot_device: u32,
    cmdline: u32,
    mods_count: u32,
    mods_addr: u32,
    syms: [u32; 4],
    mmap_length: u32,
    mmap_addr: u32,
    drives_length: u32,
    drives_addr: u32,
    config_table: u32,
    boot_loader_name: u32,
    apm_table: u32,
    vbe_control_info: u32,
    vbe_mode_info: u32,
    vbe_mode: u16,
    vbe_interface_seg: u16,
    vbe_interface_off: u16,
    vbe_interface_len: u16,
    framebuffer_addr: u64,
    framebuffer_pitch: u32,
    framebuffer_width: u32,
    framebuffer_height: u32,
    framebuffer_bpp: u8,
    framebuffer_type: u8,
    color_info: [u8; 6],
}
const SA_MULTIBOOT_INFORMATION_SIZE: usize =
    (mem::size_of::<MultibootInformation>() == 116) as usize - 1;

#[repr(C)]
struct MultibootModule {
    mod_start: u32,
    mod_end: u32,
    cmdline: u32,
    reserved: u32,
}

#[repr(C)]
struct MultibootMemory {
    size: u32,
    base_addr_low: u32,
    base_addr_high: u32,
    length_low: u32,
    length_high: u32,
    mem_type: u32,
}
const SA_MULTIBOOT_MEMORY_SIZE: usize =
    (mem::size_of::<MultibootMemory>() == 24) as usize - 1;

impl MultibootMemory {
    fn base_addr(&self) -> u64 {
        let mut ret = self.base_addr_high as u64;
        ret <<= 32;
        ret |= self.base_addr_low as u64;
        ret
    }

    fn length(&self) -> u64 {
        let mut ret = self.length_high as u64;
        ret <<= 32;
        ret |= self.length_low as u64;
        ret
    }

    fn end_addr(&self) -> u64 {
        self.base_addr() + self.length()
    }
}

unsafe fn slice_from_cstr(s: *const u8) -> &'static [u8] {
    let mut count = 0usize;
    loop {
        if *s.add(count) == 0 {
            break;
        }
        count += 1;
    }
    core::slice::from_raw_parts(s, count)
}

fn mem_type_to_str(mem_type: u32) -> &'static str {
    match mem_type {
        1 => "available",
        3 => "ACPI",
        4 => "hibernation",
        5 => "defective",
        _ => "reserved",
    }
}

impl Debug for MultibootInformation {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        f.write_str("MB:\n")?;
        f.write_fmt(format_args!(
                "flags 0x{:08X}\n", self.flags))?;

        if self.flags & (1 << 0) != 0 {
            f.write_fmt(format_args!(
                    "range 0x{:08X} - 0x{:08X}\n",
                    self.mem_lower, self.mem_upper))?;
        }

        if self.flags & (1 << 1) != 0 {
            f.write_fmt(format_args!(
                    "boot device 0x{:02X}\n", self.boot_device))?;
        }

        if self.flags & (1 << 2) != 0 {
            let cmdline = self.cmdline as *const u8;
            let cmdline = unsafe { slice_from_cstr(cmdline) };
            f.write_str(core::str::from_utf8(cmdline).unwrap())?;
            f.write_str("\n")?;
        }

        if self.flags & (1 << 3) != 0 {
            let modules = unsafe { core::slice::from_raw_parts(
                self.mods_addr as *const MultibootModule,
                self.mods_count as usize) };
            for module in modules {
                let cmdline = module.cmdline as *const u8;
                let cmdline = unsafe { slice_from_cstr(cmdline) };
                f.write_fmt(format_args!(
                        "mod 0x{:08X} - 0x{:08X} {}\n",
                        module.mod_start, module.mod_end,
                        core::str::from_utf8(cmdline).unwrap()))?;
            }
        }

        if self.flags & (1 << 4) != 0 || self.flags & (1 << 5) != 0 {
            // self.syms;
        }

        if self.flags & (1 << 6) != 0 {
            unsafe {
                let mut sum_size = 0u32;
                let mut ptr = self.mmap_addr as *const MultibootMemory;
                while sum_size < self.mmap_length {
                    f.write_fmt(format_args!(
                            "mem 0x{:016X} - 0x{:016X} {}\n",
                            (*ptr).base_addr(), (*ptr).end_addr(),
                            mem_type_to_str((*ptr).mem_type)))?;
                    let size = (*ptr).size + 4;
                    ptr = (ptr as *const u8).add(size as usize) as *const MultibootMemory;
                    sum_size += size;
                }
            }
        }

        if self.flags & (1 << 7) != 0 {
            // self.drives_length;
            // self.drives_addr;
        }

        if self.flags & (1 << 8) != 0 {
            // self.config_table;
        }

        if self.flags & (1 << 9) != 0 {
            // self.boot_loader_name;
        }

        if self.flags & (1 << 10) != 0 {
            // self.apm_table;
        }

        if self.flags & (1 << 11) != 0 {
            // vbe
        }

        if self.flags & (1 << 12) != 0 {
            // framebuffer
        }

        Ok(())
    }
}

fn busy_wait() {
    let mut i = 0;
    while i < 1000000 {
        i += 1;
    }
}

fn kernel_thread_proc()
{
    loop {
        serial::write_str("1");
        busy_wait();
    }
}

extern "C" {
    fn kcall() -> u32;
}

fn user_thread_proc()
{
    let mut vga = Vga::new();
    loop {
        let x = unsafe { kcall() };
        write!(vga, "{}", x).unwrap();
        busy_wait();
    }
}

#[no_mangle]
pub fn kernel_main() -> ! {
    idt::setup_idt();
    pic::remap(0x20, 0x28);
    pic::mask(0xEC, 0xFF);
    serial::serial_init();
    serial::write_str("Booting kernel...\n");
    let mut vga = Vga::new();
    vga.clear_screen();
    write!(vga, "{:?}", unsafe { _multiboot_info }).unwrap();
    sched::init_scheduler();
    sched::create_kernel_thread(kernel_thread_proc as *const ());
    sched::create_user_thread(user_thread_proc as *const ());
    sched::start_scheduler();
}
