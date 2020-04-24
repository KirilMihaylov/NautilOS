use crate::protocols::device_path::{
	EfiDevicePathProcotol,
	EfiDevicePathInto,
};

#[repr(C)]
pub struct EfiUsbDevicePath {
	base: EfiDevicePathProcotol,
	parent_port_number: u8,
	interface_number: u8,
}

impl EfiUsbDevicePath {
	pub fn parent_port_number(&self) -> u8 {
		self.parent_port_number
	}

	pub fn interface_number(&self) -> u8 {
		self.interface_number
	}
}

impl EfiDevicePathInto<EfiUsbDevicePath> for EfiUsbDevicePath {}
