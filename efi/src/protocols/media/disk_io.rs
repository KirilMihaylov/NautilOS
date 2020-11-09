use crate::{
    guid::EfiGuid,
    protocols::EfiProtocol,
    status::{EfiStatus, EfiStatusEnum},
    types::{VoidMutPtr, VoidPtr},
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
        )
        .into_enum()
    }

    pub fn write_disk(&self, media_id: u32, offset: u64, buffer: &[u8]) -> EfiStatusEnum {
        (self.write_disk)(
            self,
            media_id,
            offset,
            buffer.len(),
            buffer.as_ptr() as VoidPtr,
        )
        .into_enum()
    }
}

impl EfiProtocol for EfiDiskIOProtocol {
    type Parsed = &'static Self;
    type Error = !;

    fn guid() -> EfiGuid {
        crate::guids::EFI_DISK_IO_PROTOCOL
    }

    unsafe fn parse(ptr: VoidPtr) -> Result<<Self as EfiProtocol>::Parsed, <Self as EfiProtocol>::Error> {
        Ok(&*(ptr as *const Self))
    }
}
