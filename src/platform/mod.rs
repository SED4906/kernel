#[cfg(target_arch="x86_64")]
#[cfg(platform_pc)]
pub mod pc;
#[cfg(target_arch="x86_64")]
#[cfg(platform_pc)]
pub use pc::*;