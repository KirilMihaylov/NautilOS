use core::mem::size_of;

use crate::{
    boot_services::memory::EfiMemoryType,
    protocols::device_path::{EfiDevicePathProcotol, EfiDevicePathRepr},
    *,
};

#[repr(C)]
pub struct EfiMemoryMappedDevicePath {
    base: EfiDevicePathProcotol,
    memory_type: [u8; size_of::<EfiMemoryType>()],
    start_address: [u8; size_of::<EfiPhysicalAddress>()],
    end_address: [u8; size_of::<EfiPhysicalAddress>()],
}

impl EfiMemoryMappedDevicePath {
    pub fn memory_type(&self) -> EfiMemoryType {
        unsafe { (self.memory_type.as_ptr() as *const EfiMemoryType).read_unaligned() }
    }

    pub fn start_address(&self) -> EfiPhysicalAddress {
        unsafe { (self.start_address.as_ptr() as *const EfiPhysicalAddress).read_unaligned() }
    }

    pub fn end_address(&self) -> EfiPhysicalAddress {
        unsafe { (self.end_address.as_ptr() as *const EfiPhysicalAddress).read_unaligned() }
    }
}

impl EfiDevicePathRepr for EfiMemoryMappedDevicePath {}
