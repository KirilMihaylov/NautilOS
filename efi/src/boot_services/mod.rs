mod v1_0;

pub use v1_0::*;

use crate::{
	table_header::EfiTableHeader,
	boot_services::EfiBootServicesRevision_1_0_Raw,
};

#[repr(C)]
pub struct EfiBootServices {
	table_header: EfiTableHeader,
	v1_0: EfiBootServicesRevision_1_0_Raw,
}

impl EfiBootServices {
	pub fn verify_table(&self) -> bool {
		self.table_header.verify_table()
	}

	pub fn header(&self) -> &EfiTableHeader {
		&self.table_header
	}

	pub fn revision(&self) -> u32 {
		self.table_header.revision()
	}
	
	pub fn revision_1_0(&self) -> &dyn EfiBootServicesRevision_1_0 {
		&self.v1_0
	}
}
