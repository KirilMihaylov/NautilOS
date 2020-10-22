#![no_std]
#![doc(html_no_source)]
#![forbid(warnings)]

mod efi_object;
pub use efi_object::EfiObject;

pub mod traits;
pub mod types;
