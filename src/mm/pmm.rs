static mut FREELIST: Option<*mut Freelist> = None;
pub struct Freelist {
    next: Option<*mut Freelist>
}

impl Freelist {
    pub unsafe fn free<T>(page: *mut T) {
        if page as usize & 0xFFF == 0 {
            (*page.cast::<Freelist>()).next = FREELIST;
            FREELIST = Some(page.cast());
        }
    }

    pub unsafe fn allocate<T>() -> Option<*mut T> {
        let freelist = FREELIST?;
        FREELIST = (*freelist).next;
        return Some(freelist.cast());
    }
}