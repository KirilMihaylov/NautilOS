use crate::protocols::device_path::{
	EfiDevicePathProcotol,
	EfiDevicePathRepr,
};

#[repr(C)]
pub struct EfiInfiniBandDevicePath {
	base: EfiDevicePathProcotol,
	resource_flags: [u8; 4],
	port_gid: [u8; 16],
	id: [u8; 8],
	target_port_id: [u8; 8],
	device_id: [u8; 8],
}

impl EfiInfiniBandDevicePath {
	pub fn resource_flags(&self) -> EfiInfiniBandDevicePathResourceFlags {
		unsafe {
			(
				self.resource_flags.as_ptr() as *const EfiInfiniBandDevicePathResourceFlags
			).read_unaligned()
		}
	}

	pub fn port_gid(&self) -> u128 {
		unsafe {
			(
				self.port_gid.as_ptr() as *const u128
			).read_unaligned()
		}
	}

	pub fn id(&self) -> u64 {
		unsafe {
			(
				self.id.as_ptr() as *const u64
			).read_unaligned()
		}
	}

	pub fn target_port_id(&self) -> u64 {
		unsafe {
			(
				self.target_port_id.as_ptr() as *const u64
			).read_unaligned()
		}
	}

	pub fn device_id(&self) -> u64 {
		unsafe {
			(
				self.device_id.as_ptr() as *const u64
			).read_unaligned()
		}
	}
}

impl EfiDevicePathRepr for EfiInfiniBandDevicePath {}

#[repr(transparent)]
#[derive(Clone,Copy)]
pub struct EfiInfiniBandDevicePathResourceFlags {
	resource_flags: u32,
}

impl EfiInfiniBandDevicePathResourceFlags {
	pub fn ioc_service(&self) -> EfiInfiniBandDevicePathType {
		use EfiInfiniBandDevicePathType::*;

		match self.resource_flags & 1 {
			0 => IOC,
			1 => Service,
			_ => unreachable!(),
		}
	}

	pub fn extended_boot_environment(&self) -> bool {
		self.resource_flags & 2 == 2
	}

	pub fn console_protocol(&self) -> bool {
		self.resource_flags & 4 == 4
	}

	pub fn storage_protocol(&self) -> bool {
		self.resource_flags & 8 == 8
	}

	pub fn network_protocol(&self) -> bool {
		self.resource_flags & 0x10 == 0x10
	}
}

impl From<EfiInfiniBandDevicePathResourceFlags> for u32 {
	fn from(data: EfiInfiniBandDevicePathResourceFlags) -> Self {
		data.resource_flags
	}
}

pub enum EfiInfiniBandDevicePathType {
	IOC,
	Service,
}
