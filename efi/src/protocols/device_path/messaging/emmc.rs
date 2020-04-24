use crate::protocols::device_path::{
	EfiDevicePathProcotol,
	EfiDevicePathInto,
};

#[repr(C)]
pub struct EfiEmbeddedMultiMediaCardDevicePath {
	base: EfiDevicePathProcotol,
	slot_number: u8,
}

impl EfiEmbeddedMultiMediaCardDevicePath {
	pub fn firewire_guid(&self) -> u8 {
		self.slot_number
	}
}

impl EfiDevicePathInto<EfiEmbeddedMultiMediaCardDevicePath> for EfiEmbeddedMultiMediaCardDevicePath {}
