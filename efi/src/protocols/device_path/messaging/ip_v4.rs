use crate::protocols::device_path::{
	EfiDevicePathProcotol,
	EfiDevicePathInto,
};

#[repr(C)]
pub struct EfiIPv4DevicePath {
	base: EfiDevicePathProcotol,
	local_ip_address: [u8; 4],
	remote_ip_address: [u8; 4],
	local_port: [u8; 2],
	remote_port: [u8; 2],
	protocol: [u8; 2],
	static_ip_address: u8,
	/*~~~ Size check required (Length: 27) ~~~*/
	gateway_ip_address: [u8; 4],
	subnet_mask: [u8; 4],
}

impl EfiIPv4DevicePath {
	pub fn local_ip_address(&self) -> [u8; 4] {
		self.local_ip_address
	}

	pub fn remote_ip_address(&self) -> [u8; 4] {
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

	pub fn static_ip_address(&self) -> u8 {
		self.static_ip_address
	}

	pub fn gateway_ip_address(&self) -> Option<[u8; 4]> {
		if self.base.len() >= 27 {
			Some(self.gateway_ip_address)
		} else {
			None
		}
	}

	pub fn subnet_mask(&self) -> Option<[u8; 4]> {
		if self.base.len() >= 27 {
			Some(self.subnet_mask)
		} else {
			None
		}
	}
}

impl EfiDevicePathInto<EfiIPv4DevicePath> for EfiIPv4DevicePath {}
