use crate::protocols::device_path::{
	EfiDevicePathProcotol,
	EfiDevicePathRepr,
};

#[repr(C)]
pub struct EfiLogicalUnitDevicePath {
	base: EfiDevicePathProcotol,
	logical_unit_number: u8,
}

impl EfiLogicalUnitDevicePath {
	pub fn logical_unit_number(&self) -> u8 {
		self.logical_unit_number
	}
}

impl EfiDevicePathRepr for EfiLogicalUnitDevicePath {}
