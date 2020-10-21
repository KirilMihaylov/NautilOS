use {
    crate::warn,
    core::{mem::size_of, slice::from_raw_parts_mut},
    efi::{
        boot_services::{
            memory::{EfiMemoryType, EFI_MEMORY_TYPE_SIZE},
            EfiBootServicesRevision1x0,
        },
        EfiStatusEnum, VoidMutPtr,
    },
};

pub fn efi_alloc<T, U>(
    boot_services: &dyn EfiBootServicesRevision1x0,
    length: usize,
    memory_type: U,
) -> &'static mut [T]
where
    U: Into<[u8; EFI_MEMORY_TYPE_SIZE]>,
{
    match boot_services.allocate_pool(
        EfiMemoryType::custom(memory_type.into()),
        length * size_of::<T>(),
    ) {
        EfiStatusEnum::Success(ptr) => unsafe {
            from_raw_parts_mut(ptr as VoidMutPtr as *mut T, length)
        },
        EfiStatusEnum::Warning(status, ptr) => {
            warn!(
                "(EFI) Warning occured while allocating memory.\tWarning: {:?}",
                status
            );

            unsafe { from_raw_parts_mut(ptr as VoidMutPtr as *mut T, length) }
        }
        EfiStatusEnum::Error(status, _) => {
            panic!(
                "(EFI) Error occured while allocating memory!\nError: {:?}",
                status
            );
        }
    }
}
