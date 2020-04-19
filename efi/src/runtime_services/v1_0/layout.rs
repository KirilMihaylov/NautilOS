use crate::runtime_services::v1_0::{
	miscellaneous::structs::EfiMiscellaneous,
	time::structs::EfiTime,
	variable::structs::EfiVariable,
	virtual_memory::structs::EfiVirtualMemory,
};

#[repr(C)]
pub struct EfiRuntimeServicesLayout {
	pub time: EfiTime,
	pub variable: EfiVariable,
	pub virtual_memory: EfiVirtualMemory,
	pub miscellaneous: EfiMiscellaneous,
}