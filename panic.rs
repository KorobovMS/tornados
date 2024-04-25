use crate::idt::{hang, disable_interrupts};
use crate::vga::Vga;

#[panic_handler]
fn panic(info: &core::panic::PanicInfo) -> ! {
    let mut vga = Vga::new();
    vga.clear_screen();
    if let Some(arg) = info.message() {
        let result = core::fmt::write(&mut vga, *arg);
        if let Err(core::fmt::Error) = result {
            vga.write("oh no");
        }
    } else {
        vga.write("unknown arg");
    }
    disable_interrupts();
    hang();
}
