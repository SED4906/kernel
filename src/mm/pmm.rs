use crate::{error::KernelError, serial_println, println};

static mut FREELIST: Option<*mut Freelist> = None;
pub struct Freelist {
    next: Option<*mut Freelist>
}

impl Freelist {
    pub unsafe fn free<T>(page: *mut T) {
        if page.is_aligned_to(4096) {
            (*page.cast::<Freelist>()).next = FREELIST;
            FREELIST = Some(page.cast());
        }
    }

    pub unsafe fn allocate<T>() -> Result<*mut T, KernelError> {
        if let Some(freelist) = FREELIST {
            FREELIST = (*freelist).next;
            Ok(freelist.cast())
        } else {
            Err(KernelError::AllocatePage)
        }
    }
}

static MEMMAP_REQUEST: limine::MemmapRequest = limine::MemmapRequest::new(0);

const ENTRY_TYPE_NAMES: [&str;8] = ["Usable", "Reserved", "ACPI (Reclaimable)", "ACPI (NVS)", "Bad Memory", "Bootloader (Reclaimable)", "Kernel & Modules", "Framebuffer"];

pub unsafe fn pmm_init() {
    if let Some(memmap_request) = MEMMAP_REQUEST.get_response().get() {
        let mut usable_pages = 0;
        for entry in memmap_request.memmap() {
            serial_println!("{:x}->{:x} {}", entry.base, entry.base+entry.len, ENTRY_TYPE_NAMES[entry.typ as usize]);
            if entry.typ != limine::MemoryMapEntryType::Usable {continue}
            let mut page = entry.base;
            usable_pages += entry.len >> 12;
            while page < entry.base + entry.len {
                Freelist::free(page as *mut Freelist);
                page += 4096;
            }
        }
        println!("Usable memory: {usable_pages} pages ({} MiB)", usable_pages / 256);
    }
}