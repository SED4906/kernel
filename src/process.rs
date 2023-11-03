use core::arch::asm;

use x86_64::VirtAddr;

use crate::{mm::{vmm::{create_address_space, copy_image_into_other_address_space, map_to}, pmm::Freelist}, error::KernelError, cpu::descriptors::TSS};

pub struct Process {
    pub registers: [u64;16],
    pub instruction_pointer: u64,
    pub address_space: u64,
}

const PROCESS_INFO_PAGE: u64 = 0x40000;
const PROCESS_RECVBOX_PAGE: u64 = 0x10000;
const PROCESS_SENDBOX_PAGE: u64 = 0x18000;

pub static mut CURRENT_PROCESS: Option<*mut Process> = None;
pub static mut PROCESS_LIST: Option<*mut Process> = None;

pub unsafe fn create_process(image: &[u8]) -> Result<(), KernelError> {
    let process = create_process_setup(image)?;
    if CURRENT_PROCESS.is_none() {
        CURRENT_PROCESS = Some(process);
    }
    Ok(())
}

pub unsafe fn create_process_setup(image: &[u8]) -> Result<*mut Process, KernelError> {
    let address_space = create_address_space()?;
    let entry_point = load_process_image(address_space, image)?;
    let process = Freelist::allocate::<Process>()?;
    *process = Process {
        registers: [0;16],
        instruction_pointer: entry_point,
        address_space,
    };
    map_to(address_space, PROCESS_INFO_PAGE, process as u64, 3)?;
    for i in 0..STACK_PAGES {
        map_to(address_space, STACK_TOP - i * 0x1000, Freelist::allocate::<u64>()? as u64, 7)?;
    }
    Ok(process)
}

const ENTRY_POINT: u64 = 0x80000;
const STACK_TOP: u64 = (1<<39) - 0x1000;
const STACK_PAGES: u64 = 8;

pub unsafe fn load_process_image(address_space: u64, image: &[u8]) -> Result<u64, KernelError> {
    let entry_point = ENTRY_POINT;
    copy_image_into_other_address_space(address_space, image, entry_point as *mut u8)?;
    Ok(entry_point)
}

#[no_mangle]
unsafe extern "C" fn save_stack_to_tss(stack_pointer: u64) {
    TSS.privilege_stack_table[0] = VirtAddr::new(stack_pointer);
}

#[naked]
pub unsafe extern "C" fn switch_to() {
    asm!(
        "mov rdi, rsp",
        "call save_stack_to_tss",
        "push 0x23",
        "mov rdi, 7",
        "call get_process_register",
        "push rax",
        "pushfq",
        "push 0x1b",
        "call get_process_instruction_pointer",
        "push rax",
        "mov rdi, 0",
        "call get_process_register",
        "push rax",
        "mov rdi, 1",
        "call get_process_register",
        "push rax",
        "mov rdi, 2",
        "call get_process_register",
        "push rax",
        "mov rdi, 3",
        "call get_process_register",
        "push rax",
        "mov rdi, 4",
        "call get_process_register",
        "push rax",
        "mov rdi, 5",
        "call get_process_register",
        "push rax",
        "mov rdi, 6",
        "call get_process_register",
        "push rax",
        "mov rdi, 8",
        "call get_process_register",
        "push rax",
        "mov rdi, 9",
        "call get_process_register",
        "push rax",
        "mov rdi, 10",
        "call get_process_register",
        "push rax",
        "mov rdi, 11",
        "call get_process_register",
        "push rax",
        "mov rdi, 12",
        "call get_process_register",
        "push rax",
        "mov rdi, 13",
        "call get_process_register",
        "push rax",
        "mov rdi, 14",
        "call get_process_register",
        "push rax",
        "mov rdi, 15",
        "call get_process_register",
        "push rax",
        "call get_process_address_space",
        "mov cr3, rax",
        "pop r15",
        "pop r14",
        "pop r13",
        "pop r12",
        "pop r11",
        "pop r10",
        "pop r9",
        "pop r8",
        "pop rbp",
        "pop rdi",
        "pop rsi",
        "pop rdx",
        "pop rcx",
        "pop rbx",
        "pop rax",
        "iretq",
        options(noreturn))
}

#[no_mangle]
pub unsafe extern "C" fn get_process_register(which: usize) -> u64 {
    let current_process = CURRENT_PROCESS.expect("no current process to get register from");
    (*current_process).registers[which]
}

#[no_mangle]
pub unsafe extern "C" fn get_process_address_space() -> u64 {
    let current_process = CURRENT_PROCESS.expect("no current process to get adress space from");
    (*current_process).address_space
}

#[no_mangle]
pub unsafe extern "C" fn get_process_instruction_pointer() -> u64 {
    let current_process = CURRENT_PROCESS.expect("no current process to get instruction pointer from");
    (*current_process).instruction_pointer
}