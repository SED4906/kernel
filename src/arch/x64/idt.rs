use x86_64::VirtAddr;
use x86_64::structures::idt::InterruptDescriptorTable;
use x86_64::structures::DescriptorTablePointer;
use x86_64::instructions::tables::lidt;

use super::exceptions::load_exception_entries;

pub static mut IDT: InterruptDescriptorTable = InterruptDescriptorTable::new();
static mut IDTR: DescriptorTablePointer = DescriptorTablePointer {limit: 4095, base: VirtAddr::zero()};

pub unsafe fn idt_init() {
    load_exception_entries();
    IDTR.base = VirtAddr::from_ptr(core::ptr::from_ref(&IDT));
    lidt(&IDTR);
}