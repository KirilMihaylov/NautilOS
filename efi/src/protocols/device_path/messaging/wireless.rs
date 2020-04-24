use crate::protocols::device_path::{
	EfiDevicePathProcotol,
	EfiDevicePathInto,
};

#[repr(C)]
pub struct EfiWirelessDevicePath {
	base: EfiDevicePathProcotol,
	ssid: [u8; 32],
}

impl EfiWirelessDevicePath {
	pub fn ssid(&self) -> [u8; 32] {
		self.ssid
	}
}

impl EfiDevicePathInto<EfiWirelessDevicePath> for EfiWirelessDevicePath {}
