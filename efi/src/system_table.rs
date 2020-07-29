use crate::{
    boot_services::EfiBootServices,
    protocols::console::{EfiSimpleTextInputProtocol, EfiSimpleTextOutputProtocol},
    runtime_services::EfiRuntimeServices,
    utilities::string_from_raw,
    *,
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
        self.table_header.verify_table()
            && self.boot_services().verify_table()
            && self.runtime_services().verify_table()
    }

    pub fn header(&self) -> &EfiTableHeader {
        &self.table_header
    }

    pub fn revision(&self) -> u32 {
        self.table_header.revision()
    }

    pub fn firmware_vendor(&self) -> Result<&[u16], ()> {
        unsafe { string_from_raw(self.firmware_vendor) }
    }

    pub fn firmware_revision(&self) -> u32 {
        self.firmware_revision
    }

    pub fn console_in_handle(&self) -> EfiHandle {
        self.console_in_handle
    }

    pub fn con_in(&self) -> Option<&EfiSimpleTextInputProtocol> {
        if self.con_in.is_null() {
            None
        } else {
            Some(unsafe { &*self.con_in })
        }
    }

    pub fn con_in_mut(&mut self) -> Option<&mut EfiSimpleTextInputProtocol> {
        if self.con_in.is_null() {
            None
        } else {
            Some(unsafe { &mut *self.con_in })
        }
    }

    pub fn console_out_handle(&self) -> EfiHandle {
        self.console_out_handle
    }

    pub fn con_out(&self) -> Option<&EfiSimpleTextOutputProtocol> {
        if self.con_out.is_null() {
            None
        } else {
            Some(unsafe { &*self.con_out })
        }
    }

    pub fn con_out_mut(&mut self) -> Option<&mut EfiSimpleTextOutputProtocol> {
        if self.con_out.is_null() {
            None
        } else {
            Some(unsafe { &mut *self.con_out })
        }
    }

    pub fn standart_error_handle(&self) -> EfiHandle {
        self.standart_error_handle
    }

    pub fn std_err(&self) -> Option<&EfiSimpleTextOutputProtocol> {
        if self.std_err.is_null() {
            None
        } else {
            Some(unsafe { &*self.std_err })
        }
    }

    pub fn std_err_mut(&mut self) -> Option<&mut EfiSimpleTextOutputProtocol> {
        if self.std_err.is_null() {
            None
        } else {
            Some(unsafe { &mut *self.std_err })
        }
    }

    pub fn runtime_services(&self) -> &EfiRuntimeServices {
        unsafe { &*self.runtime_services }
    }

    pub fn runtime_services_mut(&mut self) -> &mut EfiRuntimeServices {
        unsafe { &mut *self.runtime_services }
    }

    pub fn boot_services(&self) -> &EfiBootServices {
        unsafe { &*self.boot_services }
    }

    pub fn boot_services_mut(&mut self) -> &mut EfiBootServices {
        unsafe { &mut *self.boot_services }
    }

    pub fn configuration_tables<'a>(&self) -> EfiConfigurationTable<'a> {
        unsafe {
            EfiConfigurationTable::new(self.configuration_tables, self.configuration_tables_count)
        }
    }

    /// Returns a [`&mut &Void`] that can be passed to [`convert_pointer`].
    ///
    /// [`&mut &Void`]: types/type.Void.html
    /// [`convert_pointer`]: runtime_services/virtual_memory/structs/struct.EfiVirtualMemory.html#method.convert_pointer
    pub fn configuration_tables_pointer(&mut self) -> &mut &Void {
        unsafe {
            &mut *(&mut self.configuration_tables as *mut *mut EfiConfigurationTableEntry
                as *mut &Void)
        }
    }
}
