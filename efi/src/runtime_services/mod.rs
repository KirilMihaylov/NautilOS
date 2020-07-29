mod v1_0;

pub use v1_0::*;

use crate::{runtime_services::EfiRuntimeServicesRevision_1_0_Raw, *};

#[repr(C)]
pub struct EfiRuntimeServices {
    table_header: EfiTableHeader,
    v1_0: EfiRuntimeServicesRevision_1_0_Raw,
}

impl EfiRuntimeServices {
    pub fn verify_table(&self) -> bool {
        self.table_header.verify_table()
    }

    pub fn header(&self) -> &EfiTableHeader {
        &self.table_header
    }

    pub fn revision(&self) -> u32 {
        self.table_header.revision()
    }

    pub fn revision_1_0(&self) -> &dyn EfiRuntimeServicesRevision_1_0 {
        &self.v1_0
    }
}
