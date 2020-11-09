use crate::{
    boot_services::types::memory::{EfiMemoryDescriptor, EfiMemoryDescriptors},
    EfiStatus, EfiStatusEnum, VoidMutPtrPtr, VoidPtr,
};

#[repr(C)]
#[derive(Clone, Copy)]
pub(super) struct EfiVirtualMemoryRaw {
    set_virtual_address_map:
        extern "efiapi" fn(usize, usize, u32, *const EfiMemoryDescriptor) -> EfiStatus,
    convert_pointer: extern "efiapi" fn(usize, VoidMutPtrPtr) -> EfiStatus,
}

impl EfiVirtualMemoryRaw {
    pub(super) fn set_virtual_address_map(
        &self,
        _memory_map: EfiMemoryDescriptors,
    ) -> EfiStatusEnum {
        // TODO

        // (self.set_virtual_address_map)(
        //     memory_map.memory_map_size(),
        //     memory_map.descriptor_size(),
        //     memory_map.descriptor_version(),
        //     memory_map.as_ptr(),
        // )
        // .into_enum()
        EfiStatus::success().into_enum()
    }

    pub(super) fn convert_pointer(
        &self,
        pointer: &mut VoidPtr,
        flags_builder: EfiConvertPointerFlagsBuilder,
    ) -> EfiStatusEnum {
        (self.convert_pointer)(flags_builder.finish(), pointer as *mut VoidPtr).into_enum()
    }
}

pub trait EfiVirtualMemory {
    fn set_virtual_address_map(&self, memory_map: EfiMemoryDescriptors) -> EfiStatusEnum;

    fn convert_pointer(
        &self,
        pointer: &mut VoidPtr,
        flags_builder: EfiConvertPointerFlagsBuilder,
    ) -> EfiStatusEnum;
}

#[repr(transparent)]
#[derive(Clone, Copy)]
pub struct EfiConvertPointerFlagsBuilder {
    flags: usize,
}

impl EfiConvertPointerFlagsBuilder {
    pub fn new() -> Self {
        Self { flags: 0 }
    }

    pub fn finish(&self) -> usize {
        self.flags
    }

    pub fn custom_flag(&mut self, offset: usize, value: bool) -> Self {
        match value {
            true => self.flags |= 1usize << offset,
            false => self.flags &= !(1usize << offset),
        }

        Self { flags: self.flags }
    }

    pub fn optional_pointer(&mut self, value: bool) -> Self {
        self.custom_flag(1, value)
    }
}

impl Default for EfiConvertPointerFlagsBuilder {
    fn default() -> Self {
        Self::new()
    }
}
