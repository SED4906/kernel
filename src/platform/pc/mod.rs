pub mod pic;
pub mod framebuffer;
pub mod terminal;

pub unsafe fn platform_init() {
    pic::pic_remap();
}