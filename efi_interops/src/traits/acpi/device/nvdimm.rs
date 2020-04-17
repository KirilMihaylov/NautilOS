use crate::traits::conversion::EfiToAcpiObject;

pub unsafe trait NvdimmDeviceHandle: EfiToAcpiObject {}
