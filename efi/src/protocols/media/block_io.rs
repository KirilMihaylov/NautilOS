use crate::{
    guid::EfiGuid,
    protocols::EfiProtocol,
    status::{EfiStatus, EfiStatusEnum},
    types::{EfiLBA, VoidMutPtr, VoidPtr},
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
    fn media(&self) -> &EfiBlockIOMediaRaw {
        unsafe { &*self.media }
    }

    pub fn revision(&self) -> u64 {
        self.revision
    }

    pub fn media_revision_1(&self) -> &dyn EfiBlockIOMediaRevision1 {
        self.media()
    }

    pub fn media_revision_2(&self) -> Option<&dyn EfiBlockIOMediaRevision2> {
        if self.revision >= 0x20001 {
            Some(self.media())
        } else {
            None
        }
    }

    pub fn media_revision_3(&self) -> Option<&dyn EfiBlockIOMediaRevision3> {
        if self.revision >= 0x2001F {
            Some(self.media())
        } else {
            None
        }
    }

    pub fn reset(&self, extended_verification: bool) -> EfiStatusEnum {
        (self.reset)(self, extended_verification).into_enum()
    }

    pub fn read_blocks(&self, media_id: u32, lba: EfiLBA, buffer: &mut [u8]) -> EfiStatusEnum {
        (self.read_blocks)(
            self,
            media_id,
            lba,
            buffer.len(),
            buffer.as_mut_ptr() as VoidMutPtr,
        )
        .into_enum()
    }

    pub fn write_blocks(&self, media_id: u32, lba: EfiLBA, buffer: &[u8]) -> EfiStatusEnum {
        (self.write_blocks)(
            self,
            media_id,
            lba,
            buffer.len(),
            buffer.as_ptr() as VoidPtr,
        )
        .into_enum()
    }

    pub fn flush_blocks(&self) -> EfiStatusEnum {
        (self.flush_blocks)(self).into_enum()
    }
}

impl EfiProtocol for EfiBlockIOProtocol {
    fn guid() -> EfiGuid {
        (
            0x964E5B21,
            0x6459,
            0x11D2,
            [0x8E, 0x39, 0x00, 0xA0, 0xC9, 0x69, 0x72, 0x3B],
        )
            .into()
    }
}

#[repr(C)]
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

/// Defines interface for accessing state defined in the base revision.
pub trait EfiBlockIOMediaRevision1 {
    fn media_id(&self) -> u32;

    fn removable_media(&self) -> bool;

    fn media_present(&self) -> bool;

    fn logical_partition(&self) -> bool;

    fn read_only(&self) -> bool;

    fn write_caching(&self) -> bool;

    fn block_size(&self) -> u32;

    fn io_alignment(&self) -> u32;

    fn last_block(&self) -> EfiLBA;
}

impl core::fmt::Debug for &dyn EfiBlockIOMediaRevision1 {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(
			f,
			"EfiBlockIOMediaRevision1 {{ ID: {}, Removable: {}, Present: {}, Logical Partition: {}, Read-Only: {}, Write-caching: {}, Block size: {}, I/O alignment: {}, Last block's LBA: {} }}",
			self.media_id(),
			self.removable_media(),
			self.media_present(),
			self.logical_partition(),
			self.read_only(),
			self.write_caching(),
			self.block_size(),
			self.io_alignment(),
			self.last_block(),
		)
    }
}

/// Extends [`EfiBlockIOMediaRevision1`]'s interface for accessing state defined in revision 2.
pub trait EfiBlockIOMediaRevision2: EfiBlockIOMediaRevision1 {
    fn lowest_aligned_lba(&self) -> EfiLBA;

    fn logical_blocks_per_physical_block(&self) -> u32;
}

impl core::fmt::Debug for &dyn EfiBlockIOMediaRevision2 {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(
			f,
			"EfiBlockIOMediaRevision2 {{ ID: {}, Removable: {}, Present: {}, Logical Partition: {}, Read-Only: {}, Write-caching: {}, Block size: {}, I/O alignment: {}, Last block's LBA: {}, Lowest aligned LBA: {}, Logical blocks per physical block: {} }}",
			self.media_id(),
			self.removable_media(),
			self.media_present(),
			self.logical_partition(),
			self.read_only(),
			self.write_caching(),
			self.block_size(),
			self.io_alignment(),
			self.last_block(),
			self.lowest_aligned_lba(),
			self.logical_blocks_per_physical_block(),
		)
    }
}

/// Extends [`EfiBlockIOMediaRevision2`]'s interface for accessing state defined in revision 3.
pub trait EfiBlockIOMediaRevision3: EfiBlockIOMediaRevision2 {
    fn optimal_transfer_length_granulary(&self) -> u32;
}

impl core::fmt::Debug for &dyn EfiBlockIOMediaRevision3 {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(
			f,
			"EfiBlockIOMediaRevision3 {{ ID: {}, Removable: {}, Present: {}, Logical Partition: {}, Read-Only: {}, Write-caching: {}, Block size: {}, I/O alignment: {}, Last block's LBA: {}, Lowest aligned LBA: {}, Logical blocks per physical block: {}, Optimal transfer length granulary: {} }}",
			self.media_id(),
			self.removable_media(),
			self.media_present(),
			self.logical_partition(),
			self.read_only(),
			self.write_caching(),
			self.block_size(),
			self.io_alignment(),
			self.last_block(),
			self.lowest_aligned_lba(),
			self.logical_blocks_per_physical_block(),
			self.optimal_transfer_length_granulary(),
		)
    }
}

impl EfiBlockIOMediaRevision1 for EfiBlockIOMediaRaw {
    fn media_id(&self) -> u32 {
        self.media_id
    }

    fn removable_media(&self) -> bool {
        self.removable_media
    }

    fn media_present(&self) -> bool {
        self.media_present
    }

    fn logical_partition(&self) -> bool {
        self.logical_partition
    }

    fn read_only(&self) -> bool {
        self.read_only
    }

    fn write_caching(&self) -> bool {
        self.write_caching
    }

    fn block_size(&self) -> u32 {
        self.block_size
    }

    fn io_alignment(&self) -> u32 {
        self.io_alignment
    }

    fn last_block(&self) -> EfiLBA {
        self.last_block
    }
}

impl EfiBlockIOMediaRevision2 for EfiBlockIOMediaRaw {
    fn lowest_aligned_lba(&self) -> EfiLBA {
        self.lowest_aligned_lba
    }

    fn logical_blocks_per_physical_block(&self) -> u32 {
        self.logical_blocks_per_physical_block
    }
}

impl EfiBlockIOMediaRevision3 for EfiBlockIOMediaRaw {
    fn optimal_transfer_length_granulary(&self) -> u32 {
        self.optimal_transfer_length_granulary
    }
}
