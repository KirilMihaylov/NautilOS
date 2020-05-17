use core::slice::from_raw_parts;

use efi_interops::{
	types,
	traits,
};

use crate::{
	types::VoidPtr,
	guid::EfiGuid,
};

#[repr(transparent)]
pub struct EfiConfigurationTable<'a> {
	entries: &'a [EfiConfigurationTableEntry],
}

impl<'a> EfiConfigurationTable<'a> {
	pub(crate) unsafe fn new(configuration_table: *const EfiConfigurationTableEntry, configuration_table_size: usize) -> Self {
		Self {
			entries: from_raw_parts(
				configuration_table,
				configuration_table_size,
			),
		}
	}

	pub fn get(&'a self, guid: EfiGuid) -> EfiConfigurationTableIterator<'a> {
		EfiConfigurationTableIterator {
			entries: self.entries,
			guid: guid,
		}
	}
}

pub struct EfiConfigurationTableIterator<'a> {
	entries: &'a [EfiConfigurationTableEntry],
	guid: EfiGuid,
}

impl<'a> Iterator for EfiConfigurationTableIterator<'a> {
	type Item = &'a EfiConfigurationTableEntry;

	fn next(&mut self) -> Option<<Self as Iterator>::Item> {
		for (index, entry) in self.entries.iter().enumerate() {
			if entry.vendor_guid == self.guid {
				self.entries = &self.entries[(index + 1)..];
				return Some(entry);
			}
		}
		None
	}
}

#[repr(C)]
pub struct EfiConfigurationTableEntry {
	vendor_guid: EfiGuid,
	vendor_table: VoidPtr,
}

impl EfiConfigurationTableEntry {
	pub fn guid(&self) -> EfiGuid {
		self.vendor_guid
	}

	pub fn get_raw(&self) -> VoidPtr {
		self.vendor_table
	}

	pub fn get<T: traits::EfiConfigurationTable>(&self) -> Option<&'static T> {
		if T::guid() == self.vendor_guid {
			Some(
				unsafe {
					&*(self.vendor_table as *const T)
				}
			)
		} else {
			None
		}
	}
}

#[repr(C)]
pub struct EfiRTPropertiesTable {
	version: u16,
	length: u16,
	runtime_services_supported: u32,
}

impl EfiRTPropertiesTable {
	pub fn version(&self) -> u16 {
		self.version
	}

	pub fn length(&self) -> u16 {
		self.length
	}

	pub fn get_time_supported(&self) -> bool {
		self.runtime_services_supported & 1 == 1
	}

	pub fn set_time_supported(&self) -> bool {
		self.runtime_services_supported & 2 == 2
	}

	pub fn get_wakeup_time_supported(&self) -> bool {
		self.runtime_services_supported & 4 == 4
	}

	pub fn set_wakeup_time_supported(&self) -> bool {
		self.runtime_services_supported & 8 == 8
	}

	pub fn get_variable_supported(&self) -> bool {
		self.runtime_services_supported & 0x10 == 0x10
	}

	pub fn get_next_variable_supported(&self) -> bool {
		self.runtime_services_supported & 0x20 == 0x20
	}

	pub fn set_variable_supported(&self) -> bool {
		self.runtime_services_supported & 0x40 == 0x40
	}

	pub fn set_virtual_address_map_supported(&self) -> bool {
		self.runtime_services_supported & 0x80 == 0x80
	}

	pub fn convert_pointer_supported(&self) -> bool {
		self.runtime_services_supported & 0x100 == 0x100
	}

	pub fn get_next_high_monotonic_count_supported(&self) -> bool {
		self.runtime_services_supported & 0x200 == 0x200
	}

	pub fn reset_system_supported(&self) -> bool {
		self.runtime_services_supported & 0x400 == 0x400
	}

	pub fn update_capsule_supported(&self) -> bool {
		self.runtime_services_supported & 0x800 == 0x800
	}

	pub fn query_capsule_capabilities_supported(&self) -> bool {
		self.runtime_services_supported & 0x1000 == 0x1000
	}

	pub fn query_variable_info_supported(&self) -> bool {
		self.runtime_services_supported & 0x2000 == 0x2000
	}
}

unsafe impl traits::EfiConfigurationTable for EfiRTPropertiesTable {
	fn guid() -> types::EfiGuid {
		(0xeb66918a, 0x7eef, 0x402a, [0x84, 0x2e, 0x93, 0x1d, 0x21, 0xc3, 0x8a, 0xe9])
	}
}
