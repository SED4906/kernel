static MEMMAP_REQUEST: limine::MemmapRequest = limine::MemmapRequest::new(0);
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

pub unsafe fn pmm_init() {
    if let Some(memmap_request) = MEMMAP_REQUEST.get_response().get() {
        for entry in memmap_request.memmap() {
            if entry.typ != limine::MemoryMapEntryType::Usable {continue}
            let mut page = entry.base;
            while page < entry.base + entry.len {
                Freelist::free(page as *mut Freelist);
                page += 4096;
            }
        }
    }
}