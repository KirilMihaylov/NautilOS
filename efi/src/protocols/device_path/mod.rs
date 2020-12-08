use crate::{
    guid::EfiGuid, guids::EFI_DEVICE_PATH_PROTOCOL, protocols::EfiProtocol, types::NonNullVoidPtr,
};

#[repr(transparent)]
pub struct EfiDevicePathProtocolRaw {
    pointer: NonNullVoidPtr,
}

impl EfiDevicePathProtocolRaw {
    pub const fn new(pointer: NonNullVoidPtr) -> Self {
        Self { pointer }
    }
}

impl EfiProtocol for EfiDevicePathProtocolRaw {
    type Parsed = Self;
    type Error = !;

    fn guid() -> EfiGuid {
        EFI_DEVICE_PATH_PROTOCOL
    }

    unsafe fn parse(
        pointer: NonNullVoidPtr,
    ) -> Result<<Self as EfiProtocol>::Parsed, <Self as EfiProtocol>::Error> {
        Ok(Self::new(pointer))
    }
}
