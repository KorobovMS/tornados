use core::fmt::{Write, Debug, Formatter, Result};
use core::mem;
use gdt;
use idt;
use pic;
use serial;
use vga::Vga;
use sched;

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
    base_addr: u64,
    length: u64,
    mem_type: u32,
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
                            (*ptr).base_addr, (*ptr).base_addr + (*ptr).length,
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

fn thread1_proc()
{
    loop {
        serial::write_str("1");
        sched::stop_thread(0);
        busy_wait();
    }
}

fn thread2_proc()
{
    loop {
        serial::write_str("2");
        sched::stop_thread(1);
        busy_wait();
    }
}


fn thread3_proc()
{
    loop {
        serial::write_str("3");
        sched::stop_thread(2);
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
        sched::stop_thread(3);
        busy_wait();
    }
}

fn thread5_proc()
{
    let mut vga = Vga::new();
    loop {
        vga.write("123");
        sched::stop_thread(4);
        busy_wait();
    }
}

#[no_mangle]
pub fn kernel_main() -> ! {
    gdt::setup_gdt();
    idt::setup_idt();
    pic::remap(0x20, 0x28);
    pic::mask(0xEC, 0xFF);
    serial::serial_init();
    serial::write_str("Booting kernel...\n");
    let mut vga = Vga::new();
    vga.clear_screen();
    write!(&mut vga, "{:?}", unsafe { _multiboot_info }).unwrap();
    sched::init_scheduler();
    sched::create_kernel_thread(thread1_proc as *const ());
    sched::create_kernel_thread(thread2_proc as *const ());
    sched::create_kernel_thread(thread3_proc as *const ());
    sched::create_kernel_thread(thread4_proc as *const ());
    sched::create_user_thread(thread5_proc as *const ());
    sched::start_scheduler();
}
