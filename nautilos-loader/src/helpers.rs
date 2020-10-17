use {
    crate::{efi_defs::OsMemoryType, warn},
    core::{mem::size_of, slice::from_raw_parts_mut},
    efi::{
        boot_services::{memory::EfiMemoryType, EfiBootServicesRevision1_0},
        EfiStatus, EfiStatusEnum, VoidMutPtr,
    },
};

pub fn alloc<T>(boot_services: &dyn EfiBootServicesRevision1_0, length: usize) -> &'static mut [T] {
    match boot_services.allocate_pool(
        EfiMemoryType::custom(OsMemoryType::HandlesBuffer.into()),
        length * size_of::<T>(),
    ) {
        EfiStatusEnum::Success(ptr) => unsafe {
            from_raw_parts_mut(ptr as VoidMutPtr as *mut T, length)
        },
        EfiStatusEnum::Warning(status, ptr) => {
            warn!(
                "(EFI) Warning occured while allocating memory.\tWarning: {:?}",
                EfiStatus::from(status).get_warning()
            );

            unsafe { from_raw_parts_mut(ptr as VoidMutPtr as *mut T, length) }
        }
        EfiStatusEnum::Error(status, _) => {
            panic!(
                "(EFI) Error occured while allocating memory!\nError: {:?}",
                EfiStatus::from(status).get_error()
            );
        }
    }
}
