use core::arch::asm;

use x86_64::VirtAddr;

use crate::{mm::{vmm::{create_address_space, copy_image_into_other_address_space, map_to}, pmm::Freelist}, error::KernelError, cpu::descriptors::TSS};

pub struct Process {
    pub registers: [u64;16],
    pub instruction_pointer: u64,
    pub address_space: u64,
}

pub struct ProcessList {
    proc: *mut Process,
    next: Option<*mut ProcessList>,
    prev: Option<*mut ProcessList>,
}

const PROCESS_INFO_PAGE: u64 = 0x40000;

pub static mut CURRENT_PROCESS: Option<*mut Process> = None;
pub static mut PROCESS_LIST: Option<*mut ProcessList> = None;

pub unsafe fn create_process(image: &[u8]) -> Result<(), KernelError> {
    let process = create_process_setup(image)?;
    let process_list = Freelist::allocate::<ProcessList>()?;
    (*process_list).proc = process;
    if CURRENT_PROCESS.is_none() {
        CURRENT_PROCESS = Some(process);
        (*process_list).next = Some(process_list);
        (*process_list).prev = Some(process_list);
        PROCESS_LIST = Some(process_list);
    } else {
        (*process_list).next = PROCESS_LIST;
        (*process_list).prev = (*PROCESS_LIST.unwrap()).prev;
        (*(*PROCESS_LIST.unwrap()).prev.unwrap()).next = Some(process_list);
        (*PROCESS_LIST.unwrap()).prev = Some(process_list);
    }
    Ok(())
}

pub unsafe fn create_process_setup(image: &[u8]) -> Result<*mut Process, KernelError> {
    let address_space = create_address_space()?;
    let entry_point = load_process_image(address_space, image)?;
    let process = Freelist::allocate::<Process>()?;
    *process = Process {
        registers: [0,0,0,0,0,0,0,STACK_TOP,0,0,0,0,0,0,0,0],
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

#[no_mangle]
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
        "pop rax",
        "or rax, (1<<9)",
        "push rax",
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
        "mov dx, 0x20",
        "mov al, 0x20",
        "out dx, al",
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
pub unsafe extern "C" fn save_process_register(which: usize, value: u64) {
    let current_process = PROCESS_INFO_PAGE as *mut Process;
    (*current_process).registers[which] = value;
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

#[no_mangle]
pub unsafe extern "C" fn save_process_instruction_pointer(value: u64) {
    let current_process = PROCESS_INFO_PAGE as *mut Process;
    (*current_process).instruction_pointer = value;
}

#[no_mangle]
unsafe extern "C" fn go_next_process() {
    let process_list = PROCESS_LIST.unwrap();
    PROCESS_LIST = (*process_list).next;
    CURRENT_PROCESS = Some((*PROCESS_LIST.unwrap()).proc);
}