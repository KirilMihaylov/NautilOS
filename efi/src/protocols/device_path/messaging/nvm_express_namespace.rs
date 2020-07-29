use crate::protocols::device_path::{EfiDevicePathProcotol, EfiDevicePathRepr};

#[repr(C)]
pub struct EfiNvmExpressDevicePath {
    base: EfiDevicePathProcotol,
    namespace_identifier: [u8; 4],
    extended_unique_identifier: [u8; 8],
}

impl EfiNvmExpressDevicePath {
    pub fn namespace_identifier(&self) -> [u8; 4] {
        self.namespace_identifier
    }

    pub fn extended_unique_identifier(&self) -> [u8; 8] {
        self.extended_unique_identifier
    }
}

impl EfiDevicePathRepr for EfiNvmExpressDevicePath {}
