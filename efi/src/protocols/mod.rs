use {
    crate::{guid::EfiGuid, VoidPtr},
    core::fmt::Debug,
};

pub mod console;
pub mod device_path;
pub mod media;
pub mod network;

pub trait EfiProtocol {
    type Parsed;
    type Error: Debug;

    fn guid() -> EfiGuid;

    unsafe fn parse(
        ptr: VoidPtr,
    ) -> Result<<Self as EfiProtocol>::Parsed, <Self as EfiProtocol>::Error>;
}
