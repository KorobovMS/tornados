use idt::{hang, disable_interrupts};
use vga::Vga;

#[panic_handler]
#[no_mangle]
pub extern "C" fn rust_begin_unwind(_info: &core::panic::PanicInfo) -> ! {
    let mut vga = Vga::new();
    if let Some(arg) = _info.message() {
        if let Some(msg) = arg.as_str() {
            vga.write(msg);
        } else {
            vga.write("unknown msg");
        }
    } else {
        vga.write("unknown arg");
    }
    disable_interrupts();
    hang();
}

#[lang = "eh_personality"]
fn eh_personality() {}
