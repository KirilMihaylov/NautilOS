use core::ops::Deref;

use crate::{
	table_header::EfiTableHeader,
	boot_services::EfiBootServicesLayout,
};

#[repr(C)]
pub struct EfiBootServices {
	table_header: EfiTableHeader,
	v1_0: EfiBootServicesLayout,
}

impl EfiBootServices {
	pub fn verify_table(&self) -> bool {
		self.table_header.verify_table()
	}

	pub fn header<'a>(&'a self) -> &'a EfiTableHeader {
		&self.table_header
	}

	pub fn revision(&self) -> u32 {
		self.table_header.revision()
	}
}

impl Deref for EfiBootServices {
	type Target = EfiBootServicesLayout;

	fn deref(&self) -> &<Self as Deref>::Target {
		&self.v1_0
	}
}
