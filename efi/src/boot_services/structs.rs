use core::ops::Deref;

use crate::{
	table_header::EfiTableHeader,
	boot_services::EfiBootServicesLayout,
};

use super::types::EfiBootServicesLatestLayout;

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
}

impl Deref for EfiBootServices {
	type Target = EfiBootServicesLatestLayout;

	fn deref(&self) -> &<Self as Deref>::Target {
		&self.v1_0
	}
}
