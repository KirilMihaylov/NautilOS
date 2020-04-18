use core::{
	mem::size_of,
	slice::from_raw_parts
};

use crate::{
	types::{
		EfiEvent,
		EfiHandle,
		VoidPtr,
	},
	status::{
		EfiStatus,
		EfiStatusEnum,
	},
	guid::EfiGuid,
	protocols::{
		EfiProtocol,
		device_path::EfiDevicePathProcotol,
	},
};

use super::enums::{
	EfiInterfaceType,
	EfiLocateSearchType,
};

#[repr(C)]
pub struct EfiProtocolHandler {
	install_protocol_interface: extern "efiapi" fn(*mut EfiHandle, *const EfiGuid, EfiInterfaceType, VoidPtr) -> EfiStatus,
	reinstall_protocol_interface: extern "efiapi" fn(*mut EfiHandle, *const EfiGuid, VoidPtr, VoidPtr) -> EfiStatus,
	uninstall_protocol_interface: extern "efiapi" fn(*mut EfiHandle, *const EfiGuid, VoidPtr) -> EfiStatus,
	handle_protocol: extern "efiapi" fn(EfiHandle, *const EfiGuid, *mut VoidPtr) -> EfiStatus,
	_reserved: VoidPtr,
	register_protocol_notify: extern "efiapi" fn(&EfiGuid, EfiEvent, *mut VoidPtr) -> EfiStatus,
	locate_handle: extern "efiapi" fn(EfiLocateSearchType, *const EfiGuid, VoidPtr, *mut usize, *mut EfiHandle) -> EfiStatus,
	locate_device_path: extern "efiapi" fn(*const EfiGuid, *mut *const EfiDevicePathProcotol, *mut EfiHandle) -> EfiStatus,
	install_configuration_table: extern "efiapi" fn(*const EfiGuid, VoidPtr) -> EfiStatus,
}

impl EfiProtocolHandler {
	pub fn install_protocol_interface<T>(&self, handle: &mut EfiHandle, protocol_guid: &EfiGuid, interface_type: EfiInterfaceType, interface: Option<&T>) -> EfiStatusEnum {
		(self.install_protocol_interface)(
			handle,
			protocol_guid,
			interface_type,
			if let Some(interface) = interface {
				interface as *const T as _
			} else {
				0 as _
			}
		).into_enum()
	}

	pub fn reinstall_protocol_interface<T>(&self, handle: &mut EfiHandle, protocol_guid: &EfiGuid, old_interface: Option<&T>, new_interface: Option<&T>) -> EfiStatusEnum {
		let (old_interface, new_interface): (*const T, *const T) = {
			let old_interface: *const T = if let Some(old_interface) = old_interface {
				old_interface
			} else {
				0 as _
			};

			let new_interface: *const T = if let Some(new_interface) = new_interface {
				new_interface
			} else {
				0 as _
			};

			(old_interface, new_interface)
		};
		
		(self.reinstall_protocol_interface)(
			handle,
			protocol_guid,
			old_interface as _,
			new_interface as _
		).into_enum()
	}

	pub fn uninstall_protocol_interface<T>(&self, handle: &mut EfiHandle, protocol_guid: &EfiGuid, interface: Option<&T>) -> EfiStatusEnum {
		let interface: *const T = if let Some(interface) = interface {
			interface
		} else {
			0 as _
		};

		(self.uninstall_protocol_interface)(
			handle,
			protocol_guid,
			interface as _
		).into_enum()
	}

	pub fn handle_protocol_raw(&self, handle: EfiHandle, protocol_guid: &EfiGuid) -> EfiStatusEnum<VoidPtr> {
		let mut interface: VoidPtr = 0 as _;
		
		(self.handle_protocol)(
			handle,
			protocol_guid,
			&mut interface
		).into_enum_data(interface)
	}

	pub fn handle_protocol<T: Sized + EfiProtocol>(&self, handle: EfiHandle) -> EfiStatusEnum<&<T as EfiProtocol>::Interface> {
		let mut interface: *const <T as EfiProtocol>::Interface = 0 as _;
		
		(self.handle_protocol)(
			handle,
			&T::guid(),
			&mut interface as *mut _ as _
		).into_enum_data(
			unsafe {
				&*interface
			}
		)
	}

	pub fn register_protocol_notify(&self, protocol_guid: &EfiGuid, event: EfiEvent) -> EfiStatusEnum<VoidPtr> {
		let mut registration: VoidPtr = 0 as _;

		(self.register_protocol_notify)(
			protocol_guid,
			event,
			&mut registration
		).into_enum_data(registration)
	}

	pub fn locate_handle<'a>(&self, search_type: EfiLocateSearchType, protocol_guid: Option<&EfiGuid>, search_key: Option<VoidPtr>, buffer: &'a mut [EfiHandle]) -> EfiStatusEnum<&'a [EfiHandle], usize> {
		const EFI_HANDLE_SIZE: usize = size_of::<EfiHandle>();

		let mut buffer_size: usize = buffer.len() * EFI_HANDLE_SIZE;

		let protocol_guid: *const EfiGuid = if let Some(protocol_guid) = protocol_guid {
			protocol_guid
		} else {
			0 as _
		};

		let search_key: VoidPtr = if let Some(search_key) = search_key {
			search_key
		} else {
			0 as _
		};

		let result: EfiStatus = (self.locate_handle)(
			search_type,
			protocol_guid,
			search_key,
			&mut buffer_size,
			buffer.as_mut_ptr()
		);

		let success_data: &'a [EfiHandle];

		if result.is_error() {
			success_data = &[];
		} else {
			success_data = unsafe {
				from_raw_parts(
					buffer.as_ptr(),
					buffer_size / EFI_HANDLE_SIZE
				)
			};
		}

		result.into_enum_data_error(
			success_data,
			buffer_size
		)
	}

	pub fn locate_device_path(&self, protocol_guid: &EfiGuid, mut device_path: &mut EfiDevicePathProcotol) -> EfiStatusEnum<EfiHandle> {
		let mut handle: EfiHandle = 0 as _;

		(self.locate_device_path)(
			protocol_guid,
			&mut device_path as *mut _ as _,
			&mut handle
		).into_enum_data(handle)
	}

	pub fn install_configuration_table(&self, table_guid: &EfiGuid, table_data: VoidPtr) -> EfiStatusEnum {
		(self.install_configuration_table)(
			table_guid,
			table_data
		).into_enum()
	}
}
