use crate::{
	types::VoidMutPtrPtr,
	status::{
		EfiStatus,
		EfiStatusEnum,
	},
	boot_services::memory::structs::{
		EfiMemoryDescriptor,
		EfiMemoryMapMetadata,
	},
};

#[repr(C)]
pub struct EfiVirtualMemory {
	set_virtual_address_map: extern "efiapi" fn(usize, usize, u32, *const EfiMemoryDescriptor) -> EfiStatus,
	convert_pointer: extern "efiapi" fn(usize, VoidMutPtrPtr) -> EfiStatus,
}

impl EfiVirtualMemory {
	pub fn set_virtual_address_map(&self, metadata: EfiMemoryMapMetadata) -> EfiStatusEnum {
		(self.set_virtual_address_map)(
			metadata.memory_map_size(),
			metadata.descriptor_size(),
			metadata.descriptor_version(),
			metadata.descriptors_array()
		).into_enum()
	}

	pub fn convert_pointer<T>(&self, pointer: &mut &T, flags_builder: EfiConvertPointerFlagsBuilder) -> EfiStatusEnum {
		(self.convert_pointer)(
			flags_builder.finish(),
			pointer as *mut &T as VoidMutPtrPtr
		).into_enum()
	}

	pub fn convert_raw_pointer<T>(&self, pointer: &mut *const T, flags_builder: EfiConvertPointerFlagsBuilder) -> EfiStatusEnum {
		(self.convert_pointer)(
			flags_builder.finish(),
			pointer as *mut *const T as VoidMutPtrPtr
		).into_enum()
	}
}

#[repr(transparent)]
#[derive(Clone)]
pub struct EfiConvertPointerFlagsBuilder {
	flags: usize,
}

impl EfiConvertPointerFlagsBuilder {
	pub fn new() -> Self {
		Self {
			flags: 0,
		}
	}

	pub fn finish(&self) -> usize {
		self.flags
	}

	pub fn custom_flag(&mut self, offset: usize, value: bool) -> Self {
		match value {
			true => self.flags |= 1usize << offset,
			false => self.flags &= !(1usize << offset),
		}

		Self {
			flags: self.flags,
		}
	}

	pub fn optional_pointer(&mut self, value: bool) -> Self {
		self.custom_flag(1, value)
	}
}
