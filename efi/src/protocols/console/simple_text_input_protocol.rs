use crate::{
    guid::EfiGuid,
    protocols::EfiProtocol,
    status::{EfiStatus, EfiStatusEnum},
    types::EfiEvent,
    VoidPtr,
};

#[repr(C)]
pub struct EfiSimpleTextInputProtocol {
    reset: extern "efiapi" fn(*const Self, extended_verification: bool) -> EfiStatus,
    read_key_stroke: extern "efiapi" fn(*const Self, key: *mut EfiInputKey) -> EfiStatus,
    wait_for_key: EfiEvent,
}

impl EfiSimpleTextInputProtocol {
    pub fn reset(&self, extended_verification: bool) -> EfiStatusEnum {
        (self.reset)(self, extended_verification).into_enum()
    }

    pub fn read_key_stroke(&self) -> EfiStatusEnum<EfiInputKey> {
        let mut key: EfiInputKey = EfiInputKey {
            scan_code: 0,
            unicode_char: 0,
        };

        (self.read_key_stroke)(self, &mut key).into_enum_data(|| key)
    }

    pub fn wait_for_key(&self) -> &EfiEvent {
        &self.wait_for_key
    }

    pub fn wait_for_key_mut(&mut self) -> &mut EfiEvent {
        &mut self.wait_for_key
    }
}

impl EfiProtocol for EfiSimpleTextInputProtocol {
    type Parsed = &'static Self;
    type Error = !;

    fn guid() -> EfiGuid {
        crate::guids::EFI_SIMPLE_TEXT_INPUT_PROTOCOL
    }

    unsafe fn parse(ptr: VoidPtr) -> Result<<Self as EfiProtocol>::Parsed, <Self as EfiProtocol>::Error> {
        Ok(&*(ptr as *const Self))
    }
}

#[repr(C)]
pub struct EfiInputKey {
    scan_code: u16,
    unicode_char: u16,
}
