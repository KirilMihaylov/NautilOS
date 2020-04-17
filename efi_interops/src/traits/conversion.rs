use crate::EfiObject;

pub unsafe trait EfiToAcpiObject: Sized {
	fn convert(efi_object: EfiObject) -> Option<Self>;
}
