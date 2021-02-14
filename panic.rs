use vga::Vga;

extern "C" {
    fn hang() -> !;
}

#[panic_handler]
fn panic_impl(_info: &core::panic::PanicInfo) -> ! {
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
    unsafe {
        hang();
    }
}

#[lang = "eh_personality"]
fn eh_personality() {}