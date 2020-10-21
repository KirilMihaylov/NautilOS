use core::fmt::{Debug, Display, Formatter, Result as FmtResult};

use crate::types::EfiStatusRaw;

#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct EfiStatus(EfiStatusRaw);

impl EfiStatus {
    pub const fn success() -> Self {
        Self(0)
    }

    pub const fn warning(code: EfiStatusRaw) -> Self {
        Self((code << 1) >> 1)
    }

    pub const fn error(code: EfiStatusRaw) -> Self {
        Self(1usize.rotate_right(1) | code)
    }

    pub const fn is_success(&self) -> bool {
        /* 0 => Success */
        /* _ => Warning or Error */
        self.0 == 0
    }

    pub const fn is_warning(&self) -> bool {
        !(self.is_success() || self.is_error())
    }

    pub const fn is_error(&self) -> bool {
        self.0.leading_zeros() == 0
    }

    pub const fn into_enum(self) -> EfiStatusEnum {
        use EfiStatusEnum::*;

        if self.is_success() {
            Success(())
        } else if self.is_warning() {
            Warning(self.get_warning(), ())
        } else {
            Error(self.get_error(), ())
        }
    }

    pub fn into_enum_data<S, F: FnOnce() -> S>(self, data: F) -> EfiStatusEnum<S> {
        use EfiStatusEnum::*;

        if self.is_success() {
            Success(data())
        } else if self.is_warning() {
            Warning(self.get_warning(), data())
        } else {
            Error(self.get_error(), ())
        }
    }

    pub fn into_enum_data_error<S, Fs: FnOnce() -> S, E, Fe: FnOnce() -> E>(
        self,
        success: Fs,
        error: Fe,
    ) -> EfiStatusEnum<S, E> {
        use EfiStatusEnum::*;

        if self.is_success() {
            Success(success())
        } else if self.is_warning() {
            Warning(self.get_warning(), success())
        } else {
            Error(self.get_error(), error())
        }
    }

    pub const fn get_warning(&self) -> EfiStatusWarning {
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

                _ => UnknownWarning(self.0),
            }
        } else {
            NoWarning
        }
    }

    pub const fn get_error(&self) -> EfiStatusError {
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

                _ => UnknownError(self.0),
            }
        } else {
            NoError
        }
    }
}

impl Display for EfiStatus {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "{}", self.into_enum(),)
    }
}

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
#[derive(Debug, Copy)]
pub enum EfiStatusEnum<T = (), E = ()> {
    Success(T),
    Warning(EfiStatusWarning, T),
    Error(EfiStatusError, E),
}

impl<T, E> EfiStatusEnum<T, E> {
    pub const fn is_success(&self) -> bool {
        matches!(self, Self::Success(_))
    }

    pub const fn is_warning(&self) -> bool {
        matches!(self, Self::Warning(_, _))
    }

    pub const fn is_error(&self) -> bool {
        matches!(self, Self::Error(_, _))
    }

    pub fn map<U, F: FnOnce(T) -> U>(self, f: F) -> EfiStatusEnum<U, E> {
        match self {
            Self::Success(data) => EfiStatusEnum::Success(f(data)),
            Self::Warning(status, data) => EfiStatusEnum::Warning(status, f(data)),
            Self::Error(status, data) => EfiStatusEnum::Error(status, data),
        }
    }

    pub fn map_error<U, F: FnOnce(E) -> U>(self, f: F) -> EfiStatusEnum<T, U> {
        match self {
            Self::Success(data) => EfiStatusEnum::Success(data),
            Self::Warning(status, data) => EfiStatusEnum::Warning(status, data),
            Self::Error(status, data) => EfiStatusEnum::Error(status, f(data)),
        }
    }

    pub fn unfold(self) -> Result<(EfiStatusWarning, T), (EfiStatusError, E)> {
        match self {
            Self::Success(data) => Ok((EfiStatusWarning::NoWarning, data)),
            Self::Warning(status, data) => Ok((status, data)),
            Self::Error(status, data) => Err((status, data)),
        }
    }
}

impl<T, E> Display for EfiStatusEnum<T, E> {
    default fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        match self {
            Self::Success(_) => write!(f, "Success"),
            Self::Warning(status, _) => write!(f, "Warning ({:?})", status,),
            Self::Error(status, _) => write!(f, "Error ({:?})", status,),
        }
    }
}

impl<T: Clone, E: Clone> Clone for EfiStatusEnum<T, E> {
    fn clone(&self) -> Self {
        match self {
            Self::Success(data) => Self::Success(data.clone()),
            Self::Warning(status, data) => Self::Warning(*status, data.clone()),
            Self::Error(status, data) => Self::Error(*status, data.clone()),
        }
    }
}

impl<T: PartialEq, E: PartialEq> PartialEq<Self> for EfiStatusEnum<T, E> {
    default fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Success(data), Self::Success(other_data)) => data == other_data,
            (Self::Warning(status, data), Self::Warning(other_status, other_data)) => {
                status == other_status && data == other_data
            }
            (Self::Error(status, data), Self::Error(other_status, other_data)) => {
                status == other_status && data == other_data
            }
            _ => false,
        }
    }
}

impl<E: PartialEq> PartialEq<Self> for EfiStatusEnum<(), E> {
    default fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Success(_), Self::Success(_)) => true,
            (Self::Warning(self_status, _), Self::Warning(other_status, _)) => {
                self_status == other_status
            }
            (Self::Error(self_status, data), Self::Error(other_status, other_data)) => {
                self_status == other_status && data == other_data
            }
            _ => false,
        }
    }
}

impl PartialEq<Self> for EfiStatusEnum<(), ()> {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Success(_), Self::Success(_)) => true,
            (Self::Warning(status, _), Self::Warning(other_status, _)) => status == other_status,
            (Self::Error(status, _), Self::Error(other_status, _)) => status == other_status,
            _ => false,
        }
    }
}

impl<T: Eq, E: Eq> Eq for EfiStatusEnum<T, E> {}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[non_exhaustive]
pub enum EfiStatusWarning {
    NoWarning,

    UnknownWarning(EfiStatusRaw),

    EfiWarnUnknownGlyph,
    EfiWarnDeleteFailure,
    EfiWarnWriteFailure,
    EfiWarnBufferTooSmall,
    EfiWarnStaleData,
    EfiWarnFileSystem,
    EfiWarnResetRequired,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[non_exhaustive]
pub enum EfiStatusError {
    NoError,

    UnknownError(EfiStatusRaw),

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
