use core::slice::from_raw_parts;

use crate::protocols::device_path::{EfiDevicePathProcotol, EfiDevicePathRepr};

#[repr(C)]
pub struct EfiBiosBootSpecification_1_01_DevicePath {
    base: EfiDevicePathProcotol,
    device_type: [u8; 2],
    status_flag: [u8; 2],
    description_string: [u8; 1],
}

impl EfiBiosBootSpecification_1_01_DevicePath {
    pub fn device_type(&self) -> u16 {
        unsafe { (self.device_type.as_ptr() as *const u16).read_unaligned() }
    }

    pub fn status_flag(&self) -> u16 {
        unsafe { (self.status_flag.as_ptr() as *const u16).read_unaligned() }
    }

    pub fn description_string<'a>(&self) -> &'a [u8] {
        unsafe {
            from_raw_parts(
                self.description_string.as_ptr(),
                self.base.len() as usize - 8,
            )
        }
    }
}

impl EfiDevicePathRepr for EfiBiosBootSpecification_1_01_DevicePath {}
