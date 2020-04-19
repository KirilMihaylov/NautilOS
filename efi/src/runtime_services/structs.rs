use core::ops::Deref;

use crate::{
	table_header::EfiTableHeader,
	runtime_services::EfiRuntimeServicesLayout,
};

#[repr(C)]
pub struct EfiRuntimeServices {
	table_header: EfiTableHeader,
	v1_0: EfiRuntimeServicesLayout,
}

impl EfiRuntimeServices {
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

impl Deref for EfiRuntimeServices {
	type Target = EfiRuntimeServicesLayout;

	fn deref(&self) -> &<Self as Deref>::Target {
		&self.v1_0
	}
}
