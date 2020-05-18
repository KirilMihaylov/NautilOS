use crate::{
	types::{
		VoidPtr,
		VoidMutPtr,
	},
	status::{
		EfiStatus,
		EfiStatusEnum,
	},
	guid::EfiGuid,
	protocols::EfiProtocol,
};

/// Implementation of EFI's `EFI_DISK_IO_PROTOCOL`.
#[repr(C)]
pub struct EfiDiskIOProtocol {
	revision: u64,
	read_disk: extern "efiapi" fn(*const Self, u32, u64, usize, VoidMutPtr) -> EfiStatus,
	write_disk: extern "efiapi" fn(*const Self, u32, u64, usize, VoidPtr) -> EfiStatus,
}

impl EfiDiskIOProtocol {
	pub fn revision(&self) -> u64 {
		self.revision
	}

	pub fn read_disk(&self, media_id: u32, offset: u64, buffer: &mut [u8]) -> EfiStatusEnum {
		(self.read_disk)(
			self,
			media_id,
			offset,
			buffer.len(),
			buffer.as_mut_ptr() as VoidMutPtr,
		).into_enum()
	}

	pub fn write_disk(&self, media_id: u32, offset: u64, buffer: &[u8]) -> EfiStatusEnum {
		(self.write_disk)(
			self,
			media_id,
			offset,
			buffer.len(),
			buffer.as_ptr() as VoidPtr,
		).into_enum()
	}
}

impl EfiProtocol for EfiDiskIOProtocol {
	fn guid() -> EfiGuid {
		(0xCE345171, 0xBA0B, 0x11D2, [0x8E, 0x4F, 0x00, 0xA0, 0xC9, 0x69, 0x72, 0x3B]).into()
	}
}
