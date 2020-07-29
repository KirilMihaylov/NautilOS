use core::mem::size_of;

use crate::protocols::device_path::{EfiDevicePathProcotol, EfiDevicePathRepr};

use crate::*;

#[repr(C)]
pub struct EfiBaseboardManagementControllerDevicePath {
    base: EfiDevicePathProcotol,
    interface_type: u8,
    base_address: [u8; size_of::<EfiPhysicalAddress>()],
}

impl EfiBaseboardManagementControllerDevicePath {
    pub fn interface_type(&self) -> u8 {
        self.interface_type
    }

    pub fn base_address(&self) -> EfiPhysicalAddress {
        unsafe { (self.base_address.as_ptr() as *const EfiPhysicalAddress).read_unaligned() }
    }
}

impl EfiDevicePathRepr for EfiBaseboardManagementControllerDevicePath {}
