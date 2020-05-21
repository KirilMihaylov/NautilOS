use crate::protocols::device_path::{
	EfiDevicePathProcotol,
	EfiDevicePathRepr,
};

#[repr(C)]
pub struct EfiPcCardDevicePath {
	base: EfiDevicePathProcotol,
	function: u8,
}

impl EfiPcCardDevicePath {
	pub fn function(&self) -> u8 {
		self.function
	}
}

impl EfiDevicePathRepr for EfiPcCardDevicePath {}
