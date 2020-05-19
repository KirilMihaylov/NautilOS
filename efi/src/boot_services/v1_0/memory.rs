use core::marker::PhantomData;

use crate::*;

#[repr(C)]
#[derive(Clone,Copy)]
#[non_exhaustive]
pub enum EfiAllocateType {
	AllocateAnyPages,
	AllocateMaxAddress,
	AllocateAddress,
	MaxAllocateType,
}

#[repr(C)]
#[derive(Clone,Copy)]
#[non_exhaustive]
pub enum EfiMemoryType {
	EfiReservedMemoryType,
	EfiLoaderCode,
	EfiLoaderData,
	EfiBootServicesCode,
	EfiBootServicesData,
	EfiRuntimeServicesCode,
	EfiRuntimeServicesData,
	EfiConventionalMemory,
	EfiUnusableMemory,
	EfiACPIReclaimMemory,
	EfiACPIMemoryNVS,
	EfiMemoryMappedIO,
	EfiMemoryMappedIOPortSpace,
	EfiPalCode,
	EfiPersistentMemory,
	EfiMaxMemoryType,
}

#[repr(C)]
#[derive(Clone,Copy)]
pub(super) struct EfiMemoryRaw {
	allocate_pages: extern "efiapi" fn(EfiAllocateType, EfiMemoryType, usize, *mut EfiPhysicalAddress) -> EfiStatus,
	free_pages: extern "efiapi" fn(EfiPhysicalAddress, usize) -> EfiStatus,
	get_memory_map: extern "efiapi" fn(*mut usize, *mut EfiMemoryDescriptor, *mut usize, *mut usize, *mut u32) -> EfiStatus,
	allocate_pool: extern "efiapi" fn(EfiMemoryType, usize, *mut VoidPtr) -> EfiStatus,
	free_pool: extern "efiapi" fn(VoidPtr) -> EfiStatus,
}

impl EfiMemoryRaw {
	pub(super) fn allocate_pages(&self, allocation_type: EfiAllocateType, memory_type: EfiMemoryType, number_of_pages: usize, physical_address: &mut EfiPhysicalAddress) -> EfiStatusEnum {
		(self.allocate_pages)(
			allocation_type,
			memory_type,
			number_of_pages,
			physical_address,
		).into_enum()
	}

	pub(super) fn free_pages(&self, physical_address: EfiPhysicalAddress, number_of_pages: usize) -> EfiStatusEnum {
		(self.free_pages)(
			physical_address,
			number_of_pages
		).into_enum()
	}

	pub(super) fn get_memory_map<'a>(&self, memory_map: &'a mut [u8]) -> EfiStatusEnum<EfiMemoryDescriptors, usize> {
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
			EfiMemoryDescriptors {
				memory_map_key: memory_map_key,
				memory_map_size: allocation_size,
				descriptor_size: descriptor_size,
				descriptor_version: descriptor_version,
				descriptors_array: memory_map.as_ptr(),
			},
			allocation_size
		)
	}

	pub(super) fn allocate_pool(&self, pool_type: EfiMemoryType, pool_size: usize) -> EfiStatusEnum<VoidPtr> {
		let mut buffer: VoidPtr = 0 as VoidPtr;
		
		(self.allocate_pool)(
			pool_type,
			pool_size,
			&mut buffer
		).into_enum_data(buffer)
	}

	pub(super) fn free_pool(&self, buffer: VoidPtr) -> EfiStatusEnum {
		(self.free_pool)(
			buffer
		).into_enum()
	}
}

pub trait EfiMemory {
	fn allocate_pages(&self, allocation_type: EfiAllocateType, memory_type: EfiMemoryType, number_of_pages: usize, physical_address: &mut EfiPhysicalAddress) -> EfiStatusEnum;

	fn free_pages(&self, physical_address: EfiPhysicalAddress, number_of_pages: usize) -> EfiStatusEnum;

