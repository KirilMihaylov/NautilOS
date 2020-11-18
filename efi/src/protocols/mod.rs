use {
    crate::{guid::EfiGuid, types::NonNullVoidPtr},
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

    /// Used for properly and safely obtaining a handle to the requested protocol.
    /// # Safety
    /// The pointer must be non-null pointer and must point to a valid instance of the given protocol's interface.
    unsafe fn parse(
        ptr: NonNullVoidPtr,
    ) -> Result<<Self as EfiProtocol>::Parsed, <Self as EfiProtocol>::Error>;
}

pub trait ParseResult
where Self: EfiProtocol {
    type Result;
}

impl<T> ParseResult for T
where T: EfiProtocol + ?Sized {
    type Result = Result<<Self as EfiProtocol>::Parsed, <Self as EfiProtocol>::Error>;
}
