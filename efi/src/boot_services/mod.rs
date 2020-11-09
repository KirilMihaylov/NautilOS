pub mod types;

use {
    crate::{
        protocols::{device_path::EfiDevicePathProtocolRaw, EfiProtocol},
        EfiEvent, EfiGuid, EfiHandle, EfiPhysicalAddress, EfiStatus, EfiStatusEnum, EfiTableHeader,
        Void, VoidPtr,
    },
    core::{
        mem::size_of,
        ops::{Deref, DerefMut},
        slice::from_raw_parts,
    },
    types::{
        event_and_timer::{EfiEventNotifyCallback, EfiEventType, EfiTimerDelay},
        memory::{
            EfiAllocateType, EfiGetMemoryMapResult, EfiMemoryDescriptor, EfiMemoryDescriptorsMut,
            EfiMemoryType,
        },
        protocol_handler::{EfiInterfaceType, EfiLocateSearchType},
        task_priority::EfiTaskPriorityLevel,
    },
};

#[repr(C)]
pub struct EfiBootServices {
    table_header: EfiTableHeader,
    v1_0: EfiBootServices1x0,
}

impl EfiBootServices {
    pub fn header(&self) -> &EfiTableHeader {
        &self.table_header
    }

    pub fn header_mut(&mut self) -> &mut EfiTableHeader {
        &mut self.table_header
    }
}

impl Deref for EfiBootServices {
    type Target = EfiBootServices1x0;

    fn deref(&self) -> &<Self as Deref>::Target {
        &self.v1_0
    }
}

impl DerefMut for EfiBootServices {
    fn deref_mut(&mut self) -> &mut <Self as Deref>::Target {
        &mut self.v1_0
    }
}

#[repr(C)]
pub struct EfiBootServices1x0 {
    raise_tpl: extern "efiapi" fn(EfiTaskPriorityLevel) -> EfiStatus,
    restore_tpl: extern "efiapi" fn(EfiTaskPriorityLevel) -> Void,

    /*~~~~~~~~~~*/
    allocate_pages: extern "efiapi" fn(
        EfiAllocateType,
        EfiMemoryType,
        usize,
        *mut EfiPhysicalAddress,
    ) -> EfiStatus,
    free_pages: extern "efiapi" fn(EfiPhysicalAddress, usize) -> EfiStatus,
    get_memory_map: extern "efiapi" fn(
        *mut usize,
        *mut EfiMemoryDescriptor,
        *mut usize,
        *mut usize,
        *mut u32,
    ) -> EfiStatus,
    allocate_pool: extern "efiapi" fn(EfiMemoryType, usize, *mut VoidPtr) -> EfiStatus,
    free_pool: extern "efiapi" fn(VoidPtr) -> EfiStatus,

    /*~~~~~~~~~~*/
    create_event: extern "efiapi" fn(
        EfiEventType,
        EfiTaskPriorityLevel,
        EfiEventNotifyCallback,
        VoidPtr,
        *mut EfiEvent,
    ) -> EfiStatus,
    set_timer: extern "efiapi" fn(EfiEvent, EfiTimerDelay, u64) -> EfiStatus,
    wait_for_event: extern "efiapi" fn(usize, *const EfiEvent, *mut usize) -> EfiStatus,
    signal_event: extern "efiapi" fn(EfiEvent) -> EfiStatus,
    close_event: extern "efiapi" fn(EfiEvent) -> EfiStatus,
    check_event: extern "efiapi" fn(EfiEvent) -> EfiStatus,

