use crate::{
	types::{
		EfiLBA,
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

/// Implementation of EFI's `EFI_BLOCK_IO_PROTOCOL`.
#[repr(C)]
pub struct EfiBlockIOProtocol {
	revision: u64,
	media: *const EfiBlockIOMediaRaw,
	reset: extern "efiapi" fn(*const Self, bool) -> EfiStatus,
	read_blocks: extern "efiapi" fn(*const Self, u32, EfiLBA, usize, VoidMutPtr) -> EfiStatus,
	write_blocks: extern "efiapi" fn(*const Self, u32, EfiLBA, usize, VoidPtr) -> EfiStatus,
	flush_blocks: extern "efiapi" fn(*const Self) -> EfiStatus,
}

impl EfiBlockIOProtocol {
	pub fn revision(&self) -> u64 {
		self.revision
	}

	pub fn media_revision_1<'a>(&'a self) -> impl EfiBlockIOMediaRevision1<'a> + 'a {
		EfiBlockIOMedia(self)
	}

	pub fn media_revision_2<'a>(&'a self) -> Option<impl EfiBlockIOMediaRevision2<'a> + 'a> {
		if self.revision >= 0x20001 {
			Some(EfiBlockIOMedia(self))
		} else {
			None
		}
	}

	pub fn media_revision_3<'a>(&'a self) -> Option<impl EfiBlockIOMediaRevision3<'a> + 'a> {
		if self.revision >= 0x2001F {
			Some(EfiBlockIOMedia(self))
		} else {
			None
		}
	}

	pub fn reset(&self, extended_verification: bool) -> EfiStatusEnum {
		(self.reset)(
			self,
			extended_verification,
		).into_enum()
	}

	pub fn read_blocks(&self, media_id: u32, lba: EfiLBA, buffer: &mut [u8]) -> EfiStatusEnum {
		(self.read_blocks)(
			self,
			media_id,
			lba,
			buffer.len(),
			buffer.as_mut_ptr() as VoidMutPtr,
		).into_enum()
	}

	pub fn write_blocks(&self, media_id: u32, lba: EfiLBA, buffer: &[u8]) -> EfiStatusEnum {
		(self.write_blocks)(
			self,
			media_id,
			lba,
			buffer.len(),
			buffer.as_ptr() as VoidPtr,
		).into_enum()
	}

	pub fn flush_blocks(&self) -> EfiStatusEnum {
		(self.flush_blocks)(
			self,
		).into_enum()
	}
}

impl EfiProtocol for EfiBlockIOProtocol {
	fn guid() -> EfiGuid {
		(0x964E5B21,0x6459,0x11D2, [0x8E, 0x39, 0x00, 0xA0, 0xC9, 0x69, 0x72, 0x3B]).into()
	}
}

#[repr(C)]
#[derive(Clone,Copy)]
struct EfiBlockIOMediaRaw {
	media_id: u32,
	removable_media: bool,
	media_present: bool,
	logical_partition: bool,
	read_only: bool,
	write_caching: bool,
	block_size: u32,
	io_alignment: u32,
	last_block: EfiLBA,
	/* Revision 2+ */
	lowest_aligned_lba: EfiLBA,
	logical_blocks_per_physical_block: u32,
	/* Revision 3+ */
	optimal_transfer_length_granulary: u32,
}

/// Defines [`EfiBlockIOMedia`]'s interface for accessing state defined in the base revision.
pub trait EfiBlockIOMediaRevision1<'a> {
	fn get_protocol(&'a self) -> &'a EfiBlockIOProtocol;

	fn media_id(&'a self) -> u32 {
		(unsafe { *self.get_protocol().media }).media_id
	}

	fn removable_media(&'a self) -> bool {
		(unsafe { *self.get_protocol().media }).removable_media
	}

	fn media_present(&'a self) -> bool {
		(unsafe { *self.get_protocol().media }).media_present
	}

	fn logical_partition(&'a self) -> bool {
		(unsafe { *self.get_protocol().media }).logical_partition
	}

	fn read_only(&'a self) -> bool {
		(unsafe { *self.get_protocol().media }).read_only
	}

	fn write_caching(&'a self) -> bool {
		(unsafe { *self.get_protocol().media }).write_caching
	}

	fn block_size(&'a self) -> u32 {
		(unsafe { *self.get_protocol().media }).block_size
	}

	fn io_alignment(&'a self) -> u32 {
		(unsafe { *self.get_protocol().media }).io_alignment
	}

	fn last_block(&'a self) -> EfiLBA {
		(unsafe { *self.get_protocol().media }).last_block
	}
}

/// Extends [`EfiBlockIOMediaRevision1`]'s interface for accessing state defined in revision 2.
pub trait EfiBlockIOMediaRevision2<'a>: EfiBlockIOMediaRevision1<'a> {
	fn lowest_aligned_lba(&'a self) -> EfiLBA {
		(unsafe { *self.get_protocol().media }).lowest_aligned_lba
	}

	fn logical_blocks_per_physical_block(&'a self) -> u32 {
		(unsafe { *self.get_protocol().media }).logical_blocks_per_physical_block
	}
}

/// Extends [`EfiBlockIOMediaRevision2`]'s interface for accessing state defined in revision 3.
pub trait EfiBlockIOMediaRevision3<'a>: EfiBlockIOMediaRevision2<'a> {
	fn optimal_transfer_length_granulary(&'a self) -> u32 {
		(unsafe { *self.get_protocol().media }).optimal_transfer_length_granulary
	}
}

/// Implementation of EFI's `EFI_BLOCK_IO_MEDIA`.
#[repr(transparent)]
#[derive(Clone,Copy)]
pub struct EfiBlockIOMedia<'a>(&'a EfiBlockIOProtocol);

impl<'a> EfiBlockIOMediaRevision1<'a> for EfiBlockIOMedia<'a> {
	fn get_protocol(&'a self) -> &'a EfiBlockIOProtocol {
		self.0
	}
}
impl<'a> EfiBlockIOMediaRevision2<'a> for EfiBlockIOMedia<'a> {}
impl<'a> EfiBlockIOMediaRevision3<'a> for EfiBlockIOMedia<'a> {}
