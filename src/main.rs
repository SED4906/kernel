#![feature(abi_x86_interrupt)]
#![feature(panic_info_message)]
#![feature(ptr_from_ref)]
#![feature(slice_pattern)]
#![no_std]
#![no_main]

pub mod mm;
pub mod framebuffer;
pub mod terminal;
pub mod interrupts;

use crate::interrupts::idt::idt_init;
use crate::mm::pmm::pmm_init;

#[no_mangle]
unsafe extern "C" fn _start() -> ! {
    println!("Object Kernel");
    idt_init();
    print!("I");
    pmm_init();
    print!("M");
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