    /*~~~~~~~~~~*/
    install_protocol_interface:
        extern "efiapi" fn(*mut EfiHandle, *const EfiGuid, EfiInterfaceType, VoidPtr) -> EfiStatus,
    reinstall_protocol_interface:
        extern "efiapi" fn(*mut EfiHandle, *const EfiGuid, VoidPtr, VoidPtr) -> EfiStatus,
    uninstall_protocol_interface:
        extern "efiapi" fn(*mut EfiHandle, *const EfiGuid, VoidPtr) -> EfiStatus,
    handle_protocol: extern "efiapi" fn(EfiHandle, *const EfiGuid, *mut VoidPtr) -> EfiStatus,
    _reserved: VoidPtr,
    register_protocol_notify: extern "efiapi" fn(&EfiGuid, EfiEvent, *mut VoidPtr) -> EfiStatus,
    locate_handle: extern "efiapi" fn(
        EfiLocateSearchType,
        *const EfiGuid,
        VoidPtr,
        *mut usize,
        *mut EfiHandle,
    ) -> EfiStatus,
    locate_device_path: extern "efiapi" fn(
        *const EfiGuid,
        *mut EfiDevicePathProtocolRaw,
        *mut EfiHandle,
    ) -> EfiStatus,
    install_configuration_table: extern "efiapi" fn(*const EfiGuid, VoidPtr) -> EfiStatus,

    /*~~~~~~~~~~*/
    load_image: extern "efiapi" fn(
        bool,
        EfiHandle,
        EfiDevicePathProtocolRaw,
        VoidPtr,
        usize,
        *const EfiHandle,
    ) -> EfiStatus,
    start_image: extern "efiapi" fn(EfiHandle, *mut usize, *mut *const u16) -> EfiStatus,
    exit: extern "efiapi" fn(EfiHandle, EfiStatus, usize, *const u16) -> EfiStatus,
    unload_image: extern "efiapi" fn(EfiHandle) -> EfiStatus,
    exit_boot_services: extern "efiapi" fn(EfiHandle, usize) -> EfiStatus,
}

impl EfiBootServices1x0 {
    /* TASK PRIORITY */

    #[inline(always)]
    pub fn raise_priority_level(&self, new_priority_level: EfiTaskPriorityLevel) -> EfiStatusEnum {
        (self.raise_tpl)(new_priority_level).into_enum()
    }

    #[inline(always)]
    pub fn restore_priority_level(&self, old_priority_level: EfiTaskPriorityLevel) {
        (self.restore_tpl)(old_priority_level);
    }

    /* MEMORY */

    #[inline(always)]
    pub fn allocate_pages(
        &self,
        allocation_type: EfiAllocateType,
        memory_type: EfiMemoryType,
        number_of_pages: usize,
        physical_address: &mut EfiPhysicalAddress,
    ) -> EfiStatusEnum {
        (self.allocate_pages)(
            allocation_type,
            memory_type,
            number_of_pages,
            physical_address,
        )
        .into_enum()
    }

    #[inline(always)]
    pub fn free_pages(
        &self,
        physical_address: EfiPhysicalAddress,
        number_of_pages: usize,
    ) -> EfiStatusEnum {
        (self.free_pages)(physical_address, number_of_pages).into_enum()
    }

