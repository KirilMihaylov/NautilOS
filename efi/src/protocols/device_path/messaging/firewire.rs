use crate::protocols::device_path::{EfiDevicePathProcotol, EfiDevicePathRepr};

#[repr(C)]
pub struct EfiFirewireDevicePath {
    base: EfiDevicePathProcotol,
    firewire_guid: [u8; 8],
}

impl EfiFirewireDevicePath {
    pub fn firewire_guid(&self) -> u64 {
        unsafe { (self.firewire_guid.as_ptr() as *const u64).read_unaligned() }
    }
}

impl EfiDevicePathRepr for EfiFirewireDevicePath {}
