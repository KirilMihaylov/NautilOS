/// Defines all error types that can be returned by the functions within this crate.
#[non_exhaustive]
pub enum Error {
	/// Indicates the required operation is not available on the current platform.
	Unavailable,

	/// Indicates the parameter(s) for the required operation are not properly aligned.
	Unaligned,
}

/// Defines new type equivalent to `Result<T, Error>` over the core library's [`core::result::Result`] where [`Error`] is the one defined by this crate.
/// 
/// [`Error`]: enum.Error.html
pub type Result<T> = core::result::Result<T, Error>;
