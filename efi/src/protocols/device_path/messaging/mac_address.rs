use crate::protocols::device_path::{
	EfiDevicePathProcotol,
	EfiDevicePathInto,
};

#[repr(C)]
pub struct EfiMacAddressDevicePath {
	base: EfiDevicePathProcotol,
	mac_address: [u8; 32],
	if_type: u8,
}

impl EfiMacAddressDevicePath {
	pub fn mac_address(&self) -> [u8; 32] {
		self.mac_address
	}

	pub fn if_type(&self) -> u8 {
		self.if_type
	}
}

impl EfiDevicePathInto<EfiMacAddressDevicePath> for EfiMacAddressDevicePath {}
