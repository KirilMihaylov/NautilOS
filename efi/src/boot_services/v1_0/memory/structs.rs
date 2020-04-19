use core::{
	marker::PhantomData,
	iter::StepBy,
	slice::{
		IterMut,
		from_raw_parts_mut,
	},
};

use crate::{
	types::{
		EfiPhysicalAddress,
		EfiVirtualAddress,
		VoidPtr,
	},
	status::{
		EfiStatus,
		EfiStatusEnum,
	},
};

use super::enums::{
	EfiAllocateType,
	EfiMemoryType,
};

#[repr(C)]
pub struct EfiMemory {
	allocate_pages: extern "efiapi" fn(EfiAllocateType, EfiMemoryType, usize, *mut EfiPhysicalAddress) -> EfiStatus,
	free_pages: extern "efiapi" fn(EfiPhysicalAddress, usize) -> EfiStatus,
	get_memory_map: extern "efiapi" fn(*mut usize, *mut EfiMemoryDescriptor, *mut usize, *mut usize, *mut u32) -> EfiStatus,
	allocate_pool: extern "efiapi" fn(EfiMemoryType, usize, *mut VoidPtr) -> EfiStatus,
	free_pool: extern "efiapi" fn(VoidPtr) -> EfiStatus,
}

impl EfiMemory {
	pub fn allocate_pages(&self, allocation_type: EfiAllocateType, memory_type: EfiMemoryType, number_of_pages: usize, physical_address: &mut EfiPhysicalAddress) -> EfiStatusEnum {
		(self.allocate_pages)(
			allocation_type,
			memory_type,
			number_of_pages,
			physical_address,
		).into_enum()
	}

	pub fn free_pages(&self, physical_address: EfiPhysicalAddress, number_of_pages: usize) -> EfiStatusEnum {
		(self.free_pages)(
			physical_address,
			number_of_pages
		).into_enum()
	}

	pub fn get_memory_map<'a>(&self, memory_map: &'a mut [u8]) -> EfiStatusEnum<(EfiMemoryDescriptorIterator<'a>, EfiMemoryDescriptorMetadata), usize> {
		let (
			mut allocation_size,
			mut memory_map_key,
			mut descriptor_size,
			mut descriptor_version
		): (usize, usize, usize, u32) = (memory_map.len(), 0, 0, 0);

		let result: EfiStatus = (self.get_memory_map)(
			&mut allocation_size,
			memory_map.as_mut_ptr() as *mut EfiMemoryDescriptor,
			&mut memory_map_key,
			&mut descriptor_size,
			&mut descriptor_version,
		);

		/* Skip heavy contruction procedures when error is returned */
		if result.is_error() {
			return EfiStatusEnum::Error(result.into(), allocation_size);
		}
		
		result.into_enum_data_error(
			(
				EfiMemoryDescriptorIterator {
					descriptor_iterator: unsafe {
						from_raw_parts_mut(
							memory_map.as_ptr() as *mut u8,
							allocation_size - (allocation_size % descriptor_size),
						).iter_mut().step_by(descriptor_size)
					},
					_phantom_data: PhantomData,
				},
				EfiMemoryDescriptorMetadata { 
					memory_map_size: allocation_size,
					descriptor_size: descriptor_size,
					descriptor_version: descriptor_version,
					descriptors_array: memory_map.as_ptr() as *const EfiMemoryDescriptor,
				}
			),
			allocation_size
		)
	}

	pub fn allocate_pool(&self, pool_type: EfiMemoryType, pool_size: usize) -> EfiStatusEnum<VoidPtr> {
		let mut buffer: VoidPtr = 0 as VoidPtr;
		
		(self.allocate_pool)(
			pool_type,
			pool_size,
			&mut buffer
		).into_enum_data(buffer)
	}

	pub fn free_pool(&self, buffer: VoidPtr) -> EfiStatusEnum {
		(self.free_pool)(
			buffer
		).into_enum()
	}
}

#[repr(C)]
#[derive(Clone,Copy)]
pub struct EfiMemoryDescriptor {
	memory_type: u32,
	physical_start: EfiPhysicalAddress,
	virtual_start: EfiVirtualAddress,
	number_of_pages: u64,
	attribute: u64,
}

#[derive(Clone,Copy)]
pub struct EfiMemoryDescriptorMetadata {
	memory_map_size: usize,
	descriptor_size: usize,
	descriptor_version: u32,
	descriptors_array: *const EfiMemoryDescriptor,
}

impl EfiMemoryDescriptorMetadata {
	pub fn memory_map_size(&self) -> usize {
		self.memory_map_size
	}

	pub fn descriptor_size(&self) -> usize {
		self.descriptor_size
	}

	pub fn descriptor_version(&self) -> u32 {
		self.descriptor_version
	}

	pub(crate) fn descriptors_array(&self) -> *const EfiMemoryDescriptor {
		self.descriptors_array
	}
}

pub struct EfiMemoryDescriptorIterator<'a> {
	descriptor_iterator: StepBy<IterMut<'a, u8>>,
	_phantom_data: PhantomData<&'a EfiMemoryDescriptor>,
}

impl<'a> Iterator for EfiMemoryDescriptorIterator<'a> {
	type Item = &'a mut EfiMemoryDescriptor;

	fn next(&mut self) -> Option<<Self as Iterator>::Item> {
		match self.descriptor_iterator.next() {
			None => None,
			Some(descriptor) => {
				Some(
					unsafe {
						&mut *(descriptor as *mut u8 as *mut <Self as Iterator>::Item)
					}
				)
			}
		}
	}
}
