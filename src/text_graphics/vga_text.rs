use volatile::Volatile;
use super::*;

pub struct VGABufferWriter {
    color_code: VGAColorCode,
    buffer: &'static mut VGABuffer,
}

impl Writer for VGABufferWriter {
    fn new() -> VGABufferWriter {
        VGABufferWriter {
            color_code: VGAColorCode::new(Color::White, Color::Black),
            buffer: unsafe { &mut *(0xb8000 as *mut VGABuffer) },
        }
    }

    fn get_dimensions(&self) -> (usize, usize) {
        (BUFFER_WIDTH, BUFFER_HEIGHT)
    }

    fn set_color(&mut self, foreground_color: Color, background_color: Color) {
        self.color_code = VGAColorCode::new(foreground_color, background_color);
    }

    fn write_byte_at(&mut self, byte: u8, col: usize, row: usize) {
        self.buffer.chars[row][col].write(VGAChar {
            char: byte,
            color_code: self.color_code,
        });
    }

    fn read_byte_at(&mut self, col: usize, row: usize) -> (u8, Color, Color) {
        let vga_char = self.buffer.chars[row][col].read();
        let (foreground_color, background_color) = vga_char.color_code.get_colors();
        (vga_char.char, foreground_color, background_color)
    }

    fn fill(&mut self, byte: u8) {
        for row in 0..BUFFER_HEIGHT {
            for col in 0..BUFFER_WIDTH {
                self.write_byte_at(byte, col, row);
            }
        }
    }

    fn clear(&mut self) {
        self.fill(b' ');
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(C)]
struct VGAChar {
    char: u8,
    color_code: VGAColorCode,
}

const BUFFER_HEIGHT: usize = 25;
const BUFFER_WIDTH: usize = 80;

#[repr(transparent)]
struct VGABuffer {
    chars: [[Volatile<VGAChar>; BUFFER_WIDTH]; BUFFER_HEIGHT],
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
struct VGAColorCode(u8);

impl VGAColorCode {
    fn new(foreground: Color, background: Color) -> VGAColorCode {
        VGAColorCode((background as u8) << 4 | (foreground as u8))
    }

    fn get_colors(&self) -> (Color, Color) {
        (Color::try_from(self.0 & 0b00001111).unwrap(), Color::try_from((self.0 & 0b11110000) >> 4).unwrap())
    }
}