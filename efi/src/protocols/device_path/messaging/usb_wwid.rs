use crate::protocols::device_path::{EfiDevicePathProcotol, EfiDevicePathRepr};

#[repr(C)]
pub struct EfiUsbWwidDevicePath {
    base: EfiDevicePathProcotol,
    parent_port_number: u8,
    interface_number: u8,
}

impl EfiUsbWwidDevicePath {
    pub fn parent_port_number(&self) -> u8 {
        self.parent_port_number
    }

    pub fn interface_number(&self) -> u8 {
        self.interface_number
    }
}

impl EfiDevicePathRepr for EfiUsbWwidDevicePath {}
