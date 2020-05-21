use crate::protocols::device_path::{
	EfiDevicePathProcotol,
	EfiDevicePathRepr,
};

#[repr(C)]
pub struct EfiNvdimmNamespaceDevicePath {
	base: EfiDevicePathProcotol,
	uuid: [u8; 16],
}

impl EfiNvdimmNamespaceDevicePath {
	pub fn uuid(&self) -> [u8; 16] {
		self.uuid
	}
}

impl EfiDevicePathRepr for EfiNvdimmNamespaceDevicePath {}
