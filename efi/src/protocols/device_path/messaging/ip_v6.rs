use crate::protocols::device_path::{
	EfiDevicePathProcotol,
	EfiDevicePathRepr,
};

#[repr(C)]
pub struct EfiIPv6DevicePath {
	base: EfiDevicePathProcotol,
	local_ip_address: [u8; 16],
	remote_ip_address: [u8; 16],
	local_port: [u8; 2],
	remote_port: [u8; 2],
	protocol: [u8; 2],
	ip_address_origin: u8,
	prefix_length: u8,
	gateway_ip_address: [u8; 16],
}

impl EfiIPv6DevicePath {
	pub fn local_ip_address(&self) -> [u8; 16] {
		self.local_ip_address
	}

	pub fn remote_ip_address(&self) -> [u8; 16] {
		self.remote_ip_address
	}

	pub fn local_port(&self) -> u16 {
		unsafe {
			(
				self.local_port.as_ptr() as *const u16
			).read_unaligned()
		}
	}

	pub fn remote_port(&self) -> u16 {
		unsafe {
			(
				self.remote_port.as_ptr() as *const u16
			).read_unaligned()
		}
	}

	pub fn protocol(&self) -> u16 {
		unsafe {
			(
				self.remote_port.as_ptr() as *const u16
			).read_unaligned()
		}
	}

	pub fn ip_address_origin(&self) -> u8 {
		self.ip_address_origin
	}

	pub fn prefix_length(&self) -> u8 {
		self.prefix_length
	}

	pub fn gateway_ip_address(&self) -> [u8; 16] {
		self.gateway_ip_address
	}
}

impl EfiDevicePathRepr for EfiIPv6DevicePath {}
