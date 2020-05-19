use crate::*;

#[repr(C)]
#[derive(Clone,Copy)]
#[non_exhaustive]
pub enum EfiResetType {
	Cold,
	Warm,
	Shutdown,
	PlatformSpecific,
}

#[repr(C)]
#[derive(Clone,Copy)]
pub(super) struct EfiMiscellaneousRaw {
	get_next_high_monotonic_count: extern "efiapi" fn(*mut u32) -> EfiStatus,
	reset: extern "efiapi" fn(EfiResetType, EfiStatus, usize, VoidPtr) -> !,
}

impl EfiMiscellaneousRaw {
	pub(super) fn get_next_high_monotonic_count(&self) -> EfiStatusEnum<u32> {
		let mut return_value: u32 = 0;

		(self.get_next_high_monotonic_count)(
			&mut return_value
		).into_enum_data(return_value)
	}

	pub(super) fn reset(&self, reset_type: EfiResetType, reset_code: EfiStatus, data: &[u8]) -> ! {
		(self.reset)(
			reset_type,
			reset_code,
			data.len(),
			data.as_ptr() as VoidPtr
		)
	}
}

pub trait EfiMiscellaneous {
	fn get_next_high_monotonic_count(&self) -> EfiStatusEnum<u32>;

	fn reset(&self, reset_type: EfiResetType, reset_code: EfiStatus, data: &[u8]) -> !;
}
