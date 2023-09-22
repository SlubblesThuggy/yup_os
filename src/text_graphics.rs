mod vga_text;
pub use vga_text::*;

pub trait Writer {
    fn new() -> Self;
    fn get_dimensions(&self) -> (usize, usize);
    fn set_color(&mut self, foreground_color: VGAColor, background_color: VGAColor);
    fn write_byte_at(&mut self, byte: u8, col: usize, row: usize);
    fn read_byte_at(&mut self, col: usize, row: usize) -> (u8, VGAColor, VGAColor);
    fn fill(&mut self, byte: u8);
    fn clear(&mut self);
}