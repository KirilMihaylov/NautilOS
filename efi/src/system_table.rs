use crate::{
	types::{
		EfiHandle,
		Void,
	},
	table_header::EfiTableHeader,
	utilities::string_from_raw,
	protocols::console::{
		simple_text_input_protocol::EfiSimpleTextInputProtocol,
		simple_text_output_protocol::EfiSimpleTextOutputProtocol,
	},
	boot_services::structs::EfiBootServices,
	runtime_services::structs::EfiRuntimeServices,
	configuration_table::{
		EfiConfigurationTable,
		EfiConfigurationTableEntry,
	},
};

#[repr(C)]
pub struct EfiSystemTable {
	table_header: EfiTableHeader,
	firmware_vendor: *const u16,
	firmware_revision: u32,
	console_in_handle: EfiHandle,
	con_in: *mut EfiSimpleTextInputProtocol,
	console_out_handle: EfiHandle,
	con_out: *mut EfiSimpleTextOutputProtocol,
	standart_error_handle: EfiHandle,
	std_err: *mut EfiSimpleTextOutputProtocol,
	runtime_services: *mut EfiRuntimeServices,
	boot_services: *mut EfiBootServices,
	configuration_tables_count: usize,
	configuration_tables: *mut EfiConfigurationTableEntry,
}

impl EfiSystemTable {
	pub fn verify_table(&self) -> bool {
		self.table_header.verify_table() && self.boot_services().verify_table() && self.runtime_services().verify_table()
	}

	pub fn header<'a>(&'a self) -> &'a EfiTableHeader {
		&self.table_header
	}

	pub fn revision(&self) -> u32 {
		self.table_header.revision()
	}

	pub fn firmware_vendor<'a>(&'a self) -> Result<&'a [u16], ()> {
		unsafe {
			string_from_raw(self.firmware_vendor)
		}
	}

	pub fn firmware_revision(&self) -> u32 {
		self.firmware_revision
	}

	pub fn console_in_handle(&self) -> EfiHandle {
		self.console_in_handle
	}

	pub fn con_in<'a>(&'a self) -> &'a mut EfiSimpleTextInputProtocol {
		unsafe {
			&mut *self.con_in
		}
	}

	pub fn console_out_handle(&self) -> EfiHandle {
		self.console_out_handle
	}

	pub fn con_out<'a>(&'a self) -> &'a mut EfiSimpleTextOutputProtocol {
		unsafe {
			&mut *self.con_out
		}
	}

	pub fn standart_error_handle(&self) -> EfiHandle {
		self.standart_error_handle
	}

	pub fn std_err<'a>(&'a self) -> &'a mut EfiSimpleTextOutputProtocol {
		unsafe {
			&mut *self.std_err
		}
	}

	pub fn runtime_services<'a>(&'a self) -> &'a mut EfiRuntimeServices {
		unsafe {
			&mut *self.runtime_services
		}
	}

	pub fn boot_services<'a>(&'a self) -> &'a mut EfiBootServices {
		unsafe {
			&mut *self.boot_services
		}
	}

	pub fn configuration_tables<'a>(&'a self) -> EfiConfigurationTable<'a> {
		unsafe {
			EfiConfigurationTable::new(self.configuration_tables, self.configuration_tables_count)
		}
	}

	/// Returns a [`&mut &Void`] that can be passed to [`convert_pointer`].
	/// 
	/// [`&mut &Void`]: types/type.Void.html
	/// [`convert_pointer`]: runtime_services/virtual_memory/structs/struct.EfiVirtualMemory.html#method.convert_pointer
	pub fn configuration_tables_pointer(&self) -> &mut &Void {
		unsafe {
			&mut*(&self.configuration_tables as *const *mut EfiConfigurationTableEntry as *mut &Void)
		}
	}
}
