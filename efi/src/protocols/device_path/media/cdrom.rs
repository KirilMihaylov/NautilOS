use crate::protocols::device_path::{EfiDevicePathProcotol, EfiDevicePathRepr};

#[repr(C)]
pub struct EfiCDROMDevicePath {
    base: EfiDevicePathProcotol,
    partition_start: [u8; 8],
    partition_size: [u8; 8],
}

impl EfiCDROMDevicePath {
    pub fn partition_start(&self) -> u64 {
        unsafe { (self.partition_start.as_ptr() as *const u64).read_unaligned() }
    }

    pub fn partition_size(&self) -> u64 {
        unsafe { (self.partition_size.as_ptr() as *const u64).read_unaligned() }
    }
}

impl EfiDevicePathRepr for EfiCDROMDevicePath {}
