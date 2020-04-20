use efi_interops::{
	EfiObject,
	traits::acpi::device::NvdimmDeviceHandle,
};

use crate::protocols::device_path::{
	EfiDevicePathProcotol,
	EfiDevicePathInto
};

#[repr(C)]
pub struct EfiNVDIMMDevicePath {
	base: EfiDevicePathProcotol,
	handle: [u8; 4],
}

impl EfiNVDIMMDevicePath {
	pub fn as_acpi_object<T: NvdimmDeviceHandle>(&self) -> Option<T> {
		T::convert(EfiObject::new(&self.handle))
	}
}

impl EfiDevicePathInto<EfiNVDIMMDevicePath> for EfiNVDIMMDevicePath {}