    pub fn get_memory_map<'a>(
        &self,
        memory_map: &'a mut [u8],
    ) -> EfiStatusEnum<EfiGetMemoryMapResult<'a>, usize> {
        let (
			mut allocation_size,
			mut memory_map_key,
			mut descriptor_size,
			mut descriptor_version
		): (usize, usize, usize, u32) = (memory_map.len(), 0, 0, 0);

        let result: EfiStatus = (self.get_memory_map)(
            &mut allocation_size,
            memory_map.as_mut_ptr() as *mut EfiMemoryDescriptor,
            &mut memory_map_key,
            &mut descriptor_size,
            &mut descriptor_version,
        );

        let return_data: EfiGetMemoryMapResult = EfiGetMemoryMapResult::new(
            memory_map_key,
            descriptor_version,
            EfiMemoryDescriptorsMut::new(&mut memory_map[..allocation_size], descriptor_size),
        );

        result.into_enum_data_error(|| return_data, || allocation_size)
    }

    pub fn allocate_pool(
        &self,
        pool_type: EfiMemoryType,
        pool_size: usize,
    ) -> EfiStatusEnum<VoidPtr> {
        let mut buffer: VoidPtr = 0 as VoidPtr;

        (self.allocate_pool)(pool_type, pool_size, &mut buffer).into_enum_data(|| buffer)
    }

    #[inline(always)]
    pub fn free_pool(&self, buffer: VoidPtr) -> EfiStatusEnum {
        (self.free_pool)(buffer).into_enum()
    }

    /* Event and Timer */

    pub fn create_event(
        &self,
        event_type: EfiEventType,
        tpl: EfiTaskPriorityLevel,
        notify: Option<(EfiEventNotifyCallback, VoidPtr)>,
    ) -> EfiStatusEnum<EfiEvent> {
        let mut event: EfiEvent = 0 as EfiEvent;

        let (notify_function, notify_context): (EfiEventNotifyCallback, VoidPtr) = {
            match notify {
                None => (
                    unsafe { *(&0usize as *const usize as *const EfiEventNotifyCallback) },
                    0 as VoidPtr,
                ),
                Some((notify_function, notify_context)) => (notify_function, notify_context),
            }
        };

        (self.create_event)(event_type, tpl, notify_function, notify_context, &mut event)
            .into_enum_data(|| event)
    }

    #[inline(always)]
    pub fn set_timer(
        &self,
        event: EfiEvent,
        timer_type: EfiTimerDelay,
        trigger_time: u64,
    ) -> EfiStatusEnum {
        (self.set_timer)(event, timer_type, trigger_time).into_enum()
    }

    pub fn wait_for_event(&self, events: &[EfiEvent]) -> EfiStatusEnum<usize> {
        let mut index: usize = 0;

        (self.wait_for_event)(events.len(), events.as_ptr(), &mut index).into_enum_data(|| index)
    }

    #[inline(always)]
    pub fn signal_event(&self, event: EfiEvent) -> EfiStatusEnum {
        (self.signal_event)(event).into_enum()
    }

    #[inline(always)]
    pub fn close_event(&self, event: EfiEvent) -> EfiStatusEnum {
        (self.close_event)(event).into_enum()
    }

    #[inline(always)]
    pub fn check_event(&self, event: EfiEvent) -> EfiStatusEnum {
        (self.check_event)(event).into_enum()
    }

    /* Protocol Handler */

    #[inline(always)]
    pub fn install_protocol_interface(
        &self,
        handle: &mut EfiHandle,
        protocol_guid: &EfiGuid,
        interface_type: EfiInterfaceType,
        interface: Option<&[u8]>,
    ) -> EfiStatusEnum {
        (self.install_protocol_interface)(
            handle,
            protocol_guid,
            interface_type,
            if let Some(interface) = interface {
                interface.as_ptr() as _
            } else {
                0 as _
            },
        )
        .into_enum()
    }

    pub fn reinstall_protocol_interface(
        &self,
        handle: &mut EfiHandle,
        protocol_guid: &EfiGuid,
        old_interface: Option<&[u8]>,
        new_interface: Option<&[u8]>,
    ) -> EfiStatusEnum {
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
            new_interface as _,
        )
        .into_enum()
    }

    pub fn uninstall_protocol_interface(
        &self,
        handle: &mut EfiHandle,
        protocol_guid: &EfiGuid,
        interface: Option<&[u8]>,
    ) -> EfiStatusEnum {
        let interface: *const u8 = if let Some(interface) = interface {
            interface.as_ptr()
        } else {
            0 as _
        };

        (self.uninstall_protocol_interface)(handle, protocol_guid, interface as _).into_enum()
    }

    #[inline(always)]
    pub fn handle_protocol<T>(
        &self,
        handle: EfiHandle,
    ) -> EfiStatusEnum<Result<T::Parsed, T::Error>>
    where
        T: EfiProtocol + ?Sized,
    {
        let mut interface: VoidPtr = 0 as _;

        (self.handle_protocol)(handle, &T::guid(), &mut interface as *mut _ as _)
            .into_enum_data(|| unsafe { T::parse(interface) })
    }

    #[inline(always)]
    pub fn register_protocol_notify(
        &self,
        protocol_guid: &EfiGuid,
        event: EfiEvent,
    ) -> EfiStatusEnum<VoidPtr> {
        let mut registration: VoidPtr = 0 as _;

        (self.register_protocol_notify)(protocol_guid, event, &mut registration)
            .into_enum_data(|| registration)
    }

    pub fn locate_handle<'a>(
        &self,
        search_type: EfiLocateSearchType,
        protocol_guid: Option<&EfiGuid>,
        search_key: Option<VoidPtr>,
        buffer: &'a mut [EfiHandle],
    ) -> EfiStatusEnum<&'a [EfiHandle], usize> {
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
            buffer.as_mut_ptr(),
        );

        result.into_enum_data_error(
            || unsafe { from_raw_parts(buffer.as_ptr(), buffer_size / EFI_HANDLE_SIZE) },
            || buffer_size,
        )
    }

    #[inline(always)]
    pub fn locate_device_path(
        &self,
        protocol_guid: &EfiGuid,
        device_path: &mut EfiDevicePathProtocolRaw,
    ) -> EfiStatusEnum<EfiHandle> {
        let mut handle: EfiHandle = 0 as _;

        (self.locate_device_path)(
            protocol_guid,
            device_path as *mut EfiDevicePathProtocolRaw,
            &mut handle,
        )
        .into_enum_data(|| handle)
    }

    #[inline(always)]
    pub fn install_configuration_table(
        &self,
        table_guid: &EfiGuid,
        table_data: VoidPtr,
    ) -> EfiStatusEnum {
        (self.install_configuration_table)(table_guid, table_data).into_enum()
    }

    /* Image */

    pub fn load_image(
        &self,
        boot_policy: bool,
        parent_image_handle: EfiHandle,
        device_path: EfiDevicePathProtocolRaw,
        source_buffer: Option<&[u8]>,
    ) -> EfiStatusEnum<EfiHandle> {
        let mut image_handle: EfiHandle = 0 as _;

        let (source_buffer_ptr, source_buffer_len): (VoidPtr, usize) =
            if let Some(source_buffer) = source_buffer {
                (source_buffer.as_ptr() as _, source_buffer.len())
            } else {
                (0 as _, 0)
            };

        (self.load_image)(
            boot_policy,
            parent_image_handle,
            device_path,
            source_buffer_ptr,
            source_buffer_len,
            &mut image_handle,
        )
        .into_enum_data(|| image_handle)
    }

    pub fn start_image(&self, image_handle: EfiHandle) -> EfiStatusEnum<(&[u16], &[u8])> {
        let (mut exit_data, mut exit_data_size): (*const u16, usize) = (0 as _, 0);

        let efi_status: EfiStatus =
            (self.start_image)(image_handle, &mut exit_data_size, &mut exit_data);

        let (utf_16_data, raw_data): (&[u16], &[u8]);

        if exit_data.is_null() {
            utf_16_data = &[];
            raw_data = &[];
        } else {
            let utf_16_data_length: usize = {
                let (mut ptr, mut utf_16_data_length): (*const u16, usize) = (exit_data, 2);
                loop {
                    if exit_data_size <= utf_16_data_length {
                        unsafe {
                            if *ptr == 0 {
                                break utf_16_data_length;
                            } else {
                                ptr = ptr.offset(1);
                                utf_16_data_length += 2;
                            }
                        }
                    } else {
                        break 0;
                    }
                }
            };
            utf_16_data = unsafe { from_raw_parts(exit_data, utf_16_data_length / 2) };
            raw_data =
                unsafe { from_raw_parts(exit_data as _, exit_data_size - utf_16_data_length) };
        }

        efi_status.into_enum_data(|| (utf_16_data, raw_data))
    }

    pub fn exit(
        &self,
        image_handle: EfiHandle,
        exit_status: EfiStatus,
        exit_data: Option<&[u16]>,
    ) -> EfiStatusEnum {
        let (exit_data_ptr, exit_data_len): (*const u16, usize) = if let Some(exit_data) = exit_data
        {
            (exit_data.as_ptr(), exit_data.len() * 2)
        } else {
            (0 as _, 0)
        };

        (self.exit)(image_handle, exit_status, exit_data_len, exit_data_ptr).into_enum()
    }

    #[inline(always)]
    pub fn unload_image(&self, image_handle: EfiHandle) -> EfiStatusEnum {
        (self.unload_image)(image_handle).into_enum()
    }

    #[inline(always)]
    pub fn exit_boot_services(
        &self,
        image_handle: EfiHandle,
        memory_map_key: usize,
    ) -> EfiStatusEnum {
        (self.exit_boot_services)(image_handle, memory_map_key).into_enum()
    }
}
