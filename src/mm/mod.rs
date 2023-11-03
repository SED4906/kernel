use self::{pmm::pmm_init, vmm::vmm_init};

pub mod pmm;
pub mod vmm;

pub unsafe fn mm_init() {
    pmm_init();
    vmm_init();
}