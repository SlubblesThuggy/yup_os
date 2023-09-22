#![no_std]
#![no_main]

use core::panic::PanicInfo;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

mod text_graphics;
use text_graphics::*;

#[no_mangle]
pub extern "C" fn _start() -> ! {
    let mut writer: VGABufferWriter = VGABufferWriter::new(); 
    writer.set_color(VGAColor::Cyan, VGAColor::DarkGray);
    writer.clear();
    
    let (cols, rows) = writer.get_dimensions();
    for row in 0..rows {
        for col in 0..cols {
            writer.write_byte_at((col + row * cols) as u8, col, row);
        }
    }

    loop {}
}
