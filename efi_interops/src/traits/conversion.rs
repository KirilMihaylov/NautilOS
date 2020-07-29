use crate::EfiObject;

pub unsafe trait FromEfiObject: Sized {
    fn convert(efi_object: EfiObject) -> Option<Self>;
}
