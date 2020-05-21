use crate::protocols::device_path::{
	EfiDevicePathProcotol,
	EfiDevicePathRepr,
};

#[repr(C)]
pub struct EfiHardDriveDevicePath {
	base: EfiDevicePathProcotol,
	partition_number: [u8; 4],
	partition_start: [u8; 8],
	partition_size: [u8; 8],
	partition_signature: [u8; 16],
	partition_format: u8,
	signature_type: u8,
}

impl EfiHardDriveDevicePath {
	pub fn partition_number(&self) -> u32 {
		unsafe {
			(
				self.partition_number.as_ptr() as *const u32
			).read_unaligned()
		}
	}

	pub fn partition_start(&self) -> u64 {
		unsafe {
			(
				self.partition_start.as_ptr() as *const u64
			).read_unaligned()
		}
	}

	pub fn partition_size(&self) -> u64 {
		unsafe {
			(
				self.partition_size.as_ptr() as *const u64
			).read_unaligned()
		}
	}

	pub fn partition_signature(&self) -> EfiHardDriveDevicePathPartitionSignature {
		match self.signature_type {
			0 => EfiHardDriveDevicePathPartitionSignature::NoSignature,
			1 => EfiHardDriveDevicePathPartitionSignature::MBR([self.partition_signature[0], self.partition_signature[1], self.partition_signature[2], self.partition_signature[3]]),
			2 => EfiHardDriveDevicePathPartitionSignature::GUID(self.partition_signature),
			
			_ => EfiHardDriveDevicePathPartitionSignature::UnknownSignature,
		}
	}

	pub fn partition_format(&self) -> EfiHardDriveDevicePathPartitionFormat {
		match self.partition_format {
			1 => EfiHardDriveDevicePathPartitionFormat::MBR,
			2 => EfiHardDriveDevicePathPartitionFormat::GUID,

			_ => EfiHardDriveDevicePathPartitionFormat::UnknownFormat,
		}
	}
}

impl EfiDevicePathRepr for EfiHardDriveDevicePath {}

#[non_exhaustive]
#[derive(Clone,Copy)]
pub enum EfiHardDriveDevicePathPartitionFormat {
	UnknownFormat,

	MBR,
	GUID,
}

#[non_exhaustive]
#[derive(Clone,Copy)]
pub enum EfiHardDriveDevicePathPartitionSignature {
	UnknownSignature,

	NoSignature,

	MBR([u8; 4]),
	GUID([u8; 16]),
}
