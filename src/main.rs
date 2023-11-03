#![feature(abi_x86_interrupt)]
#![feature(panic_info_message)]
#![feature(ptr_from_ref)]
#![feature(slice_pattern)]
#![feature(naked_functions)]
#![no_std]
#![no_main]

use error::KernelError;

use crate::cpu::descriptors::desc_init;
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

#[no_mangle]
unsafe extern "C" fn _start() -> ! {
    let _ = serial::serial_init();
    framebuffer_init();
    println!("Object Kernel");
    desc_init();
    mm_init();
    //load_drivers().expect("couldn't load drivers");
    create_process(&[0xEB, 0xFE]).expect("couldn't create process");
    switch_to();
    loop {}
}

//static MODULE_REQUEST: limine::ModuleRequest = limine::ModuleRequest::new(1);

/*unsafe fn load_drivers() -> Result<(), KernelError> {
    for module in MODULE_REQUEST.get_response().get().ok_or(KernelError::LoadDrivers)?.modules() {
        if !module.cmdline.to_str().ok_or(KernelError::LoadDrivers)?.to_str().or(Err(KernelError::LoadDrivers))?.starts_with("driver") {
            continue;
        }
        let base = module.base.as_ptr().ok_or(KernelError::LoadDrivers)?;
        let length = module.length as usize;
        let image = core::slice::from_raw_parts_mut(base, length);
        create_process(image, false)?;
        println!("Loaded driver {}", module.path.to_str().ok_or(KernelError::LoadDrivers)?.to_str().or(Err(KernelError::LoadDrivers))?);
    }
    Ok(())
}*/

#[panic_handler]
fn rust_panic(_info: &core::panic::PanicInfo) -> ! {
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