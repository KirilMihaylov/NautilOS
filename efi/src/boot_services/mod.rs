mod v1_0;

pub use v1_0::*;

use crate::{boot_services::EfiBootServicesRevision1x0Raw, table_header::EfiTableHeader};

#[repr(C)]
pub struct EfiBootServices {
    table_header: EfiTableHeader,
    v1_0: EfiBootServicesRevision1x0Raw,
}

impl EfiBootServices {
    pub fn verify_table(&self) -> bool {
        self.table_header.verify_table()
    }

    pub fn header(&self) -> &EfiTableHeader {
        &self.table_header
    }

    pub fn header_mut(&mut self) -> &mut EfiTableHeader {
        &mut self.table_header
    }

    pub fn revision(&self) -> u32 {
        self.table_header.revision()
    }

    pub fn revision_1_0(&self) -> &dyn EfiBootServicesRevision1x0 {
        &self.v1_0
    }

    pub fn revision_1_0_mut(&mut self) -> &mut dyn EfiBootServicesRevision1x0 {
        &mut self.v1_0
    }
}
