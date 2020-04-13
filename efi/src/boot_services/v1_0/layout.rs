use crate::boot_services::v1_0::{
	event_and_timer::structs::EfiEventAndTimer,
	image::structs::EfiImage,
	memory::structs::EfiMemory,
	miscellaneous::structs::EfiMiscellaneous,
	protocol_handler::structs::EfiProtocolHandler,
	task_priority::structs::EfiTaskPriority,
};

#[repr(C)]
pub struct EfiBootServicesLayout {
	pub task_priority: EfiTaskPriority,
	pub memory: EfiMemory,
	pub event_and_timer: EfiEventAndTimer,
	pub protocol_handler: EfiProtocolHandler,
	pub image: EfiImage,
	pub miscellaneous: EfiMiscellaneous,
}
