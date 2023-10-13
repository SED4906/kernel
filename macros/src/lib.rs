use proc_macro::TokenStream;

extern crate proc_macro;

#[proc_macro]
pub fn remaining_interrupt_handlers(_: TokenStream) -> TokenStream {
    let mut out_string = String::new();
    for num in 32..256 {
        out_string.push_str(&["extern \"x86-interrupt\" fn isr","(stack: InterruptStackFrame) {handle_interrupt!(no_err "," \"Interrupt #","\", stack);}"].join(num.to_string().as_str()));
    }
    out_string.parse().unwrap()
}

#[proc_macro]
pub fn set_remaining_interrupt_handlers(_: TokenStream) -> TokenStream {
    let mut out_string = String::new();
    for num in 32..256 {
        out_string.push_str(&["IDT[","].set_handler_fn(isr",");"].join(num.to_string().as_str()));
    }
    out_string.parse().unwrap()
}

#[proc_macro]
pub fn derive_gfxfb(_item: TokenStream) -> TokenStream {
    "fn rect(&self, x0: usize, y0: usize, x1: usize, y1: usize, border: Self::ColorType, fill: Self::ColorType) {
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

    fn line(&self, x0: usize, y0: usize, x1: usize, y1: usize, color: Self::ColorType) {
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

    fn clear(&self, color: u32) {
        for y in 0..self.height {
            for x in 0..self.width {
                self.pixel(x, y, color)
            }
        }
    }

    fn character(&self, x: usize, y: usize, c: u8, color: Self::ColorType) {
        for py in y..y + 16 {
            for px in x..x + 8 {
                if crate::gfx::framebuffer::FONT[(c as usize) * 16 + py as usize - y as usize] & (128 >> (px - x)) != 0 {
                    self.pixel(px, py, color);
                }
            }
        }
    }

    fn string(&self, x: usize, y: usize, s: &[u8], wrap: Option<usize>, color: Self::ColorType) {
        let mut line_length = 0;
        let mut line = 0;
        for c in s {
            match c {
                8 => line_length -= 1,
                9 => line_length += 8,
                13 => line_length = 0,
                10 => {
                    line_length = 0;
                    line += 1;
                }
                _ => {
                    self.character(x + line_length * 8, y + line * 16, *c, color);
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
    }".parse().unwrap()
}
