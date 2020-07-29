use core::slice::from_raw_parts;

use crate::protocols::device_path::{EfiDevicePathProcotol, EfiDevicePathRepr};

#[repr(C)]
pub struct EfiFilePathDevicePath {
    base: EfiDevicePathProcotol,
    path_name: (),
}

impl EfiFilePathDevicePath {
    pub fn path_name(&self) -> &[u8] {
        unsafe {
            from_raw_parts(
                &self.path_name as *const () as *const u8,
                self.base.len() as usize - 4,
            )
        }
    }
}

impl EfiDevicePathRepr for EfiFilePathDevicePath {}
