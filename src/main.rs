#![feature(abi_x86_interrupt,panic_info_message,ptr_from_ref,slice_pattern,naked_functions,pointer_is_aligned)]
#![no_std]
#![no_main]

use error::KernelError;
use gfx::framebuffer;
use mm::vmm::KERNEL_PAGEMAP;

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
    desc_init();
    mm_init();
    pic_init();
    pit_init();
    load_modules().expect("couldn't load modules");
    switch_to();
    loop {}
}

static MODULE_REQUEST: limine::ModuleRequest = limine::ModuleRequest::new(0);

unsafe fn load_modules() -> Result<(), KernelError> {
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
        framebuffer.circle(framebuffer.width/2, framebuffer.height/2, 128, 0x80808080);
        framebuffer.line(framebuffer.width/2-90, framebuffer.height/2-90,framebuffer.width/2+90, framebuffer.height/2+90, 0x80808080);
    }
    hcf();
}

fn hcf() -> ! {
    x86_64::instructions::interrupts::disable();
    loop {
        x86_64::instructions::hlt();
    }
}