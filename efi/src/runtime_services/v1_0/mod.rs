pub mod miscellaneous;
pub mod time;
pub mod variable;
pub mod virtual_memory;

use crate::*;

use miscellaneous::*;
use time::*;
use variable::*;
use virtual_memory::*;

use crate::boot_services::memory::EfiMemoryDescriptors;

#[repr(C)]
#[derive(Clone,Copy)]
pub struct EfiRuntimeServicesRevision_1_0_Raw {
	time: EfiTimeRaw,
	variable: EfiVariableRaw,
	virtual_memory: EfiVirtualMemoryRaw,
	miscellaneous: EfiMiscellaneousRaw,
}

#[allow(non_camel_case_types)]
pub trait EfiRuntimeServicesRevision_1_0: EfiTime + EfiVariable + EfiVirtualMemory + EfiMiscellaneous {}
impl EfiRuntimeServicesRevision_1_0 for EfiRuntimeServicesRevision_1_0_Raw {}

impl EfiMiscellaneous for EfiRuntimeServicesRevision_1_0_Raw {
	fn get_next_high_monotonic_count(&self) -> EfiStatusEnum<u32> {
		self.miscellaneous.get_next_high_monotonic_count()
	}

	fn reset(&self, reset_type: EfiResetType, reset_code: EfiStatus, data: &[u8]) -> ! {
		self.miscellaneous.reset(reset_type, reset_code, data)
	}
}

impl EfiTime for EfiRuntimeServicesRevision_1_0_Raw {
	fn get_time(&self) -> EfiStatusEnum<(EfiTimeRepresentation, EfiTimeCapabilities)> {
		self.time.get_time()
	}

	fn set_time(&self, time: &EfiTimeRepresentation) -> EfiStatusEnum {
		self.time.set_time(time)
	}

	fn get_wakeup_time(&self) -> EfiStatusEnum<EfiWakeupTime> {
		self.time.get_wakeup_time()
	}

	fn set_wakeup_time(&self, enabled: bool, time: &EfiTimeRepresentation) -> EfiStatusEnum {
		self.time.set_wakeup_time(enabled, time)
	}
}

impl EfiVariable for EfiRuntimeServicesRevision_1_0_Raw {
	fn get_variable(&self, variable_name: &[u16], vendor_guid: &EfiGuid, data: Option<&mut [u8]>) -> EfiStatusEnum<(usize, EfiVariableAttributes), (usize, EfiVariableAttributes)> {
		self.variable.get_variable(variable_name, vendor_guid, data)
	}

	fn get_next_variable_name(&self, variable_name: &mut [u16], vendor_guid: &mut EfiGuid) -> EfiStatusEnum<(), usize> {
		self.variable.get_next_variable_name(variable_name, vendor_guid)
	}

	fn set_variable(&self, variable_name: &[u16], vendor_guid: &EfiGuid, attributes: &EfiVariableAttributes, data: &[u8]) -> EfiStatusEnum {
		self.variable.set_variable(variable_name, vendor_guid, attributes, data)
	}
}

impl EfiVirtualMemory for EfiRuntimeServicesRevision_1_0_Raw {
	fn set_virtual_address_map(&self, memory_map: EfiMemoryDescriptors) -> EfiStatusEnum {
		self.virtual_memory.set_virtual_address_map(memory_map)
	}

	fn convert_pointer(&self, pointer: &mut VoidPtr, flags_builder: EfiConvertPointerFlagsBuilder) -> EfiStatusEnum {
		self.virtual_memory.convert_pointer(pointer, flags_builder)
	}
}
