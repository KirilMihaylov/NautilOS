use core::slice::from_raw_parts;

use efi_interops::traits;

use crate::types::VoidPtr;
use crate::guid::EfiGuid;

#[repr(transparent)]
pub struct EfiConfigurationTable<'a> {
	entries: &'a [EfiConfigurationTableEntry],
}

impl<'a> EfiConfigurationTable<'a> {
	pub(crate) fn new(configuration_table: *const EfiConfigurationTableEntry, configuration_table_size: usize) -> Self {
		Self {
			entries: unsafe {
				from_raw_parts(
					configuration_table,
					configuration_table_size,
				)
			},
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

		/* GUID CONSTANTS */
		// const ACPI_V1_0_TABLE: EfiGuidTuple = (0xeb9d2d30, 0x2d88, 0x11d3, [0x9a, 0x16, 0x00, 0x90, 0x27, 0x3f, 0xc1, 0x4d]);
		// const ACPI_V2_0_TABLE: EfiGuidTuple = (0x8868e871, 0xe4f1, 0x11d3, [0xbc, 0x22, 0x00, 0x80, 0xc7, 0x3c, 0x88, 0x81]);
		// const SAL_SYSTEM_TABLE: EfiGuidTuple = (0xeb9d2d32, 0x2d88, 0x11d3, [0x9a, 0x16, 0x00, 0x90, 0x27, 0x3f, 0xc1, 0x4d]);
		// const SMBIOS_TABLE: EfiGuidTuple = (0xeb9d2d31, 0x2d88, 0x11d3, [0x9a, 0x16, 0x00, 0x90, 0x27, 0x3f, 0xc1, 0x4d]);
		// const SMBIOS_3_TABLE: EfiGuidTuple = (0xf2fd1544, 0x9794, 0x4a2c, [0x99, 0x2e, 0xe5, 0xbb, 0xcf, 0x20, 0xe3, 0x94]);
		// const MPS_TABLE: EfiGuidTuple = (0xeb9d2d2f, 0x2d88, 0x11d3, [0x9a, 0x16, 0x00, 0x90, 0x27, 0x3f, 0xc1, 0x4d]);
	}
}
