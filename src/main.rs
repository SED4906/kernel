#![feature(abi_x86_interrupt,panic_info_message,ptr_from_ref,slice_pattern,naked_functions,pointer_is_aligned)]
#![no_std]
#![no_main]

use error::KernelError;
use gfx::framebuffer;

use crate::hw::descriptors::desc_init;
use crate::hw::pic::pic_init;
use crate::hw::pit::pit_init;
use crate::gfx::framebuffer::framebuffer_init;
use crate::mm::mm_init;
use crate::gfx::terminal;
use crate::process::{create_process, switch_to};
pub mod hw;
pub mod mm;
pub mod gfx;
pub mod process;
pub mod error;
pub mod serial;

#[no_mangle]
unsafe extern "C" fn _start() -> ! {
    let _ = serial::serial_init();
    framebuffer_init();
    println!("Object Kernel");
    if let Some(framebuffer) = unsafe {&framebuffer::FRAMEBUFFER} {
        framebuffer.rect(128, 128, 192, 192, 0xFFFF0000, 0x80FF0000);
        framebuffer.rect(136, 136, 200, 200, 0xFFFF8000, 0x80FF8000);
        framebuffer.rect(144, 144, 208, 208, 0xFF80FF00, 0x8080FF00);
        framebuffer.rect(152, 152, 216, 216, 0xFF00FF00, 0x8000FF00);
        framebuffer.rect(160, 160, 224, 224, 0xFF00FF80, 0x8000FF80);
        framebuffer.rect(168, 168, 232, 232, 0xFF0080FF, 0x800080FF);
        framebuffer.rect(176, 176, 240, 240, 0xFF0000FF, 0x800000FF);
    }
    desc_init();
    mm_init();
    load_modules().expect("couldn't load modules");
    pic_init();
    pit_init();
    switch_to();
    loop {}
}

static MODULE_REQUEST: limine::ModuleRequest = limine::ModuleRequest::new(0);

unsafe fn load_modules() -> Result<(), KernelError> {
    println!("Loading modules...");
    for module in MODULE_REQUEST.get_response().get().ok_or(KernelError::LoadModules)?.modules() {
        println!("Module path: {}",module.path.to_str().ok_or(KernelError::LoadModules)?.to_str().or(Err(KernelError::LoadModules))?);
        println!("Module cmdline: {}",module.cmdline.to_str().ok_or(KernelError::LoadModules)?.to_str().or(Err(KernelError::LoadModules))?);
        if !module.cmdline.to_str().ok_or(KernelError::LoadModules)?.to_str().or(Err(KernelError::LoadModules))?.starts_with("init") {
            continue;
        }
        let base = module.base.as_ptr().ok_or(KernelError::LoadModules)?;
        let length = module.length as usize;
        create_process(core::slice::from_raw_parts(base, length))?;
    }
    println!("Finished loading modules.");
    Ok(())
}

#[panic_handler]
fn rust_panic(_info: &core::panic::PanicInfo) -> ! {
    if let Some(message) = _info.message() {
        serial_println!("panic: {message}");
        println!();
        println!("                             ");
        println!("                             ");
        println!("    FLAGRANT SYSTEM ERROR    ");
        println!("       Computer over.        ");
        println!("      Panic = Very Yes.      ");
        println!("                             ");
        println!("                             ");
        println!("                             ");
        println!("{message}");
    }
    if let Some(framebuffer) = unsafe {&framebuffer::FRAMEBUFFER} {
        framebuffer.circle(framebuffer.width/2, framebuffer.height/2, 128, 0xFF808080);
        framebuffer.circle(framebuffer.width/2, framebuffer.height/2, 127, 0xFF808080);
        framebuffer.circle(framebuffer.width/2, framebuffer.height/2, 126, 0xFF808080);
        framebuffer.line(framebuffer.width/2-90, framebuffer.height/2-90,framebuffer.width/2+90, framebuffer.height/2+90, 0xFF808080);
        framebuffer.line(framebuffer.width/2-91, framebuffer.height/2-89,framebuffer.width/2+89, framebuffer.height/2+91, 0xFF808080);
        framebuffer.line(framebuffer.width/2-89, framebuffer.height/2-91,framebuffer.width/2+91, framebuffer.height/2+89, 0xFF808080);
    }
    hcf();
}

fn hcf() -> ! {
    x86_64::instructions::interrupts::disable();
    loop {
        x86_64::instructions::hlt();
    }
}