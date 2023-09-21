#![no_std]
#![no_main]

use core::panic::PanicInfo;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

mod vga_buffer;
use vga_buffer::*;

#[no_mangle]
pub extern "C" fn _start() -> ! {
    /*
    use core::fmt::Write;

    WRITER.lock().write_string("How are you today?\n");
    WRITER.lock().write_string("Grötig? That sounds awful...\n");
    write!(WRITER.lock(), "Here, have some numbers: {} and {}\n", 69, 2.0/3.0).unwrap();
    */

    let mut writer = Writer::new(Color::Cyan, Color::Black);
    writer.write_string("How are you today?\n");

    loop {}
}
