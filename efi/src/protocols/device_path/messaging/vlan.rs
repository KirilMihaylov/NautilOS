use crate::protocols::device_path::{EfiDevicePathProcotol, EfiDevicePathRepr};

#[repr(C)]
pub struct EfiVlanDevicePath {
    base: EfiDevicePathProcotol,
    vlan_id: [u8; 2],
}

impl EfiVlanDevicePath {
    pub fn vlan_id(&self) -> u16 {
        unsafe { (self.vlan_id.as_ptr() as *const u16).read_unaligned() }
    }
}

impl EfiDevicePathRepr for EfiVlanDevicePath {}
