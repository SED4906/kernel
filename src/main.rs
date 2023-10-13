#![feature(abi_x86_interrupt)]
#![feature(panic_info_message)]
#![feature(ptr_from_ref)]
#![feature(slice_pattern)]
#![no_std]
#![no_main]

use crate::{arch::arch_init, platform::platform_init};
use crate::gfx::terminal;
pub mod mm;
pub mod gfx;
pub mod arch;
pub mod platform;

#[no_mangle]
unsafe extern "C" fn _start() -> ! {
    println!("Object Kernel");
    arch_init();
    platform_init();
    loop {}
}

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