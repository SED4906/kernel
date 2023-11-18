#![feature(abi_x86_interrupt)]
#![feature(panic_info_message)]
#![feature(ptr_from_ref)]
#![feature(slice_pattern)]
#![feature(naked_functions)]
#![no_std]
#![no_main]

use error::KernelError;
use gfx::framebuffer;

use crate::cpu::descriptors::desc_init;
use crate::pc::pic::pic_init;
use crate::pc::pit::pit_init;
use crate::gfx::framebuffer::framebuffer_init;
use crate::mm::mm_init;
use crate::gfx::terminal;
use crate::process::{create_process, switch_to};
pub mod mm;
pub mod gfx;
pub mod cpu;
pub mod process;
pub mod error;
pub mod serial;
pub mod pc;

#[no_mangle]
unsafe extern "C" fn _start() -> ! {
    let _ = serial::serial_init();
    framebuffer_init();
    println!("Object Kernel");
    desc_init();
    mm_init();
    pic_init();
    pit_init();
    load_init_process().expect("couldn't create first process");
    switch_to();
    loop {}
}

static MODULE_REQUEST: limine::ModuleRequest = limine::ModuleRequest::new(0);

unsafe fn load_init_process() -> Result<(), KernelError> {
    for module in MODULE_REQUEST.get_response().get().ok_or(KernelError::LoadInitProcess)?.modules() {
        println!("Loading driver {}", module.path.to_str().ok_or(KernelError::LoadInitProcess)?.to_str().or(Err(KernelError::LoadInitProcess))?);
        if !module.cmdline.to_str().ok_or(KernelError::LoadInitProcess)?.to_str().or(Err(KernelError::LoadInitProcess))?.starts_with("init") {
            continue;
        }
        let base = module.base.as_ptr().ok_or(KernelError::LoadInitProcess)?;
        let length = module.length as usize;
        let image = core::slice::from_raw_parts_mut(base, length);
        create_process(image)?;
        break;
    }
    Err(KernelError::LoadInitProcess)
}

#[panic_handler]
fn rust_panic(_info: &core::panic::PanicInfo) -> ! {
    unsafe {
        if let Some(framebuffer) = &framebuffer::FRAMEBUFFER {
            framebuffer.circle(framebuffer.width/2, framebuffer.height/2, 128, 0x80808080);
            framebuffer.line(framebuffer.width/2-90, framebuffer.height/2-90,framebuffer.width/2+90, framebuffer.height/2+90, 0x80808080);
        }
    }
    if let Some(message) = _info.message() {
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
    hcf();
}

fn hcf() -> ! {
    x86_64::instructions::interrupts::disable();
    loop {
        x86_64::instructions::hlt();
    }
}