use crate::protocols::device_path::{
	EfiDevicePathProcotol,
	EfiDevicePathRepr,
};

#[repr(C)]
pub struct EfiSecureDigitalDevicePath {
	base: EfiDevicePathProcotol,
	slot_number: u8,
}

impl EfiSecureDigitalDevicePath {
	pub fn slot_number(&self) -> u8 {
		self.slot_number
	}
}

impl EfiDevicePathRepr for EfiSecureDigitalDevicePath {}
