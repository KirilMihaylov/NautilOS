use core::mem::MaybeUninit;

use crate::{
	status::{
		EfiStatus,
		EfiStatusEnum,
	},
};

#[repr(C)]
pub struct EfiTime {
	get_time: extern "efiapi" fn(*mut EfiTimeRepresentation, *mut EfiTimeCapabilities) -> EfiStatus,
	set_time: extern "efiapi" fn(*const EfiTimeRepresentation) -> EfiStatus,
	get_wakeup_time: extern "efiapi" fn(*mut bool, *mut bool, *mut EfiTimeRepresentation) -> EfiStatus,
	set_wakeup_time: extern "efiapi" fn(bool, *const EfiTimeRepresentation) -> EfiStatus,
}

impl EfiTime {
	pub fn get_time(&self) -> EfiStatusEnum<(EfiTimeRepresentation, EfiTimeCapabilities)> {
		let (mut time, mut capabilities): (EfiTimeRepresentation, EfiTimeCapabilities) = unsafe {
			MaybeUninit::zeroed().assume_init()
		};

		let result: EfiStatus = (self.get_time)(
			&mut time,
			&mut capabilities
		);

		if result.is_error() {
			return EfiStatusEnum::Error(result.into(), ());
		}

		result.into_enum_data((time, capabilities))
	}

	pub fn set_time(&self, time: &EfiTimeRepresentation) -> EfiStatusEnum {
		(self.set_time)(
			time
		).into_enum()
	}

	pub fn get_wakeup_time(&self) -> EfiStatusEnum<EfiGetWakeupTime> {
		let mut wakeup_time: EfiGetWakeupTime = unsafe {
			MaybeUninit::zeroed().assume_init()
		};

		(self.get_wakeup_time)(
			&mut wakeup_time.enabled,
			&mut wakeup_time.pending,
			&mut wakeup_time.time
		).into_enum_data(wakeup_time)
	}

	pub fn set_wakeup_time(&self, enabled: bool, time: &EfiTimeRepresentation) -> EfiStatusEnum {
		(self.set_wakeup_time)(
			enabled,
			time
		).into_enum()
	}
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

#[repr(transparent)]
#[derive(Clone,Copy)]
pub struct EfiTimeZone {
	time_zone: i16,
}

impl EfiTimeZone {
	pub const fn unspecified_time_zone() -> i16 {
		0x7FF
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
	pub fn new(year: u16, month: u8, day: u8, hour: u8, minute: u8, second: u8, nanosecond: u32, time_zone: EfiTimeZone, daylight: EfiDaylight) -> Self {
		Self {
			year: year,
			month: month,
			day: day,
			hour: hour,
			minute: minute,
			second: second,
			_padding_1: 0,
			nanosecond: nanosecond,
			time_zone: time_zone,
			daylight: daylight,
			_padding_2: 0,
		}
	}
}

#[repr(C)]
#[derive(Clone,Copy)]
pub struct EfiTimeCapabilities {
	pub resolution: u32,
	pub accuracy: u32,
	pub sets_to_zero: bool,
}

pub struct EfiGetWakeupTime {
	enabled: bool,
	pending: bool,
	time: EfiTimeRepresentation,
}

impl EfiGetWakeupTime {
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
