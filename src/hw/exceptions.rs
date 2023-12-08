use core::arch::asm;
use crate::mm::vmm::KERNEL_PAGEMAP;
use x86_64::structures::idt::{InterruptStackFrame, PageFaultErrorCode};
use macros::{remaining_interrupt_handlers,set_remaining_interrupt_handlers};
#[macro_export]
macro_rules! handle_interrupt {
    (no_err $n:literal) => {
        handle_interrupt_no_err!($n);
    };
    (err $n:literal) => {
        handle_interrupt_err!($n);
    };
}

macro_rules! handle_interrupt_no_err {
    ($n:literal) => {
        unsafe{asm!(
            "push rax",
            "push rbx",
            "push rcx",
            "push rdx",
            "push rsi",
            "push rdi",
            "push rbp",
            "push r8",
            "push r9",
            "push r10",
            "push r11",
            "push r12",
            "push r13",
            "push r14",
            "push r15",
            "pop rsi",
            "mov rdi, 15",
            "call save_process_register",
            "pop rsi",
            "mov rdi, 14",
            "call save_process_register",
            "pop rsi",
            "mov rdi, 13",
            "call save_process_register",
            "pop rsi",
            "mov rdi, 12",
            "call save_process_register",
            "pop rsi",
            "mov rdi, 11",
            "call save_process_register",
            "pop rsi",
            "mov rdi, 10",
            "call save_process_register",
            "pop rsi",
            "mov rdi, 9",
            "call save_process_register",
            "pop rsi",
            "mov rdi, 8",
            "call save_process_register",
            "pop rsi",
            "mov rdi, 6",
            "call save_process_register",
            "pop rsi",
            "mov rdi, 5",
            "call save_process_register",
            "pop rsi",
            "mov rdi, 4",
            "call save_process_register",
            "pop rsi",
            "mov rdi, 3",
            "call save_process_register",
            "pop rsi",
            "mov rdi, 2",
            "call save_process_register",
            "pop rsi",
            "mov rdi, 1",
            "call save_process_register",
            "pop rsi",
            "mov rdi, 0",
            "call save_process_register",
            "mov rdi, 7",
            "mov rsi, [rsp+24]",
            "call save_process_register",
            "mov rdi, [rsp]",
            "call save_process_instruction_pointer",
            "add rsp, 40",
            "mov rax, {0}",
            "mov cr3, rax",
            concat!("mov rdi, ",$n),
            "call interrupt_handler",
            "jmp switch_to",
            sym KERNEL_PAGEMAP,
            options(noreturn)
        )}
    };
}

macro_rules! handle_interrupt_err {
    ($n:literal) => {
        unsafe{asm!(
            "push rax",
            "push rbx",
            "push rcx",
            "push rdx",
            "push rsi",
            "push rdi",
            "push rbp",
            "push r8",
            "push r9",
            "push r10",
            "push r11",
            "push r12",
            "push r13",
            "push r14",
            "push r15",
            "pop rsi",
            "mov rdi, 15",
            "call save_process_register",
            "pop rsi",
            "mov rdi, 14",
            "call save_process_register",
            "pop rsi",
            "mov rdi, 13",
            "call save_process_register",
            "pop rsi",
            "mov rdi, 12",
            "call save_process_register",
            "pop rsi",
            "mov rdi, 11",
            "call save_process_register",
            "pop rsi",
            "mov rdi, 10",
            "call save_process_register",
            "pop rsi",
            "mov rdi, 9",
            "call save_process_register",
            "pop rsi",
            "mov rdi, 8",
            "call save_process_register",
            "pop rsi",
            "mov rdi, 6",
            "call save_process_register",
            "pop rsi",
            "mov rdi, 5",
            "call save_process_register",
            "pop rsi",
            "mov rdi, 4",
            "call save_process_register",
            "pop rsi",
            "mov rdi, 3",
            "call save_process_register",
            "pop rsi",
            "mov rdi, 2",
            "call save_process_register",
            "pop rsi",
            "mov rdi, 1",
            "call save_process_register",
            "pop rsi",
            "mov rdi, 0",
            "call save_process_register",
            "mov rdi, 7",
            "mov rsi, [rsp+24]",
            "call save_process_register",
            "mov rdi, [rsp]",
            "call save_process_instruction_pointer",
            "add rsp, 40",
            "mov rax, {0}",
            "mov cr3, rax",
            concat!("mov rdi, ",$n),
            "pop rsi",
            "call interrupt_handler",
            "jmp switch_to",
            sym KERNEL_PAGEMAP,
            options(noreturn)
        )}
    };
}


#[no_mangle]
pub fn interrupt_handler(which: u8, error_code: u64) {
    unsafe{asm!("mov cr3, {0}",in(reg) KERNEL_PAGEMAP);}
    match which {
        8..=15|17|18|21..=31 => {
            let mut stack_rip: u64;
            let mut stack_rsp: u64;
            unsafe{asm!("mov {0}, [rsp+8]",out(reg) stack_rip)};
            unsafe{asm!("mov {0}, [rsp+32]",out(reg) stack_rsp)};
            panic!("Unhandled interrupt #{which}: {error_code} @ rip{stack_rip:x}; rsp{stack_rsp:x}");
        }
        0..=7|16|19|20|_ => {
            let mut stack_rip: u64;
            let mut stack_rsp: u64;
            unsafe{asm!("mov {0}, [rsp+8]",out(reg) stack_rip)};
            unsafe{asm!("mov {0}, [rsp+32]",out(reg) stack_rsp)};
            panic!("Unhandled interrupt #{which} @ rip{stack_rip:x}; rsp{stack_rsp:x}");
        }
    }
}

