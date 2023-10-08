use x86_64::structures::idt::InterruptStackFrame;

use super::exceptions::REGISTERED_HANDLERS;

pub unsafe fn register_handler(isr: u8, handler: *const fn(InterruptStackFrame, Option<u64>)) {
    REGISTERED_HANDLERS[isr as usize] = Some(handler as u64)
}