use crate::protocols::device_path::{
	EfiDevicePathProcotol,
	EfiDevicePathInto,
};

#[repr(C)]
pub struct EfiBluetoothDevicePath {
	base: EfiDevicePathProcotol,
	device_address: [u8; 6],
}

impl EfiBluetoothDevicePath {
	pub fn device_address(&self) -> [u8; 6] {
		self.device_address
	}
}

impl EfiDevicePathInto<EfiBluetoothDevicePath> for EfiBluetoothDevicePath {}
