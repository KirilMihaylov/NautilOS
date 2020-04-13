use crate::status::{
	EfiStatus,
	EfiStatusEnum,
};

#[repr(C)]
pub struct EfiMiscellaneous {
	get_next_monotonic_count: extern "efiapi" fn(count: *mut u64) -> EfiStatus,
	stall: extern "efiapi" fn(microseconds: usize) -> EfiStatus,
	set_watchdog_timer: extern "efiapi" fn(timeout: usize, watchdog_code: u64, data_size: usize, watchdog_data: *const u16) -> EfiStatus,
}

impl EfiMiscellaneous {
	pub fn get_next_monotonic_count(&self) -> EfiStatusEnum<(u32, u32)> {
		let mut count: u64 = 0;

		(self.get_next_monotonic_count)(
			&mut count
		).into_enum_data(
			(
				(count >> 32) as u32,
				count as u32
			)
		)
	}

	pub fn stall(&self, microseconds: usize) -> EfiStatusEnum {
		(self.stall)(
			microseconds
		).into_enum()
	}

	pub fn set_watchdog_timer(&self, timeout: usize, watchdog_code: u64, watchdog_data: Option<&[u16]>) -> EfiStatusEnum {
		let (watchdog_data_ptr, watchdog_data_len): (*const u16, usize) = if let Some(watchdog_data) = watchdog_data {
			(
				watchdog_data.as_ptr(),
				watchdog_data.len() * 2
			)
		} else {
			(
				0 as _,
				0
			)
		};

		(self.set_watchdog_timer)(
			timeout,
			watchdog_code,
			watchdog_data_len,
			watchdog_data_ptr
		).into_enum()
	}
}
