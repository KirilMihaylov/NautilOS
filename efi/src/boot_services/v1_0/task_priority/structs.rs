use crate::{
	status::{
		EfiStatus,
		EfiStatusEnum,
	},
	types::Void,
};

use super::types::EfiTaskPriorityLevel;

#[repr(C)]
pub struct EfiTaskPriority {
	raise_tpl: extern "efiapi" fn(EfiTaskPriorityLevel) -> EfiStatus,
	restore_tpl: extern "efiapi" fn(EfiTaskPriorityLevel) -> Void,
}

impl EfiTaskPriority {
	pub fn raise_priority_level(&self, new_priority_level: EfiTaskPriorityLevel) -> EfiStatusEnum {
		(self.raise_tpl)(
			new_priority_level
		).into_enum()
	}

	pub fn restore_priority_level(&self, old_priority_level: EfiTaskPriorityLevel) {
		(self.restore_tpl)(
			old_priority_level
		);
	}
}
