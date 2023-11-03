use x86_64::registers::control::Cr3;

use crate::{error::KernelError, serial, serial_println, println};

use super::pmm::Freelist;

#[no_mangle]
pub static mut KERNEL_PAGEMAP: u64 = 0;

pub unsafe fn map_to(pagemap: u64, page: u64, frame: u64, flags: u64) -> Result<u64, KernelError> {
    //serial_println!("map_to in {pagemap:x} {page:x} {frame:x} {flags:x}");
    let pml4e = ((page >> 39) & 0x1FF) as usize;
    let pml3e = ((page >> 30) & 0x1FF) as usize;
    let pml2e = ((page >> 21) & 0x1FF) as usize;
    let pml1e = ((page >> 12) & 0x1FF) as usize;
    let pml4 = pagemap as *mut [u64;512];
    //serial_println!("pml4: {:x}", pml4 as usize);
    let pml3 = map_to_step(pml4, pml4e)?;
    //serial_println!("pml3: {:x}", pml3 as usize);
    let pml2 = map_to_step(pml3, pml3e)?;
    //serial_println!("pml2: {:x}", pml2 as usize);
    let pml1 = map_to_step(pml2, pml2e)?;
    //serial_println!("pml1: {:x}", pml1 as usize);
    (&mut *pml1)[pml1e] = frame | flags;
    Ok(frame|flags)
}

unsafe fn map_to_step(pmlx: *mut [u64;512], entry: usize) -> Result<*mut [u64;512], KernelError> {
    if pmlx.is_null() {
        return Err(KernelError::MapTo);
    }
    if (&*pmlx)[entry] & 1 == 0 {
        //serial_println!("need page");
        let allocation = Freelist::allocate::<[u64;512]>()?;
        (&mut *allocation).fill(0);
        (&mut *pmlx)[entry] = allocation as u64 | 7;
    }
    Ok(((&*pmlx)[entry] & !0xFFF) as *mut [u64;512])
}

pub unsafe fn translate_page(pagemap: u64, page: u64) -> Result<u64, KernelError> {
    //serial_println!("translate_page in {pagemap:x}");
    let pml4e = ((page >> 39) & 0x1FF) as usize;
    let pml3e = ((page >> 30) & 0x1FF) as usize;
    let pml2e = ((page >> 21) & 0x1FF) as usize;
    let pml1e = ((page >> 12) & 0x1FF) as usize;
    let pml4 = pagemap as *mut [u64;512];
    //serial_println!("pml3: {:x}", pml4 as usize);
    let pml3 = translate_page_step(pml4, pml4e)?;
    //serial_println!("pml3: {:x}", pml3 as usize);
    let pml2 = translate_page_step(pml3, pml3e)?;
    //serial_println!("pml2: {:x}", pml2 as usize);
    let pml1 = translate_page_step(pml2, pml2e)?;
    //serial_println!("pml1: {:x}", pml1 as usize);
    Ok((&mut *pml1)[pml1e])
}

unsafe fn translate_page_step(pmlx: *mut [u64;512], entry: usize) -> Result<*mut [u64;512], KernelError> {
    if (&*pmlx)[entry] & 1 == 0 {
        Err(KernelError::TranslatePage)
    } else {
        Ok(((&*pmlx)[entry] & !0xFFF) as *mut [u64;512])
    }
}

const GC_VMEM_BASE: u64 = 1<<39;

pub unsafe fn create_address_space() -> Result<u64, KernelError> {
    let current = current_pagemap();    
    let new_pagemap = Freelist::allocate::<[u64;512]>()?;
    (*new_pagemap).fill(0);
    (*new_pagemap)[256] = (*(current as *mut [u64;512]))[256];
    (*new_pagemap)[511] = (*(current as *mut [u64;512]))[511];
    let gc_page = Freelist::allocate::<u64>()?;
    *gc_page = 0;
    map_to(new_pagemap as u64, GC_VMEM_BASE, gc_page as u64, 3)?;
    Ok(new_pagemap as u64)
}

pub unsafe fn destroy_address_space(pagemap: u64) -> Result<(), KernelError> {
    let mut gc_page = translate_page(pagemap, GC_VMEM_BASE)? & !0xFFF;
    let pages =*(gc_page as *mut u64);
    for page_number in 1..=pages {
        if page_number & 511 == 0 {
            Freelist::free(gc_page as *mut u64);
            gc_page = translate_page(pagemap, GC_VMEM_BASE + page_number*8)? & !0xFFF;
        }
        Freelist::free(((*(gc_page as *mut [u64;512]))[page_number as usize]) as *mut u64);
    }
    Ok(())
}

pub unsafe fn current_pagemap() -> u64 {
    Cr3::read().0.start_address().as_u64()
}

pub unsafe fn copy_image_into_other_address_space(address_space: u64, image: &[u8], dest: *mut u8) -> Result<(), KernelError> {
    let mut num = 0;
    let mut frame = core::ptr::null_mut();
    for byte in image {
        if num == 0 || (num + dest as u64) & 0xFFF == 0 {
            frame = Freelist::allocate::<u8>()?;
            map_to(address_space, (dest as u64 & !0xFFF) + num, frame as u64, 7)?;
        }
        *frame.add(num as usize + (dest as usize & 0xFFF)) = *byte;
        num += 1;
    }
    Ok(())
}

pub unsafe fn vmm_init() {
    KERNEL_PAGEMAP = current_pagemap();
}