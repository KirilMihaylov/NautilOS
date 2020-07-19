use core::{
	mem::size_of,
	slice::from_raw_parts,
};

use crate::*;

use crate::protocols::{
	EfiProtocol,
	device_path::EfiDevicePathProcotol,
};

#[repr(C)]
#[derive(Clone,Copy)]
#[non_exhaustive]
pub enum EfiInterfaceType {
	NativeInterface,
}

#[repr(C)]
#[derive(Clone,Copy)]
#[non_exhaustive]
pub enum EfiLocateSearchType {
	AllHandles,
	ByRegisterNotify,
	ByProtocol,
}

#[repr(C)]
#[derive(Clone,Copy)]
pub(super) struct EfiProtocolHandlerRaw {
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

impl EfiProtocolHandlerRaw {
	#[inline(always)]
	pub(super) fn install_protocol_interface(&self, handle: &mut EfiHandle, protocol_guid: &EfiGuid, interface_type: EfiInterfaceType, interface: Option<&[u8]>) -> EfiStatusEnum {
		(self.install_protocol_interface)(
			handle,
			protocol_guid,
			interface_type,
			if let Some(interface) = interface {
				interface.as_ptr() as _
			} else {
				0 as _
			}
		).into_enum()
	}

	#[inline(always)]
	pub(super) fn reinstall_protocol_interface(&self, handle: &mut EfiHandle, protocol_guid: &EfiGuid, old_interface: Option<&[u8]>, new_interface: Option<&[u8]>) -> EfiStatusEnum {
		let (old_interface, new_interface): (*const u8, *const u8) = {
			let old_interface: *const u8 = if let Some(old_interface) = old_interface {
				old_interface.as_ptr()
			} else {
				0 as _
			};

			let new_interface: *const u8 = if let Some(new_interface) = new_interface {
				new_interface.as_ptr()
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

	#[inline(always)]
	pub(super) fn uninstall_protocol_interface(&self, handle: &mut EfiHandle, protocol_guid: &EfiGuid, interface: Option<&[u8]>) -> EfiStatusEnum {
		let interface: *const u8 = if let Some(interface) = interface {
			interface.as_ptr()
		} else {
			0 as _
		};

		(self.uninstall_protocol_interface)(
			handle,
			protocol_guid,
			interface as _
		).into_enum()
	}

	#[inline(always)]
	pub(super) fn handle_protocol(&self, handle: EfiHandle, protocol_guid: &EfiGuid) -> EfiStatusEnum<EfiProtocolBinding> {
		let mut interface: VoidPtr = 0 as _;
		
		(self.handle_protocol)(
			handle,
			protocol_guid,
			&mut interface as *mut _ as _
		).into_enum_data(
			EfiProtocolBinding {
				pointer: interface,
				guid: *protocol_guid,
			}
		)
	}

	#[inline(always)]
	pub(super) fn register_protocol_notify(&self, protocol_guid: &EfiGuid, event: EfiEvent) -> EfiStatusEnum<VoidPtr> {
		let mut registration: VoidPtr = 0 as _;

		(self.register_protocol_notify)(
			protocol_guid,
			event,
			&mut registration
		).into_enum_data(registration)
	}

	#[inline(always)]
	pub(super) fn locate_handle<'a>(&self, search_type: EfiLocateSearchType, protocol_guid: Option<&EfiGuid>, search_key: Option<VoidPtr>, buffer: &'a mut [EfiHandle]) -> EfiStatusEnum<&'a [EfiHandle], usize> {
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

	#[inline(always)]
	pub(super) fn locate_device_path(&self, protocol_guid: &EfiGuid, mut device_path: &mut EfiDevicePathProcotol) -> EfiStatusEnum<EfiHandle> {
		let mut handle: EfiHandle = 0 as _;

		(self.locate_device_path)(
			protocol_guid,
			&mut device_path as *mut _ as _,
			&mut handle
		).into_enum_data(handle)
	}

	#[inline(always)]
	pub(super) fn install_configuration_table(&self, table_guid: &EfiGuid, table_data: VoidPtr) -> EfiStatusEnum {
		(self.install_configuration_table)(
			table_guid,
			table_data
		).into_enum()
	}
}

pub trait EfiProtocolHandler {
	fn install_protocol_interface(&self, handle: &mut EfiHandle, protocol_guid: &EfiGuid, interface_type: EfiInterfaceType, interface: Option<&[u8]>) -> EfiStatusEnum;

	fn reinstall_protocol_interface(&self, handle: &mut EfiHandle, protocol_guid: &EfiGuid, old_interface: Option<&[u8]>, new_interface: Option<&[u8]>) -> EfiStatusEnum;
	
	fn uninstall_protocol_interface(&self, handle: &mut EfiHandle, protocol_guid: &EfiGuid, interface: Option<&[u8]>) -> EfiStatusEnum;

	fn handle_protocol(&self, handle: EfiHandle, protocol_guid: &EfiGuid) -> EfiStatusEnum<EfiProtocolBinding>;

	fn register_protocol_notify(&self, protocol_guid: &EfiGuid, event: EfiEvent) -> EfiStatusEnum<VoidPtr>;
	
	fn locate_handle<'a>(&self, search_type: EfiLocateSearchType, protocol_guid: Option<&EfiGuid>, search_key: Option<VoidPtr>, buffer: &'a mut [EfiHandle]) -> EfiStatusEnum<&'a [EfiHandle], usize>;

	fn locate_device_path(&self, protocol_guid: &EfiGuid, device_path: &mut EfiDevicePathProcotol) -> EfiStatusEnum<EfiHandle>;

	fn install_configuration_table(&self, table_guid: &EfiGuid, table_data: VoidPtr) -> EfiStatusEnum;
}

pub struct EfiProtocolBinding {
	pointer: VoidPtr,
	guid: EfiGuid,
}

impl EfiProtocolBinding {
	pub fn guid(&self) -> EfiGuid {
		self.guid
	}

	pub fn resolve<'a, T: EfiProtocol + Sized + 'a>(&'a self) -> Option<&'a T> {
		if self.pointer.is_null() {
			None
		} else if self.guid == T::guid() {
			Some(unsafe { &*(self.pointer as *const T) })
		} else {
			None
		}
	}

	pub fn resolve_mut<T: EfiProtocol>(&mut self) -> Option<&mut T> {
		if self.pointer.is_null() {
			None
		} else if self.guid == T::guid() {
			Some(unsafe { &mut *(self.pointer as *const T as *mut T) })
		} else {
			None
		}
	}
}
