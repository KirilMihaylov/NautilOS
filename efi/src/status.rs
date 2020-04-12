use crate::types::EfiStatusRaw;

#[repr(transparent)]
pub struct EfiStatus(EfiStatusRaw);

impl EfiStatus {
	pub fn is_success(&self) -> bool {
		if self.0 == 0 {
			true /* Success */
		} else {
			false /* Warning or Error */
		}
	}

	pub fn is_warning(&self) -> bool {
		if self.is_success() {
			false /* Success */
		} else if self.is_error() {
			false /* Error */
		} else {
			true /* Warning */
		}
	}

	pub fn is_error(&self) -> bool {
		if self.0.leading_zeros() == 0 {
			true /* Error */
		} else {
			false /* Success or Warning */
		}
	}

	pub fn into_enum(&self) -> EfiStatusEnum {
		use EfiStatusEnum::*;

		if self.is_success() {
			Success(())
		} else if self.is_warning() {
			Warning(self.0, ())
		} else {
			Error(self.0, ())
		}
	}

	pub fn into_enum_data<S>(&self, data: S) -> EfiStatusEnum<S> {
		use EfiStatusEnum::*;

		if self.is_success() {
			Success(data)
		} else if self.is_warning() {
			Warning(self.0, data)
		} else {
			Error(self.0, ())
		}
	}

	pub fn into_enum_data_error<S, E>(&self, data: S, error_data: E) -> EfiStatusEnum<S, E> {
		use EfiStatusEnum::*;

		if self.is_success() {
			Success(data)
		} else if self.is_warning() {
			Warning(self.0, data)
		} else {
			Error(self.0, error_data)
		}
	}
}

impl Clone for EfiStatus {
	fn clone(&self) -> Self {
		Self(self.0)
	}
}

impl Copy for EfiStatus {}

impl From<EfiStatusRaw> for EfiStatus {
	fn from(data: EfiStatusRaw) -> Self {
		Self(data)
	}
}

impl From<EfiStatus> for EfiStatusRaw {
	fn from(data: EfiStatus) -> Self {
		data.0
	}
}

pub enum EfiStatusEnum<T = (), E = ()> {
	Success(T),
	Warning(EfiStatusRaw, T),
	Error(EfiStatusRaw, E),
}
