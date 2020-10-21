use {
    crate::{guid::EfiGuid, types::VoidPtr},
    core::{
        fmt::{Debug, Formatter, Result as FmtResult},
        slice::from_raw_parts,
    },
    efi_interops::{traits, types},
};

#[repr(transparent)]
pub struct EfiConfigurationTable {
    entries: &'static [EfiConfigurationTableEntry],
}

impl EfiConfigurationTable {
    pub(crate) unsafe fn new(
        configuration_table: *const EfiConfigurationTableEntry,
        configuration_table_size: usize,
    ) -> Self {
        Self {
            entries: from_raw_parts(configuration_table, configuration_table_size),
        }
    }

    pub fn get_all(&self) -> &'static [EfiConfigurationTableEntry] {
        self.entries
    }

    pub fn get_by_guid(&self, guid: EfiGuid) -> EfiConfigurationTableIterator {
        EfiConfigurationTableIterator {
            entries: self.entries,
            guid,
        }
    }
}

#[derive(Clone, Copy)]
pub struct EfiConfigurationTableIterator {
    entries: &'static [EfiConfigurationTableEntry],
    guid: EfiGuid,
}

impl Iterator for EfiConfigurationTableIterator {
    type Item = &'static EfiConfigurationTableEntry;

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

    pub fn get_as<T: traits::EfiConfigurationTable>(&self) -> Option<&'static T> {
        if T::guid() == self.vendor_guid {
            Some(unsafe { &*(self.vendor_table as *const T) })
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
    pub const fn version(&self) -> u16 {
        self.version
    }

    pub const fn length(&self) -> u16 {
        self.length
    }

    pub const fn get_time_supported(&self) -> bool {
        self.runtime_services_supported & 1 == 1
    }

    pub const fn set_time_supported(&self) -> bool {
        self.runtime_services_supported & 2 == 2
    }

    pub const fn get_wakeup_time_supported(&self) -> bool {
        self.runtime_services_supported & 4 == 4
    }

    pub const fn set_wakeup_time_supported(&self) -> bool {
        self.runtime_services_supported & 8 == 8
    }

    pub const fn get_variable_supported(&self) -> bool {
        self.runtime_services_supported & 0x10 == 0x10
    }

    pub const fn get_next_variable_supported(&self) -> bool {
        self.runtime_services_supported & 0x20 == 0x20
    }

    pub const fn set_variable_supported(&self) -> bool {
        self.runtime_services_supported & 0x40 == 0x40
    }

    pub const fn set_virtual_address_map_supported(&self) -> bool {
        self.runtime_services_supported & 0x80 == 0x80
    }

    pub const fn convert_pointer_supported(&self) -> bool {
        self.runtime_services_supported & 0x100 == 0x100
    }

    pub const fn get_next_high_monotonic_count_supported(&self) -> bool {
        self.runtime_services_supported & 0x200 == 0x200
    }

    pub const fn reset_system_supported(&self) -> bool {
        self.runtime_services_supported & 0x400 == 0x400
    }

    pub const fn update_capsule_supported(&self) -> bool {
        self.runtime_services_supported & 0x800 == 0x800
    }

    pub const fn query_capsule_capabilities_supported(&self) -> bool {
        self.runtime_services_supported & 0x1000 == 0x1000
    }

    pub const fn query_variable_info_supported(&self) -> bool {
        self.runtime_services_supported & 0x2000 == 0x2000
    }
}

unsafe impl traits::EfiConfigurationTable for EfiRTPropertiesTable {
    fn guid() -> types::EfiGuid {
        (
            0xeb66918a,
            0x7eef,
            0x402a,
            [0x84, 0x2e, 0x93, 0x1d, 0x21, 0xc3, 0x8a, 0xe9],
        )
    }
}

impl Debug for EfiRTPropertiesTable {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(
            f,
            "{{ \
                version: {}, \
                length: {}, \
                Time: {{ get: {}, set: {} }}, \
                Wakeup Time: {{ get: {}, set: {} }}, \
                Variable: {{ get: {}, set: {} }}, \
                Get Next Variable: {}, \
                Set Virtual Address Map: {}, \
                Convert Pointer: {}, \
                Get Next High Monotonic Count: {}, \
                Reset System: {}, \
                Update Capsule: {}, \
                Query Capsule Capabilities: {}, \
                Query Variable Info: {} \
            }}",
            self.version,
            self.length,
            self.get_time_supported(),
            self.set_time_supported(),
            self.get_wakeup_time_supported(),
            self.set_wakeup_time_supported(),
            self.get_variable_supported(),
            self.set_variable_supported(),
            self.get_next_variable_supported(),
            self.set_virtual_address_map_supported(),
            self.convert_pointer_supported(),
            self.get_next_high_monotonic_count_supported(),
            self.reset_system_supported(),
            self.update_capsule_supported(),
            self.query_capsule_capabilities_supported(),
            self.query_variable_info_supported(),
        )
    }
}
