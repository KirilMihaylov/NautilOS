use crate::protocols::device_path::{
	EfiDevicePathProcotol,
	EfiDevicePathInto,
};

#[repr(C)]
pub struct EfiPciDevicePath {
	base: EfiDevicePathProcotol,
	function: u8,
	device: u8,
}

impl EfiPciDevicePath {
	pub fn device(&self) -> u8 {
		self.device
	}

	pub fn function(&self) -> u8 {
		self.function
	}
}

impl EfiDevicePathInto<EfiPciDevicePath> for EfiPciDevicePath {}
