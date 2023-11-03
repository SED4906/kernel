#[derive(Debug)]
pub enum KernelError {
    AllocatePage,
    MapTo,
    TranslatePage,
    CreateAddressSpace,
    DestroyAddressSpace,
    LoadProcessImage,
    LoadDrivers,
    SerialInit
}