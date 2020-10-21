use core::{
    borrow::{Borrow, BorrowMut},
    ops::{Deref, DerefMut},
};

use crate::{utilities::validate_string, *};

#[repr(C)]
#[derive(Clone, Copy)]
pub(super) struct EfiVariableRaw {
    get_variable: extern "efiapi" fn(
        *const u16,
        *const EfiGuid,
        *mut u32,
        *mut usize,
        VoidMutPtr,
    ) -> EfiStatus,
    get_next_variable_name: extern "efiapi" fn(*mut usize, *mut u16, *mut EfiGuid) -> EfiStatus,
    set_variable:
        extern "efiapi" fn(*const u16, *const EfiGuid, *const u32, usize, VoidPtr) -> EfiStatus,
}

impl EfiVariableRaw {
    pub(super) fn get_variable(
        &self,
        variable_name: &[u16],
        vendor_guid: &EfiGuid,
        data: Option<&mut [u8]>,
    ) -> EfiStatusEnum<(usize, EfiVariableAttributes), (usize, EfiVariableAttributes)> {
        let (variable_name_ptr, data_ptr, mut data_len, mut attributes): (
            *const u16,
            VoidMutPtr,
            usize,
            EfiVariableAttributes,
        );

        variable_name_ptr = if validate_string(variable_name).is_ok() {
            variable_name.as_ptr()
        } else {
            core::ptr::null()
        };

        if let Some(data) = data {
            data_ptr = data.as_mut_ptr() as VoidMutPtr;
            data_len = data.len();
        } else {
            data_ptr = core::ptr::null_mut();
            data_len = 0;
        }

        attributes = EfiVariableAttributes { attributes: 0 };

        let result: EfiStatus = (self.get_variable)(
            variable_name_ptr,
            vendor_guid,
            &mut attributes.attributes as *mut u32,
            &mut data_len,
            data_ptr,
        );

        let f = || (data_len, attributes);

        result.into_enum_data_error(f, f)
    }

    pub(super) fn get_next_variable_name(
        &self,
        variable_name: &mut [u16],
        vendor_guid: &mut EfiGuid,
    ) -> EfiStatusEnum<(), usize> {
        let mut variable_name_len: usize = variable_name.len();

        (self.get_next_variable_name)(
            &mut variable_name_len,
            variable_name.as_mut_ptr(),
            vendor_guid,
        )
        .into_enum_data_error(|| (), || variable_name_len)
    }

    pub(super) fn set_variable(
        &self,
        variable_name: &[u16],
        vendor_guid: &EfiGuid,
        attributes: &EfiVariableAttributes,
        data: &[u8],
    ) -> EfiStatusEnum {
        let variable_name_ptr: *const u16 = if validate_string(variable_name).is_ok() {
            variable_name.as_ptr()
        } else {
            core::ptr::null()
        };

        (self.set_variable)(
            variable_name_ptr,
            vendor_guid,
            attributes.borrow(),
            data.len(),
            data.as_ptr() as VoidPtr,
        )
        .into_enum()
    }
}

pub trait EfiVariable {
    fn get_variable(
        &self,
        variable_name: &[u16],
        vendor_guid: &EfiGuid,
        data: Option<&mut [u8]>,
    ) -> EfiStatusEnum<(usize, EfiVariableAttributes), (usize, EfiVariableAttributes)>;

    fn get_next_variable_name(
        &self,
        variable_name: &mut [u16],
        vendor_guid: &mut EfiGuid,
    ) -> EfiStatusEnum<(), usize>;

    fn set_variable(
        &self,
        variable_name: &[u16],
        vendor_guid: &EfiGuid,
        attributes: &EfiVariableAttributes,
        data: &[u8],
    ) -> EfiStatusEnum;
}

#[repr(transparent)]
#[derive(Clone, Copy)]
pub struct EfiVariableAttributes {
    attributes: u32,
}

impl EfiVariableAttributes {
    pub fn non_volatile(&self) -> bool {
        self.attributes & 1 == 1
    }

    pub fn boot_service_access(&self) -> bool {
        self.attributes & 2 == 2
    }

    pub fn runtime_access(&self) -> bool {
        self.attributes & 4 == 4
    }

    pub fn hardware_error_record(&self) -> bool {
        self.attributes & 8 == 8
    }

    /* Deprecated by specification; Should be considered reserved */
    #[deprecated]
    pub fn authenticated_write_access(&self) -> bool {
        self.attributes & 0x10 == 0x10
    }

    pub fn time_based_authenticated_write_access(&self) -> bool {
        self.attributes & 0x20 == 0x20
    }

    pub fn apppend_write(&self) -> bool {
        self.attributes & 0x40 == 0x40
    }

    pub fn enhanced_authenticated_access(&self) -> bool {
        self.attributes & 0x80 == 0x80
    }
}

impl Deref for EfiVariableAttributes {
    type Target = u32;

    fn deref(&self) -> &<Self as Deref>::Target {
        &self.attributes
    }
}

impl DerefMut for EfiVariableAttributes {
    fn deref_mut(&mut self) -> &mut <Self as Deref>::Target {
        &mut self.attributes
    }
}

impl Borrow<<Self as Deref>::Target> for EfiVariableAttributes {
    fn borrow(&self) -> &<Self as Deref>::Target {
        &self.attributes
    }
}

impl BorrowMut<<Self as Deref>::Target> for EfiVariableAttributes {
    fn borrow_mut(&mut self) -> &mut <Self as Deref>::Target {
        &mut self.attributes
    }
}
