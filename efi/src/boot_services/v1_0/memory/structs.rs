use core::marker::PhantomData;

use crate::types::{
	EfiPhysicalAddress,
	EfiVirtualAddress,
	VoidPtr,
};
use crate::status::{
	EfiStatus,
	EfiStatusEnum,
};

use super::enums::{
	EfiAllocateType,
	EfiMemoryType,
};

pub struct EfiMemory {
	allocate_pages: extern "efiapi" fn(allocation_type: EfiAllocateType, memory_type: EfiMemoryType, number_of_pages: usize, physical_address: *mut EfiPhysicalAddress) -> EfiStatus,
	free_pages: extern "efiapi" fn(physical_address: EfiPhysicalAddress, number_of_pages: usize) -> EfiStatus,
	get_memory_map: extern "efiapi" fn(allocation_size: *mut usize, memory_map: *mut EfiMemoryDescriptor, memory_map_key: *mut usize, descriptor_size: *mut usize, descriptor_version: *mut usize) -> EfiStatus,
	allocate_pool: extern "efiapi" fn(pool_type: EfiMemoryType, pool_size: usize, buffer: *mut VoidPtr) -> EfiStatus,
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

	pub fn get_memory_map<'a>(&self, memory_map: &'a mut [u8]) -> EfiStatusEnum<(EfiMemoryDescriptorIterator<'a>, usize, usize, usize), usize> {
		let (
			mut allocation_size,
			mut memory_map_key,
			mut descriptor_size,
			mut descriptor_version
		): (usize, usize, usize, usize) = (memory_map.len(), 0, 0, 0);

		(self.get_memory_map)(
			&mut allocation_size,
			memory_map.as_mut_ptr() as *mut EfiMemoryDescriptor,
			&mut memory_map_key,
			&mut descriptor_size,
			&mut descriptor_version,
		).into_enum_data_error(
			(
				EfiMemoryDescriptorIterator {
					descriptor: memory_map.as_mut_ptr() as *mut EfiMemoryDescriptor,
					descriptor_size: descriptor_size,
					count_left: allocation_size / descriptor_size,
					_phantomdata: PhantomData
				},
				allocation_size,
				memory_map_key,
				descriptor_version
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

pub struct EfiMemoryDescriptorIterator<'a> {
	descriptor: *mut EfiMemoryDescriptor,
	descriptor_size: usize,
	count_left: usize,
	_phantomdata: PhantomData<&'a EfiMemoryDescriptor>,
}

impl<'a> Iterator for EfiMemoryDescriptorIterator<'a> {
	type Item = &'a mut EfiMemoryDescriptor;

	fn next(&mut self) -> Option<<Self as Iterator>::Item> {
		if self.count_left == 0 {
			None
		} else {
			let result: Option<Self::Item> = Some(
				unsafe {
					&mut *self.descriptor
				}
			);

			self.descriptor = (self.descriptor as usize + self.descriptor_size) as *mut EfiMemoryDescriptor;
			self.count_left -= 1;

			result
		}
	}
}
