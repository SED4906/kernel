use x86_64::structures::idt::{InterruptStackFrame, PageFaultErrorCode};
use crate::println;
pub static mut REGISTERED_HANDLERS: [Option<u64>;256] = [None;256];
use macros::{remaining_interrupt_handlers,set_remaining_interrupt_handlers};
#[macro_export]
macro_rules! handle_interrupt {
    (no_err $n:literal $f:expr, $s:ident) => {
        if let Some(handler) = unsafe{REGISTERED_HANDLERS}[$n] {
            unsafe{(*(handler as *const fn(InterruptStackFrame, Option<u64>)))($s, None)};
        } else {
            println!($f);
        }
    };
    (err $n:literal $f:expr, $s:ident $e:expr) => {
        if let Some(handler) = unsafe{REGISTERED_HANDLERS}[$n] {
            unsafe{(*(handler as *const fn(InterruptStackFrame, Option<u64>)))($s, Some($e))};
        } else {
            println!($f);
        }
    };
}

extern "x86-interrupt" fn divide_error(stack: InterruptStackFrame) {
    handle_interrupt!(no_err 0 "Divide error", stack);
}

extern "x86-interrupt" fn debug(stack: InterruptStackFrame) {
    handle_interrupt!(no_err 1 "Debug", stack);
}

extern "x86-interrupt" fn non_maskable_interrupt(stack: InterruptStackFrame) {
    handle_interrupt!(no_err 2 "Non-maskable interrupt", stack);
}

extern "x86-interrupt" fn breakpoint(stack: InterruptStackFrame) {
    handle_interrupt!(no_err 3 "Breakpoint", stack);
}

extern "x86-interrupt" fn overflow(stack: InterruptStackFrame) {
    handle_interrupt!(no_err 4 "Overflow", stack);
}

extern "x86-interrupt" fn bound_range_exceeded(stack: InterruptStackFrame) {
    handle_interrupt!(no_err 5 "Bound range exceeded", stack);
}

extern "x86-interrupt" fn invalid_opcode(stack: InterruptStackFrame) {
    handle_interrupt!(no_err 6 "Invalid opcode", stack);
}

extern "x86-interrupt" fn device_not_available(stack: InterruptStackFrame) {
    handle_interrupt!(no_err 7 "Device not available", stack);
}

extern "x86-interrupt" fn double_fault(_stack: InterruptStackFrame, _error_code: u64) -> ! {
    panic!("Double fault");
}

extern "x86-interrupt" fn invalid_tss(stack: InterruptStackFrame, error_code: u64) {
    handle_interrupt!(err 10 "Invalid TSS", stack error_code);
}

extern "x86-interrupt" fn segment_not_present(stack: InterruptStackFrame, error_code: u64) {
    handle_interrupt!(err 11 "Segment not present", stack error_code);
}

extern "x86-interrupt" fn stack_segment_fault(stack: InterruptStackFrame, error_code: u64) {
    handle_interrupt!(err 12 "Stack-segment fault", stack error_code);
}

extern "x86-interrupt" fn general_protection_fault(stack: InterruptStackFrame, error_code: u64) {
    handle_interrupt!(err 13 "General protection fault", stack error_code);
}

extern "x86-interrupt" fn page_fault(stack: InterruptStackFrame, error_code: PageFaultErrorCode) {
    handle_interrupt!(err 14 "Invalid TSS", stack error_code.bits());
}

extern "x86-interrupt" fn x87_floating_point(stack: InterruptStackFrame) {
    handle_interrupt!(no_err 16 "x87 Floating point", stack);
}

extern "x86-interrupt" fn alignment_check(stack: InterruptStackFrame, error_code: u64) {
    handle_interrupt!(err 17 "Alignment check", stack error_code);
}

extern "x86-interrupt" fn machine_check(_stack: InterruptStackFrame) -> ! {
    panic!("Machine check");
}

extern "x86-interrupt" fn simd_floating_point(stack: InterruptStackFrame) {
    handle_interrupt!(no_err 19 "SIMD Floating point", stack);
}

extern "x86-interrupt" fn virtualization(stack: InterruptStackFrame) {
    handle_interrupt!(no_err 20 "Virtualization", stack);
}

extern "x86-interrupt" fn vmm_communication_exception(stack: InterruptStackFrame, error_code: u64) {
    handle_interrupt!(err 29 "VMM communication exception", stack error_code);
}

extern "x86-interrupt" fn security_exception(stack: InterruptStackFrame, error_code: u64) {
    handle_interrupt!(err 30 "Security exception", stack error_code);
}

remaining_interrupt_handlers!();

use super::idt::IDT;

macro_rules! set_idt_handler_fn_eponymous {
    ($f:ident) => {
        IDT.$f.set_handler_fn($f);
    };
}

pub unsafe fn load_exception_entries() {
    set_idt_handler_fn_eponymous!(divide_error);
    set_idt_handler_fn_eponymous!(debug);
    set_idt_handler_fn_eponymous!(non_maskable_interrupt);
    set_idt_handler_fn_eponymous!(breakpoint);
    set_idt_handler_fn_eponymous!(overflow);
    set_idt_handler_fn_eponymous!(bound_range_exceeded);
    set_idt_handler_fn_eponymous!(invalid_opcode);
    set_idt_handler_fn_eponymous!(device_not_available);
    set_idt_handler_fn_eponymous!(double_fault);
    set_idt_handler_fn_eponymous!(invalid_tss);
    set_idt_handler_fn_eponymous!(segment_not_present);
    set_idt_handler_fn_eponymous!(stack_segment_fault);
    set_idt_handler_fn_eponymous!(general_protection_fault);
    set_idt_handler_fn_eponymous!(page_fault);
    set_idt_handler_fn_eponymous!(x87_floating_point);
    set_idt_handler_fn_eponymous!(alignment_check);
    set_idt_handler_fn_eponymous!(machine_check);
    set_idt_handler_fn_eponymous!(simd_floating_point);
    set_idt_handler_fn_eponymous!(virtualization);
    set_idt_handler_fn_eponymous!(vmm_communication_exception);
    set_idt_handler_fn_eponymous!(security_exception);
    set_remaining_interrupt_handlers!();
}

