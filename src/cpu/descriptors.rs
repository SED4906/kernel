use core::arch::asm;
use x86_64::{structures::{gdt::{GlobalDescriptorTable, Descriptor, SegmentSelector}, DescriptorTablePointer, tss::TaskStateSegment, idt::InterruptDescriptorTable}, instructions::tables::load_tss, VirtAddr};
use super::exceptions::load_exception_entries;

pub static mut TSS: TaskStateSegment = TaskStateSegment::new();
pub static mut GDT: GlobalDescriptorTable = GlobalDescriptorTable::new();
pub static mut GDTR: DescriptorTablePointer = DescriptorTablePointer {limit: 63, base: VirtAddr::zero()};
pub static mut IDT: InterruptDescriptorTable = InterruptDescriptorTable::new();
static mut IDTR: DescriptorTablePointer = DescriptorTablePointer {limit: 4095, base: VirtAddr::zero()};

pub unsafe fn desc_init() {
    // GDT, TSS
    GDTR.base = VirtAddr::from_ptr(core::ptr::from_ref(&GDT));
    GDT.add_entry(Descriptor::kernel_code_segment());
    GDT.add_entry(Descriptor::kernel_data_segment());
    GDT.add_entry(Descriptor::user_code_segment());
    GDT.add_entry(Descriptor::user_data_segment());
    GDT.add_entry(Descriptor::tss_segment(&TSS));
    GDT.load();
    asm!(
        "push 0x08",
        "lea rax, [rip+2f]",
        "push rax",
        "retfq",
        "2:",
        "mov ax, 0x10",
        "mov ss, ax");
    load_tss(SegmentSelector::new(5, x86_64::PrivilegeLevel::Ring0));
    // IDT
    load_exception_entries();
    IDTR.base = VirtAddr::from_ptr(core::ptr::from_ref(&IDT));
    IDT.load();
}