use crate::protocols::device_path::{EfiDevicePathProcotol, EfiDevicePathRepr};

#[repr(C)]
pub struct EfiUniversalFlashStorageDevicePath {
    base: EfiDevicePathProcotol,
    target_id: u8,
    logical_unit_number: u8,
}

impl EfiUniversalFlashStorageDevicePath {
    pub fn target_id(&self) -> u8 {
        self.target_id
    }

    pub fn logical_unit_number(&self) -> u8 {
        self.logical_unit_number
    }
}

impl EfiDevicePathRepr for EfiUniversalFlashStorageDevicePath {}
