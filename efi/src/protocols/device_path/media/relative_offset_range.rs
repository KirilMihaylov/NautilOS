use crate::protocols::device_path::{EfiDevicePathProcotol, EfiDevicePathRepr};

#[repr(C)]
pub struct EfiRelativeOffsetRangeDevicePath {
    base: EfiDevicePathProcotol,
    _reserved: [u8; 4],
    starting_offset: [u8; 8],
    ending_offset: [u8; 8],
}

impl EfiRelativeOffsetRangeDevicePath {
    pub fn starting_offset(&self) -> u64 {
        unsafe { (self.starting_offset.as_ptr() as *const u64).read_unaligned() }
    }

    pub fn ending_offset(&self) -> u64 {
        unsafe { (self.ending_offset.as_ptr() as *const u64).read_unaligned() }
    }
}

impl EfiDevicePathRepr for EfiRelativeOffsetRangeDevicePath {}
