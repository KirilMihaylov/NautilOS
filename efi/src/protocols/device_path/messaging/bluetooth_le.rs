use crate::protocols::device_path::{
	EfiDevicePathProcotol,
	EfiDevicePathInto,
};

#[repr(C)]
pub struct EfiBluetoothLEDevicePath {
	base: EfiDevicePathProcotol,
	device_address: [u8; 6],
	address_type: u8,
}

impl EfiBluetoothLEDevicePath {
	pub fn device_address(&self) -> [u8; 6] {
		self.device_address
	}

	pub fn address_type(&self) -> EfiBluetoothLEDevicePathAddressType {
		use EfiBluetoothLEDevicePathAddressType::*;

		match self.address_type {
			0 => PublicDeviceAddress,
			1 => RandomDeviceAddress,

			x => Unknown(x),
		}
	}
}

impl EfiDevicePathInto<EfiBluetoothLEDevicePath> for EfiBluetoothLEDevicePath {}

#[non_exhaustive]
#[derive(Clone,Copy)]
pub enum EfiBluetoothLEDevicePathAddressType {
	PublicDeviceAddress,
	RandomDeviceAddress,

	Unknown(u8),
}
