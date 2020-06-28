use crate::types::EfiStatusRaw;

#[repr(transparent)]
pub struct EfiStatus(EfiStatusRaw);

impl EfiStatus {
	pub fn success() -> Self {
		Self(0)
	}

	pub fn warning(code: EfiStatusRaw) -> Self {
		Self((code << 1) >> 1)
	}

	pub fn error(code: EfiStatusRaw) -> Self {
		Self(1usize.rotate_right(1) | code)
	}

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

	pub fn get_warning(&self) -> EfiStatusWarning {
		use EfiStatusWarning::*;

		if self.is_warning() {
			match self.0 {
				1 => EfiWarnUnknownGlyph,
				2 => EfiWarnDeleteFailure,
				3 => EfiWarnWriteFailure,
				4 => EfiWarnBufferTooSmall,
				5 => EfiWarnStaleData,
				6 => EfiWarnFileSystem,
				7 => EfiWarnResetRequired,

				_ => UnknownWarning,
			}
		} else {
			NoWarning
		}
	}

	pub fn get_error(&self) -> EfiStatusError {
		use EfiStatusError::*;

		if self.is_error() {
			match (self.0 << 1) >> 1 {
				1 => EfiLoadError,
				2 => EfiInvalidParameter,
				3 => EfiUnsupported,
				4 => EfiBadBufferSize,
				5 => EfiBufferTooSmall,
				6 => EfiNotReady,
				7 => EfiDeviceError,
				8 => EfiWriteProtected,
				9 => EfiOutOfResources,
				10 => EfiVolumeCorrupted,
				11 => EfiVolumeFull,
				12 => EfiNoMedia,
				13 => EfiMediaChanged,
				14 => EfiNotFound,
				15 => EfiAccessDenied,
				16 => EfiNoResponse,
				17 => EfiNoMapping,
				18 => EfiTimeout,
				19 => EfiNotStarted,
				20 => EfiAlreadyStarted,
				21 => EfiAborted,
				22 => EfiIcmpError,
				23 => EfiTftpError,
				24 => EfiProtocolError,
				25 => EfiIncompatibleVersion,
				26 => EfiSecurityViolation,
				27 => EfiCrcError,
				28 => EfiEndOfMedia,
				31 => EfiEndOfFile,
				32 => EfiInvalidLanguage,
				33 => EfiCompromisedData,
				34 => EfiIpAddressConflict,
				35 => EfiHttpError,

				_ => UnknownError,
			}
		} else {
			NoError
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

#[must_use = "this type's value may contain information about an error that occured"]
pub enum EfiStatusEnum<T = (), E = ()> {
	Success(T),
	Warning(EfiStatusRaw, T),
	Error(EfiStatusRaw, E),
}

impl EfiStatusEnum {
	pub fn is_success(&self) -> bool {
		if let Self::Success(_) = self {
			true
		} else {
			false
		}
	}

	pub fn is_warning(&self) -> bool {
		if let Self::Warning(_, _) = self {
			true
		} else {
			false
		}
	}

	pub fn is_error(&self) -> bool {
		if let Self::Error(_, _) = self {
			true
		} else {
			false
		}
	}
}

#[non_exhaustive]
pub enum EfiStatusWarning {
	NoWarning,

	UnknownWarning,

	EfiWarnUnknownGlyph,
	EfiWarnDeleteFailure,
	EfiWarnWriteFailure,
	EfiWarnBufferTooSmall,
	EfiWarnStaleData,
	EfiWarnFileSystem,
	EfiWarnResetRequired,
}

#[non_exhaustive]
pub enum EfiStatusError {
	NoError,

	UnknownError,

	EfiLoadError,
	EfiInvalidParameter,
	EfiUnsupported,
	EfiBadBufferSize,
	EfiBufferTooSmall,
	EfiNotReady,
	EfiDeviceError,
	EfiWriteProtected,
	EfiOutOfResources,
	EfiVolumeCorrupted,
	EfiVolumeFull,
	EfiNoMedia,
	EfiMediaChanged,
	EfiNotFound,
	EfiAccessDenied,
	EfiNoResponse,
	EfiNoMapping,
	EfiTimeout,
	EfiNotStarted,
	EfiAlreadyStarted,
	EfiAborted,
	EfiIcmpError,
	EfiTftpError,
	EfiProtocolError,
	EfiIncompatibleVersion,
	EfiSecurityViolation,
	EfiCrcError,
	EfiEndOfMedia,
	EfiEndOfFile,
	EfiInvalidLanguage,
	EfiCompromisedData,
	EfiIpAddressConflict,
	EfiHttpError,
}
