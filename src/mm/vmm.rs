use x86_64::registers::control::Cr3;

use crate::{error::KernelError, serial_println};

use super::pmm::{Freelist, self};

#[no_mangle]
pub static mut KERNEL_PAGEMAP: u64 = 0;

pub fn hhdm<T>(ptr: *const T) -> *const T {
    ((ptr as usize) + unsafe{pmm::HHDM}) as *const T
}

pub fn hhdm_mut<T>(ptr: *mut T) -> *mut T {
    ((ptr as usize) + unsafe{pmm::HHDM}) as *mut T
}

pub unsafe fn map_to(pagemap: u64, page: u64, frame: u64, flags: u64) -> Result<u64, KernelError> {
    let pml4e = ((page >> 39) & 0x1FF) as usize;
    let pml3e = ((page >> 30) & 0x1FF) as usize;
    let pml2e = ((page >> 21) & 0x1FF) as usize;
    let pml1e = ((page >> 12) & 0x1FF) as usize;
    let pml4 = pagemap as *mut [u64;512];
    let pml3 = map_to_step(pml4, pml4e)?;
    let pml2 = map_to_step(pml3, pml3e)?;
    let pml1 = map_to_step(pml2, pml2e)?;
    (&mut *pml1)[pml1e] = frame | flags;
    Ok(frame|flags)
}

unsafe fn map_to_step(pmlx: *mut [u64;512], entry: usize) -> Result<*mut [u64;512], KernelError> {
    if pmlx.is_null() {
        return Err(KernelError::MapTo);
    }
    if (&*pmlx)[entry] & 1 == 0 {
        // need page
        let allocation = Freelist::allocate::<[u64;512]>()?;
        (&mut *allocation).fill(0);
        (&mut *pmlx)[entry] = allocation as u64 | 7;
    }
    Ok(((&*pmlx)[entry] & !0xFFF) as *mut [u64;512])
}

pub unsafe fn translate_page(pagemap: u64, page: u64) -> Result<u64, KernelError> {
    let pml4e = ((page >> 39) & 0x1FF) as usize;
    let pml3e = ((page >> 30) & 0x1FF) as usize;
    let pml2e = ((page >> 21) & 0x1FF) as usize;
    let pml1e = ((page >> 12) & 0x1FF) as usize;
    let pml4 = pagemap as *mut [u64;512];
    let pml3 = translate_page_step(pml4, pml4e)?;
    let pml2 = translate_page_step(pml3, pml3e)?;
    let pml1 = translate_page_step(pml2, pml2e)?;
    Ok((&mut *pml1)[pml1e])
}

unsafe fn translate_page_step(pmlx: *mut [u64;512], entry: usize) -> Result<*mut [u64;512], KernelError> {
    serial_println!("pmlx{:x}",(&*pmlx)[entry]);
    if (&*pmlx)[entry] & 1 == 0 {
        Err(KernelError::TranslatePage)
    } else {
        Ok(((&*pmlx)[entry] & !0xFFF) as *mut [u64;512])
    }
}

pub unsafe fn create_address_space() -> Result<u64, KernelError> {
    let current = current_pagemap();    
    let new_pagemap = Freelist::allocate::<[u64;512]>()?;
    (*new_pagemap).fill(0);
    (*new_pagemap)[256] = (*(current as *mut [u64;512]))[256]; // HHDM
    (*new_pagemap)[511] = (*(current as *mut [u64;512]))[511]; // Kernel
    Ok(new_pagemap as u64)
}

pub unsafe fn current_pagemap() -> u64 {
    Cr3::read().0.start_address().as_u64()
}

pub unsafe fn copy_image_into_address_space(address_space: u64, image: &[u8], dest: usize) -> Result<(), KernelError> {
    let mut num = 0;
    let mut frame = core::ptr::null_mut();
    for byte in image {
        if num == 0 || (num + dest as u64) & 0xFFF == 0 {
            frame = Freelist::allocate::<u8>()?;
            map_to(address_space, (dest as u64 & !0xFFF) + num, frame as u64, 7)?;
        }
        *frame.add(num as usize + (dest & 0xFFF)) = *byte;
        num += 1;
    }
    Ok(())
}

pub unsafe fn vmm_init() {
    KERNEL_PAGEMAP = current_pagemap();
}