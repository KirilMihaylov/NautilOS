use crate::protocols::device_path::{
	EfiDevicePathProcotol,
	EfiDevicePathInto,
};

#[repr(C)]
pub struct EfiUsbClassDevicePath {
	base: EfiDevicePathProcotol,
	vendor_id: [u8; 2],
	product_id: [u8; 2],
	device_class: u8,
	device_subclass: u8,
	device_protocol: u8,
}

impl EfiUsbClassDevicePath {
	pub fn vendor_id(&self) -> u16 {
		unsafe {
			(
				self.vendor_id.as_ptr() as *const u16
			).read_unaligned()
		}
	}

	pub fn product_id(&self) -> u16 {
		unsafe {
			(
				self.product_id.as_ptr() as *const u16
			).read_unaligned()
		}
	}

	pub fn device_class(&self) -> u8 {
		self.device_class
	}

	pub fn device_subclass(&self) -> u8 {
		self.device_subclass
	}

	pub fn device_protocol(&self) -> u8 {
		self.device_protocol
	}
}

impl EfiDevicePathInto<EfiUsbClassDevicePath> for EfiUsbClassDevicePath {}
