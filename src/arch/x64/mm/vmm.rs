use super::pmm::Freelist;

pub unsafe fn map_to(page: u64, frame: u64, flags: u64) -> Option<u64> {
    let pml4e = ((page >> 39) & 0x1FF) as usize;
    let pml3e = ((page >> 30) & 0x1FF) as usize;
    let pml2e = ((page >> 21) & 0x1FF) as usize;
    let pml1e = ((page >> 12) & 0x1FF) as usize;
    let pml4 = page as *mut [u64;512];
    let pml3 = map_to_step(pml4, pml4e)?;
    let pml2 = map_to_step(pml3, pml3e)?;
    let pml1 = map_to_step(pml2, pml2e)?;
    (&mut *pml1)[pml1e] = frame | flags;
    Some(frame|flags)
}

unsafe fn map_to_step(pmlx: *mut [u64;512], entry: usize) -> Option<*mut [u64;512]> {
    if (&*pmlx)[entry] & 1 == 0 {
        let allocation = Freelist::allocate::<[u64;512]>()?;
        (&mut *allocation).fill(0);
        (&mut *pmlx)[entry] = allocation as u64 & 7;
    }
    return Some(((&*pmlx)[entry] & !0xFFF) as *mut [u64;512]);
}