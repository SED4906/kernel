pub use crate::mm::pmm::Freelist;

static MEMMAP_REQUEST: limine::MemmapRequest = limine::MemmapRequest::new(0);

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