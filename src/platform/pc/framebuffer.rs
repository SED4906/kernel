use core::usize;

use macros::derive_gfxfb;

use crate::gfx::framebuffer::GraphicsFramebuffer;

static FRAMEBUFFER_REQUEST: limine::FramebufferRequest = limine::FramebufferRequest::new(0);
pub static mut FRAMEBUFFER: Option<Framebuffer> = None;
pub fn get_framebuffer() {
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

impl GraphicsFramebuffer for Framebuffer {
    type ColorType = u32;

    fn pixel(&self, x: usize, y: usize, color: Self::ColorType) {
        let pixel_offset = y * self.pitch / 4 + x;
        unsafe {
            *(self.address.add(pixel_offset)) = color;
        }
    }

    fn read_pixel(&self, x: usize, y: usize) -> Self::ColorType {
        let pixel_offset = y * self.pitch / 4 + x;
        return unsafe {
            *(self.address.add(pixel_offset))
        };
    }

    derive_gfxfb!();
}
