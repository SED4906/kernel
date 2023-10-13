pub mod idt;
pub mod exceptions;
pub mod mm;

pub unsafe fn arch_init() {
    mm::pmm::pmm_init();
    idt::idt_init();
}