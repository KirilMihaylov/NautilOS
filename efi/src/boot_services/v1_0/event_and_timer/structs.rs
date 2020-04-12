use crate::types::{
	EfiEvent,
	VoidPtr,
};
use crate::status::{
	EfiStatus,
	EfiStatusEnum,
};

use super::{
	enums::{
		EfiEventType,
		EfiTimerDelay,
	},
	types::EfiEventNotifyCallback,
};

use crate::boot_services::v1_0::task_priority::types::EfiTaskPriorityLevel;

#[repr(C)]
pub struct EfiEventAndTimer {
	create_event: extern "efiapi" fn(event_type: EfiEventType, tpl: EfiTaskPriorityLevel, notify_function: EfiEventNotifyCallback, notify_context: VoidPtr, event: *mut EfiEvent) -> EfiStatus,
	set_timer: extern "efiapi" fn(event: EfiEvent, timer_type: EfiTimerDelay, trigger_time: u64) -> EfiStatus,
	wait_for_event: extern "efiapi" fn(number_of_entries: usize, *const EfiEvent, *mut usize) -> EfiStatus,
	signal_event: extern "efiapi" fn(event: EfiEvent) -> EfiStatus,
	close_event: extern "efiapi" fn(event: EfiEvent) -> EfiStatus,
	check_event: extern "efiapi" fn(event: EfiEvent) -> EfiStatus,
}

impl EfiEventAndTimer {
	pub fn create_event<T>(&self, event_type: EfiEventType, tpl: EfiTaskPriorityLevel, notify: Option<(EfiEventNotifyCallback, &T)>) -> EfiStatusEnum<EfiEvent> {
		let mut event: EfiEvent = 0 as EfiEvent;

		let (notify_function, notify_context): (EfiEventNotifyCallback, VoidPtr) = {
			match notify {
				None => (unsafe { *(&0usize as *const usize as *const EfiEventNotifyCallback) }, 0 as VoidPtr),
				Some((notify_function, notify_context)) => (notify_function, notify_context as *const T as VoidPtr)
			}
		};
		
		(self.create_event)(
			event_type,
			tpl,
			notify_function,
			notify_context,
			&mut event
		).into_enum_data(event)
	}

	pub fn set_timer(&self, event: EfiEvent, timer_type: EfiTimerDelay, trigger_time: u64) -> EfiStatusEnum {
		(self.set_timer)(
			event,
			timer_type,
			trigger_time
		).into_enum()
	}

	pub fn wait_for_event(&self, events: &[EfiEvent]) -> EfiStatusEnum<usize> {
		let mut index: usize = 0;
		
		(self.wait_for_event)(
			events.len(),
			events.as_ptr(),
			&mut index
		).into_enum_data(index)
	}

	pub fn signal_event(&self, event: EfiEvent) -> EfiStatusEnum {
		(self.signal_event)(
			event
		).into_enum()
	}

	pub fn close_event(&self, event: EfiEvent) -> EfiStatusEnum {
		(self.close_event)(
			event
		).into_enum()
	}

	pub fn check_event(&self, event: EfiEvent) -> EfiStatusEnum {
		(self.check_event)(
			event
		).into_enum()
	}
}