#[naked]
extern "x86-interrupt" fn divide_error(stack: InterruptStackFrame) {
    handle_interrupt!(no_err "0");
}

#[naked]
extern "x86-interrupt" fn debug(stack: InterruptStackFrame) {
    handle_interrupt!(no_err "1");
}

#[naked]
extern "x86-interrupt" fn non_maskable_interrupt(stack: InterruptStackFrame) {
    handle_interrupt!(no_err "2");
}

#[naked]
extern "x86-interrupt" fn breakpoint(stack: InterruptStackFrame) {
    handle_interrupt!(no_err "3");
}

#[naked]
extern "x86-interrupt" fn overflow(stack: InterruptStackFrame) {
    handle_interrupt!(no_err "4");
}

#[naked]
extern "x86-interrupt" fn bound_range_exceeded(stack: InterruptStackFrame) {
    handle_interrupt!(no_err "5");
}

#[naked]
extern "x86-interrupt" fn invalid_opcode(stack: InterruptStackFrame) {
    handle_interrupt!(no_err "6");
}

#[naked]
extern "x86-interrupt" fn device_not_available(stack: InterruptStackFrame) {
    handle_interrupt!(no_err "7");
}

extern "x86-interrupt" fn double_fault(_stack: InterruptStackFrame, _error_code: u64) -> ! {
    panic!("Double fault");
}

#[naked]
extern "x86-interrupt" fn invalid_tss(stack: InterruptStackFrame, error_code: u64) {
    handle_interrupt!(err "10");
}

#[naked]
extern "x86-interrupt" fn segment_not_present(stack: InterruptStackFrame, error_code: u64) {
    handle_interrupt!(err "11");
}

#[naked]
extern "x86-interrupt" fn stack_segment_fault(stack: InterruptStackFrame, error_code: u64) {
    handle_interrupt!(err "12");
}

#[naked]
extern "x86-interrupt" fn general_protection_fault(stack: InterruptStackFrame, error_code: u64) {
    handle_interrupt!(err "13");
}

#[naked]
extern "x86-interrupt" fn page_fault(stack: InterruptStackFrame, error_code: PageFaultErrorCode) {
    handle_interrupt!(err "14");
}

#[naked]
extern "x86-interrupt" fn x87_floating_point(stack: InterruptStackFrame) {
    handle_interrupt!(no_err "16");
}

#[naked]
extern "x86-interrupt" fn alignment_check(stack: InterruptStackFrame, error_code: u64) {
    handle_interrupt!(err "17");
}

extern "x86-interrupt" fn machine_check(_stack: InterruptStackFrame) -> ! {
    panic!("Machine check");
}

#[naked]
extern "x86-interrupt" fn simd_floating_point(stack: InterruptStackFrame) {
    handle_interrupt!(no_err "19");
}

#[naked]
extern "x86-interrupt" fn virtualization(stack: InterruptStackFrame) {
    handle_interrupt!(no_err "20");
}

#[naked]
extern "x86-interrupt" fn vmm_communication_exception(stack: InterruptStackFrame, error_code: u64) {
    handle_interrupt!(err "29");
}

#[naked]
extern "x86-interrupt" fn security_exception(stack: InterruptStackFrame, error_code: u64) {
    handle_interrupt!(err "30");
}

#[naked]
extern "x86-interrupt" fn irq_timer(stack: InterruptStackFrame) {
    unsafe{asm!(
        "push rax",
        "push rbx",
        "push rcx",
        "push rdx",
        "push rsi",
        "push rdi",
        "push rbp",
        "push r8",
        "push r9",
        "push r10",
        "push r11",
        "push r12",
        "push r13",
        "push r14",
        "push r15",
        "pop rsi",
        "mov rdi, 15",
        "call save_process_register",
        "pop rsi",
        "mov rdi, 14",
        "call save_process_register",
        "pop rsi",
        "mov rdi, 13",
        "call save_process_register",
        "pop rsi",
        "mov rdi, 12",
        "call save_process_register",
        "pop rsi",
        "mov rdi, 11",
        "call save_process_register",
        "pop rsi",
        "mov rdi, 10",
        "call save_process_register",
        "pop rsi",
        "mov rdi, 9",
        "call save_process_register",
        "pop rsi",
        "mov rdi, 8",
        "call save_process_register",
        "pop rsi",
        "mov rdi, 6",
        "call save_process_register",
        "pop rsi",
        "mov rdi, 5",
        "call save_process_register",
        "pop rsi",
        "mov rdi, 4",
        "call save_process_register",
        "pop rsi",
        "mov rdi, 3",
        "call save_process_register",
        "pop rsi",
        "mov rdi, 2",
        "call save_process_register",
        "pop rsi",
        "mov rdi, 1",
        "call save_process_register",
        "pop rsi",
        "mov rdi, 0",
        "call save_process_register",
        "mov rdi, 7",
        "mov rsi, [rsp+24]",
        "call save_process_register",
        "mov rdi, [rsp]",
        "call save_process_instruction_pointer",
        "add rsp, 40",
        "mov rax, {0}",
        "mov cr3, rax",
        "call go_next_process",
        "jmp switch_to",
        sym KERNEL_PAGEMAP,
        options(noreturn)
    )}
}

remaining_interrupt_handlers!();

use super::descriptors::IDT;

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
    IDT[32].set_handler_fn(irq_timer);
    set_remaining_interrupt_handlers!();
}
