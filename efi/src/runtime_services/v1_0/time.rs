use crate::*;

#[repr(C)]
#[derive(Clone,Copy)]
pub(super) struct EfiTimeRaw {
	get_time: extern "efiapi" fn(*mut EfiTimeRepresentation, *mut EfiTimeCapabilities) -> EfiStatus,
	set_time: extern "efiapi" fn(*const EfiTimeRepresentation) -> EfiStatus,
	get_wakeup_time: extern "efiapi" fn(*mut bool, *mut bool, *mut EfiTimeRepresentation) -> EfiStatus,
	set_wakeup_time: extern "efiapi" fn(bool, *const EfiTimeRepresentation) -> EfiStatus,
}

impl EfiTimeRaw {
	pub(super) fn get_time(&self) -> EfiStatusEnum<(EfiTimeRepresentation, EfiTimeCapabilities)> {
		let (mut time, mut capabilities): (EfiTimeRepresentation, EfiTimeCapabilities) = (
			EfiTimeRepresentation::zeroed(),
			EfiTimeCapabilities::zeroed(),
		);

		let result: EfiStatus = (self.get_time)(
			&mut time,
			&mut capabilities
		);

		if result.is_error() {
			return EfiStatusEnum::Error(result.into(), ());
		}

		result.into_enum_data((time, capabilities))
	}

	pub(super) fn set_time(&self, time: &EfiTimeRepresentation) -> EfiStatusEnum {
		(self.set_time)(
			time
		).into_enum()
	}

	pub(super) fn get_wakeup_time(&self) -> EfiStatusEnum<EfiWakeupTime> {
		let mut wakeup_time: EfiWakeupTime = EfiWakeupTime::zeroed();

		(self.get_wakeup_time)(
			&mut wakeup_time.enabled,
			&mut wakeup_time.pending,
			&mut wakeup_time.time
		).into_enum_data(wakeup_time)
	}

	pub(super) fn set_wakeup_time(&self, enabled: bool, time: &EfiTimeRepresentation) -> EfiStatusEnum {
		(self.set_wakeup_time)(
			enabled,
			time
		).into_enum()
	}
}

pub trait EfiTime {
	fn get_time(&self) -> EfiStatusEnum<(EfiTimeRepresentation, EfiTimeCapabilities)>;

	fn set_time(&self, time: &EfiTimeRepresentation) -> EfiStatusEnum;

	fn get_wakeup_time(&self) -> EfiStatusEnum<EfiWakeupTime>;

	fn set_wakeup_time(&self, enabled: bool, time: &EfiTimeRepresentation) -> EfiStatusEnum;
}

#[repr(transparent)]
#[derive(Clone,Copy)]
pub struct EfiDaylight {
	flags: u8,
}

impl EfiDaylight {
	pub fn new() -> Self {
		Self {
			flags: 0,
		}
	}

	pub fn adjust_daylight(&self) -> bool {
		self.flags & 1 == 1
	}

	pub fn set_adjust_daylight(&mut self, value: bool) {
		match value {
			true => self.flags |= 1,
			false => self.flags &= !1,
		}
	}

	pub fn in_daylight(&self) -> bool {
		self.flags & 2 == 2
	}

	pub fn set_in_daylight(&mut self, value: bool) {
		match value {
			true => self.flags |= 2,
			false => self.flags &= !2,
		}
	}
}

impl Default for EfiDaylight {
	fn default() -> Self {
		Self::new()
	}
}

#[repr(transparent)]
#[derive(Clone,Copy)]
pub struct EfiTimeZone {
	time_zone: i16,
}

impl EfiTimeZone {
	pub const fn unspecified_time_zone() -> Self {
		Self {
			time_zone: 0x7FF,
		}
	}

	pub fn new(minute_offset: i16) -> Self {
		Self {
			time_zone: minute_offset,
		}
	}

	pub fn time_zone(&self) -> i16 {
		self.time_zone
	}

	pub fn set_time_zone(&mut self, minute_offset: i16) {
		self.time_zone = minute_offset;
	}
}

#[repr(C)]
#[derive(Clone,Copy)]
pub struct EfiTimeRepresentation {
	pub year: u16,
	pub month: u8,
	pub day: u8,
	pub hour: u8,
	pub minute: u8,
	pub second: u8,
	_padding_1: u8,
	pub nanosecond: u32,
	pub time_zone: EfiTimeZone,
	pub daylight: EfiDaylight,
	_padding_2: u8,
}

impl EfiTimeRepresentation {
	/// Format for date is `(day, month, year)` and for time is `(hour, minute, second, nanosecond)`.
	pub fn new(date: (u8, u8, u16), time: (u8, u8, u8, u32), time_zone: EfiTimeZone, daylight: EfiDaylight) -> Self {
		Self {
			year: date.2,
			month: date.1,
			day: date.0,
			hour: time.0,
			minute: time.1,
			second: time.2,
			_padding_1: 0,
			nanosecond: time.3,
			time_zone,
			daylight,
			_padding_2: 0,
		}
	}

	fn zeroed() -> Self {
		Self {
			year: 0,
			month: 0,
			day: 0,
			hour: 0,
			minute: 0,
			second: 0,
			_padding_1: 0,
			nanosecond: 0,
			time_zone: EfiTimeZone::unspecified_time_zone(),
			daylight: EfiDaylight::new(),
			_padding_2: 0,
		}
	}
}

#[repr(C)]
#[derive(Clone,Copy)]
pub struct EfiTimeCapabilities {
	resolution: u32,
	accuracy: u32,
	sets_to_zero: bool,
}

impl EfiTimeCapabilities {
	fn zeroed() -> Self {
		Self {
			resolution: 0,
			accuracy: 0,
			sets_to_zero: false,
		}
	}

	pub fn resolution(&self) -> u32 {
		self.resolution
	}

	pub fn accuracy(&self) -> u32 {
		self.accuracy
	}

	pub fn sets_to_zero(&self) -> bool {
		self.sets_to_zero
	}
}

#[repr(C)]
#[derive(Clone,Copy)]
pub struct EfiWakeupTime {
	enabled: bool,
	pending: bool,
	time: EfiTimeRepresentation,
}

impl EfiWakeupTime {
	fn zeroed() -> Self {
		Self {
			enabled: false,
			pending: false,
			time: EfiTimeRepresentation::zeroed(),
		}
	}

	pub fn enabled(&self) -> bool {
		self.enabled
	}

	pub fn pending(&self) -> bool {
		self.pending
	}

	pub fn time(&self) -> EfiTimeRepresentation {
		self.time
	}
}
