#[derive(Debug)]
pub enum KernelError {
    AllocatePage,
    MapTo,
    TranslatePage,
    CreateAddressSpace,
    DestroyAddressSpace,
    LoadProcessImage,
    LoadModules,
    SerialInit
}