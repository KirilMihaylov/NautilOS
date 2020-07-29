use efi_interops::{traits::acpi::device::NvdimmDeviceHandle, EfiObject};

use crate::protocols::device_path::{EfiDevicePathProcotol, EfiDevicePathRepr};

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

impl EfiDevicePathRepr for EfiNVDIMMDevicePath {}
