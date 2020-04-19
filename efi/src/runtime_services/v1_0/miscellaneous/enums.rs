#[repr(C)]
pub enum EfiResetType {
	Cold,
	Warm,
	Shutdown,
	PlatformSpecific,
}
