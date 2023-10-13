pub static FONT: &[u8] = include_bytes!("unifont.bin");

pub trait GraphicsFramebuffer {
    type ColorType;
    fn pixel(&self, x: usize, y: usize, color: Self::ColorType);
    fn read_pixel(&self, x: usize, y: usize) -> Self::ColorType;
    fn rect(&self, x0: usize, y0: usize, x1: usize, y1: usize, border: Self::ColorType, fill: Self::ColorType);
    fn line(&self, x0: usize, y0: usize, x1: usize, y1: usize, color: Self::ColorType);
    fn clear(&self, color: Self::ColorType);
    fn character(&self, x: usize, y: usize, c: u8, color: Self::ColorType);
    fn string(&self, x: usize, y: usize, s: &[u8], wrap: Option<usize>, color: Self::ColorType);
}
