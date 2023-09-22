use volatile::Volatile;
use super::*;

pub struct VGABufferWriter {
    color_code: VGAColorCode,
    buffer: &'static mut VGABuffer,
}

impl Writer for VGABufferWriter {
    fn new() -> VGABufferWriter {
        VGABufferWriter {
            color_code: VGAColorCode::new(VGAColor::White, VGAColor::Black),
            buffer: unsafe { &mut *(0xb8000 as *mut VGABuffer) },
        }
    }

    fn get_dimensions(&self) -> (usize, usize) {
        (BUFFER_WIDTH, BUFFER_HEIGHT)
    }

    fn set_color(&mut self, foreground_color: VGAColor, background_color: VGAColor) {
        self.color_code = VGAColorCode::new(foreground_color, background_color);
    }

    fn write_byte_at(&mut self, byte: u8, col: usize, row: usize) {
        self.buffer.chars[row][col].write(VGAChar {
            char: byte,
            color_code: self.color_code,
        });
    }

    fn read_byte_at(&mut self, col: usize, row: usize) -> (u8, VGAColor, VGAColor) {
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
    fn new(foreground: VGAColor, background: VGAColor) -> VGAColorCode {
        VGAColorCode((background as u8) << 4 | (foreground as u8))
    }

    fn get_colors(&self) -> (VGAColor, VGAColor) {
        (VGAColor::try_from(self.0 & 0b00001111).unwrap(), VGAColor::try_from((self.0 & 0b11110000) >> 4).unwrap())
    }
}

#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum VGAColor {
    Black = 0,
    Blue = 1,
    Green = 2,
    Cyan = 3,
    Red = 4,
    Magenta = 5,
    Brown = 6,
    LightGray = 7,
    DarkGray = 8,
    LightBlue = 9,
    LightGreen = 10,
    LightCyan = 11,
    LightRed = 12,
    Pink = 13,
    Yellow = 14,
    White = 15,
}

impl TryFrom<u8> for VGAColor {
    type Error = ();

    fn try_from(x: u8) -> Result<Self, Self::Error> {
        match x {
            0 => Ok(VGAColor::Black),
            1 => Ok(VGAColor::Blue),
            2 => Ok(VGAColor::Green),
            3 => Ok(VGAColor::Cyan),
            4 => Ok(VGAColor::Red),
            5 => Ok(VGAColor::Magenta),
            6 => Ok(VGAColor::Brown),
            7 => Ok(VGAColor::LightGray),
            8 => Ok(VGAColor::DarkGray),
            9 => Ok(VGAColor::LightBlue),
            10 => Ok(VGAColor::LightGreen),
            11 => Ok(VGAColor::LightCyan),
            12 => Ok(VGAColor::LightRed),
            13 => Ok(VGAColor::Pink),
            14 => Ok(VGAColor::Yellow),
            15 => Ok(VGAColor::White),
            _ => Ok(VGAColor::Black),
        }
    }
}