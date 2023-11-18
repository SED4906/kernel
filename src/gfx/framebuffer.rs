pub static FONT: &[u8] = include_bytes!("FM-TOWNS.F08");
const FONT_HEIGHT: usize = 8;
//pub static FONT: &[u8] = include_bytes!("unifont.bin");
use core::usize;

static FRAMEBUFFER_REQUEST: limine::FramebufferRequest = limine::FramebufferRequest::new(0);
pub static mut FRAMEBUFFER: Option<Framebuffer> = None;
pub fn framebuffer_init() {
    if let Some(framebuffer_response) = FRAMEBUFFER_REQUEST.get_response().get() {
        if framebuffer_response.framebuffer_count < 1 {
            crate::hcf()
        }
        // Get the first framebuffer's information.
        let response = &framebuffer_response.framebuffers()[0];
        unsafe{FRAMEBUFFER = Some(Framebuffer { address: response.address.as_ptr().unwrap() as *mut u32, pitch: response.pitch as usize, width: response.width as usize, height: response.height as usize })};
    } else {
        crate::hcf()
    }
}

pub struct Framebuffer {
    pub address: *mut u32,
    pub pitch: usize,
    pub width: usize,
    pub height: usize,
}

impl Framebuffer {
    pub fn pixel(&self, x: usize, y: usize, color: u32) {
        let pixel_offset = y * self.pitch / 4 + x;
        unsafe {
            *(self.address.add(pixel_offset)) = color;
        }
    }

    pub fn read_pixel(&self, x: usize, y: usize) -> u32 {
        let pixel_offset = y * self.pitch / 4 + x;
        return unsafe {
            *(self.address.add(pixel_offset))
        };
    }

    pub fn rect(&self, x0: usize, y0: usize, x1: usize, y1: usize, border: u32, fill: u32) {
        for py in y0..=y1 {
            for px in x0..=x1 {
                if px == x0 || px == x1 || py == y0 || py == y1 {
                    self.pixel(px, py, border)
                } else {
                    self.pixel(px, py, fill)
                }
            }
        }
    }

    pub fn line(&self, x0: usize, y0: usize, x1: usize, y1: usize, color: u32) {
        let mut x0: isize = x0.try_into().unwrap();
        let mut y0: isize = y0.try_into().unwrap();
        let x1: isize = x1.try_into().unwrap();
        let y1: isize = y1.try_into().unwrap();
        let dx: isize = if x1 > x0 { x1 - x0 } else { x0 - x1 };
        let sx: isize = if x0 < x1 { 1 } else { -1 };
        let dy: isize = if y1 > y0 { y0 - y1 } else { y1 - y0 };
        let sy: isize = if y0 < y1 { 1 } else { -1 };
        let mut err = dx + dy;
        loop {
            self.pixel(x0.unsigned_abs(), y0.unsigned_abs(), color);
            if x0 == x1 && y0 == y1 {
                break;
            }
            let e2 = 2 * err;
            if e2 >= dy {
                if x0 == x1 {
                    break;
                }
                err += dy;
                x0 += sx;
            }
            if e2 <= dx {
                if y0 == y1 {
                    break;
                }
                err += dx;
                y0 += sy;
            }
        }
    }

    pub fn circle(&self, xc: usize, yc: usize, r: usize, color: u32) {
        let mut t1: isize = r as isize/16;
        let mut x: isize = r as isize;
        let mut y: isize = 0;
        while x >= y {
            let xo = x as usize;
            let yo = y as usize;
            self.pixel(xc+xo, yc+yo, color);
            self.pixel(xc+xo, yc-yo, color);
            self.pixel(xc-xo, yc+yo, color);
            self.pixel(xc-xo, yc-yo, color);
            self.pixel(xc+yo, yc+xo, color);
            self.pixel(xc+yo, yc-xo, color);
            self.pixel(xc-yo, yc+xo, color);
            self.pixel(xc-yo, yc-xo, color);
            y += 1;
            t1 += y;
            let t2 = t1 - x;
            if t2 >= 0 {
                t1 = t2;
                x -= 1;
            }
        }
    }

    pub fn clear(&self, color: u32) {
        for y in 0..self.height {
            for x in 0..self.width {
                self.pixel(x, y, color)
            }
        }
    }

    pub fn character(&self, x: usize, y: usize, c: u8, color: u32) {
        for py in y..y + FONT_HEIGHT {
            for px in x..x + 8 {
                if crate::gfx::framebuffer::FONT[(c as usize) * FONT_HEIGHT + py as usize - y as usize] & (128 >> (px - x)) != 0 {
                    self.pixel(px, py, color);
                }
            }
        }
    }

    pub fn string(&self, x: usize, y: usize, s: &[u8], wrap: Option<usize>, color: u32) -> (usize, usize) {
        let mut line_length = 0;
        let mut line = 0;
        for c in s {
            match c {
                8 => line_length -= 1,
                9 => {
                    line_length += 8;
                    line_length &= !7;
                },
                13 => line_length = 0,
                10 => {
                    line_length = 0;
                    line += 1;
                }
                _ => {
                    self.character(x + line_length * 8, y + line * 8, *c, color);
                    line_length += 1;
                }
            };
            if let Some(wrap) = wrap {
                if line_length >= wrap {
                    line_length = 0;
                    line += 1;
                }
            }
        }
        return (line_length, line)
    }
}