	fn get_memory_map<'a>(&self, memory_map: &'a mut [u8]) -> EfiStatusEnum<EfiMemoryDescriptors, usize>;

	fn allocate_pool(&self, pool_type: EfiMemoryType, pool_size: usize) -> EfiStatusEnum<VoidPtr>;

	fn free_pool(&self, buffer: VoidPtr) -> EfiStatusEnum;
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
pub struct EfiMemoryDescriptors {
	memory_map_key: usize,
	memory_map_size: usize,
	descriptor_size: usize,
	descriptor_version: u32,
	descriptors_array: *const u8,
}

impl EfiMemoryDescriptors {
	pub fn memory_map_key(&self) -> usize {
		self.memory_map_key
	}

	pub fn memory_map_size(&self) -> usize {
		self.memory_map_size
	}

	pub fn descriptor_size(&self) -> usize {
		self.descriptor_size
	}

	pub fn descriptor_version(&self) -> u32 {
		self.descriptor_version
	}

	pub(crate) fn as_ptr(&self) -> *const EfiMemoryDescriptor {
		self.descriptors_array as *const EfiMemoryDescriptor
	}

	pub fn into_iter(&self) -> impl Iterator<Item=EfiMemoryDescriptor> {
		EfiMemoryDescriptorIter {
			descriptor_size: self.descriptor_size,
			descriptor: self.descriptors_array,
			last_descriptor: unsafe {
				self.descriptors_array.offset((self.memory_map_size - (self.memory_map_size % self.descriptor_size)) as isize) as *const EfiMemoryDescriptor
			},
			_phantom_data: PhantomData,
		}
	}

	pub fn into_iter_mut<'a>(&'a mut self) -> impl Iterator<Item=&'a mut EfiMemoryDescriptor> {
		EfiMemoryDescriptorIterMut {
			descriptor_size: self.descriptor_size,
			descriptor: self.descriptors_array as *mut u8,
			last_descriptor: unsafe {
				self.descriptors_array.offset((self.memory_map_size - (self.memory_map_size % self.descriptor_size)) as isize) as *mut EfiMemoryDescriptor
			},
			_phantom_data: PhantomData,
		}
	}
}

struct EfiMemoryDescriptorIter {
	descriptor_size: usize,
	descriptor: *const u8,
	last_descriptor: *const EfiMemoryDescriptor,
	_phantom_data: PhantomData<EfiMemoryDescriptor>,
}

impl Iterator for EfiMemoryDescriptorIter {
	type Item = EfiMemoryDescriptor;

	fn next(&mut self) -> Option<<Self as Iterator>::Item> {
		let return_ptr: *const EfiMemoryDescriptor = self.descriptor as *const EfiMemoryDescriptor;
		
		if return_ptr.is_null() {
			return None;
		}

		if return_ptr == self.last_descriptor {
			self.descriptor = 0 as *const u8;
		} else {
			self.descriptor = unsafe { self.descriptor.offset(self.descriptor_size as isize) };
		}

		Some(unsafe { *return_ptr })
	}
}

struct EfiMemoryDescriptorIterMut<'a> {
	descriptor_size: usize,
	descriptor: *mut u8,
	last_descriptor: *mut EfiMemoryDescriptor,
	_phantom_data: PhantomData<&'a mut EfiMemoryDescriptor>,
}

impl<'a> Iterator for EfiMemoryDescriptorIterMut<'a> {
	type Item = &'a mut EfiMemoryDescriptor;

	fn next(&mut self) -> Option<<Self as Iterator>::Item> {
		let return_ptr: *mut EfiMemoryDescriptor = self.descriptor as *mut EfiMemoryDescriptor;
		
		if return_ptr.is_null() {
			return None;
		}
		
		if return_ptr == self.last_descriptor {
			self.descriptor = 0 as *mut u8;
		} else {
			self.descriptor = unsafe { self.descriptor.offset(self.descriptor_size as isize) };
		}

		Some(unsafe { &mut *return_ptr })
	}
}
