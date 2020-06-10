/// Defines all error types that can be returned by the functions within this crate.
#[non_exhaustive]
#[derive(Debug)]
pub enum Error {
	/// Unclassified error.
	Unclassified,
	/// Indicates the required operation is not available on the current platform.
	Unavailable,
	/// Feature is available but is disabled.
	FeatureDisabled,
	/// Feature is supported by the platform but OS interaction is required to determine whether it is supported (enabled) by the OS.
	/// 
	/// **Note**: This value will not be returned by any function in this crate when using the `kernel_mode` feature.
	OsInteractionRequired,
	/// OS interaction is required to determine whether the feature is available (on hardware level).
	/// 
	/// **Note**: This value will not be returned by any function in this crate when using the `kernel_mode` feature.
	OsInteractionRequiredAvailablility,
	/// Indicates the parameter(s) for the required operation are not properly aligned.
	Unaligned,
}

/// Defines new type equivalent to `Result<T, Error>` over the core library's `Result` where [`Error`] is the one defined by this crate.
pub type Result<T> = core::result::Result<T, Error>;
