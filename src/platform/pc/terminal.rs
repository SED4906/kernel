use core::fmt;

use crate::gfx::{terminal::{Writer, COL, ROW}, framebuffer::GraphicsFramebuffer};

pub impl fmt::Write for Writer {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        crate::vendor::framebuffer::get_framebuffer();
        if let Some(framebuffer) = unsafe{&crate::gfx::framebuffer::FRAMEBUFFER} {
            for c in s.as_bytes() {
                unsafe {
                    match c {
                        8 => {
                            COL = COL.saturating_sub(1);
                        }
                        9 => {
                            COL += 8;
                            if COL >= framebuffer.width / 8 {
                                COL = 0;
                                ROW += 1;
                                if ROW >= framebuffer.height / 16 {
                                    ROW = 0;
                                }
                            }
                        }
                        13 => {
                            COL = 0;
                        }
                        10 => {
                            COL = 0;
                            ROW += 1;
                            if ROW >= framebuffer.height / 16 {
                                ROW = 0;
                            }
                        }
                        _ => {
                            framebuffer.rect(
                                COL * 8,
                                ROW * 16,
                                COL * 8 + 8,
                                ROW * 16 + 16,
                                0x00000000,
                                0x00000000,
                            );
                            framebuffer.character(COL * 8, ROW * 16, *c, 0xFFFFFFFF);
                            COL += 1;
                            if COL >= framebuffer.width / 8 {
                                COL = 0;
                                ROW += 1;
                                if ROW >= framebuffer.height / 16 {
                                    ROW = 0;
                                }
                            }
                        }
                    }
                }
            }
        }
        Ok(())
    }
}