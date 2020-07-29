pub mod event_and_timer;
pub mod image;
pub mod memory;
pub mod miscellaneous;
pub mod protocol_handler;
pub mod task_priority;

use crate::*;
use event_and_timer::*;
use image::*;
use memory::*;
use miscellaneous::*;
use protocol_handler::*;
use task_priority::*;

use crate::protocols::device_path::EfiDevicePathProcotol;

#[repr(C)]
#[derive(Clone, Copy)]
pub(super) struct EfiBootServicesRevision_1_0_Raw {
    task_priority: EfiTaskPriorityRaw,
    memory: EfiMemoryRaw,
    event_and_timer: EfiEventAndTimerRaw,
    protocol_handler: EfiProtocolHandlerRaw,
    image: EfiImageRaw,
    miscellaneous: EfiMiscellaneousRaw,
}

#[allow(non_camel_case_types)]
pub trait EfiBootServicesRevision_1_0:
    EfiEventAndTimer + EfiImage + EfiMemory + EfiMiscellaneous + EfiProtocolHandler + EfiTaskPriority
{
}
impl EfiBootServicesRevision_1_0 for EfiBootServicesRevision_1_0_Raw {}

impl EfiEventAndTimer for EfiBootServicesRevision_1_0_Raw {
    fn create_event(
        &self,
        event_type: EfiEventType,
        tpl: EfiTaskPriorityLevel,
        notify: Option<(EfiEventNotifyCallback, VoidPtr)>,
    ) -> EfiStatusEnum<EfiEvent> {
        self.event_and_timer.create_event(event_type, tpl, notify)
    }

    fn set_timer(
        &self,
        event: EfiEvent,
        timer_type: EfiTimerDelay,
        trigger_time: u64,
    ) -> EfiStatusEnum {
        self.event_and_timer
            .set_timer(event, timer_type, trigger_time)
    }

    fn wait_for_event(&self, events: &[EfiEvent]) -> EfiStatusEnum<usize> {
        self.event_and_timer.wait_for_event(events)
    }

    fn signal_event(&self, event: EfiEvent) -> EfiStatusEnum {
        self.event_and_timer.signal_event(event)
    }

    fn close_event(&self, event: EfiEvent) -> EfiStatusEnum {
        self.event_and_timer.close_event(event)
    }

    fn check_event(&self, event: EfiEvent) -> EfiStatusEnum {
        self.event_and_timer.check_event(event)
    }
}

impl EfiImage for EfiBootServicesRevision_1_0_Raw {
    fn load_image(
        &self,
        boot_policy: bool,
        parent_image_handle: EfiHandle,
        device_path: &EfiDevicePathProcotol,
        source_buffer: Option<&[u8]>,
    ) -> EfiStatusEnum<EfiHandle> {
        self.image
            .load_image(boot_policy, parent_image_handle, device_path, source_buffer)
    }

    fn start_image(&self, image_handle: EfiHandle) -> EfiStatusEnum<(&[u16], &[u8])> {
        self.image.start_image(image_handle)
    }

    fn exit(
        &self,
        image_handle: EfiHandle,
        exit_status: EfiStatus,
        exit_data: Option<&[u16]>,
    ) -> EfiStatusEnum {
        self.image.exit(image_handle, exit_status, exit_data)
    }

    fn unload_image(&self, image_handle: EfiHandle) -> EfiStatusEnum {
        self.image.unload_image(image_handle)
    }

    fn exit_boot_services(
        &self,
        image_handle: EfiHandle,
        memory_map: &EfiMemoryDescriptors,
    ) -> EfiStatusEnum {
        self.image.exit_boot_services(image_handle, memory_map)
    }
}

impl EfiMemory for EfiBootServicesRevision_1_0_Raw {
    fn allocate_pages(
        &self,
        allocation_type: EfiAllocateType,
        memory_type: EfiMemoryType,
        number_of_pages: usize,
        physical_address: &mut EfiPhysicalAddress,
    ) -> EfiStatusEnum {
        self.memory.allocate_pages(
            allocation_type,
            memory_type,
            number_of_pages,
            physical_address,
        )
    }

    fn free_pages(
        &self,
        physical_address: EfiPhysicalAddress,
        number_of_pages: usize,
    ) -> EfiStatusEnum {
        self.memory.free_pages(physical_address, number_of_pages)
    }

    fn get_memory_map<'a>(
        &self,
        memory_map: &'a mut [u8],
    ) -> EfiStatusEnum<EfiMemoryDescriptors, usize> {
        self.memory.get_memory_map(memory_map)
    }

    fn allocate_pool(&self, pool_type: EfiMemoryType, pool_size: usize) -> EfiStatusEnum<VoidPtr> {
        self.memory.allocate_pool(pool_type, pool_size)
    }

    fn free_pool(&self, buffer: VoidPtr) -> EfiStatusEnum {
        self.memory.free_pool(buffer)
    }
}

impl EfiMiscellaneous for EfiBootServicesRevision_1_0_Raw {
    fn get_next_monotonic_count(&self) -> EfiStatusEnum<(u32, u32)> {
        self.miscellaneous.get_next_monotonic_count()
    }

    fn stall(&self, microseconds: usize) -> EfiStatusEnum {
        self.miscellaneous.stall(microseconds)
    }

    fn set_watchdog_timer(
        &self,
        timeout: usize,
        watchdog_code: u64,
        watchdog_data: Option<&[u16]>,
    ) -> EfiStatusEnum {
        self.miscellaneous
            .set_watchdog_timer(timeout, watchdog_code, watchdog_data)
    }
}

impl EfiProtocolHandler for EfiBootServicesRevision_1_0_Raw {
    fn install_protocol_interface(
        &self,
        handle: &mut EfiHandle,
        protocol_guid: &EfiGuid,
        interface_type: EfiInterfaceType,
        interface: Option<&[u8]>,
    ) -> EfiStatusEnum {
        self.protocol_handler.install_protocol_interface(
            handle,
            protocol_guid,
            interface_type,
            interface,
        )
    }

    fn reinstall_protocol_interface(
        &self,
        handle: &mut EfiHandle,
        protocol_guid: &EfiGuid,
        old_interface: Option<&[u8]>,
        new_interface: Option<&[u8]>,
    ) -> EfiStatusEnum {
        self.protocol_handler.reinstall_protocol_interface(
            handle,
            protocol_guid,
            old_interface,
            new_interface,
        )
    }

    fn uninstall_protocol_interface(
        &self,
        handle: &mut EfiHandle,
        protocol_guid: &EfiGuid,
        interface: Option<&[u8]>,
    ) -> EfiStatusEnum {
        self.protocol_handler
            .uninstall_protocol_interface(handle, protocol_guid, interface)
    }

    fn handle_protocol(
        &self,
        handle: EfiHandle,
        protocol_guid: &EfiGuid,
    ) -> EfiStatusEnum<EfiProtocolBinding> {
        self.protocol_handler.handle_protocol(handle, protocol_guid)
    }

    fn register_protocol_notify(
        &self,
        protocol_guid: &EfiGuid,
        event: EfiEvent,
    ) -> EfiStatusEnum<VoidPtr> {
        self.protocol_handler
            .register_protocol_notify(protocol_guid, event)
    }

    fn locate_handle<'a>(
        &self,
        search_type: EfiLocateSearchType,
        protocol_guid: Option<&EfiGuid>,
        search_key: Option<VoidPtr>,
        buffer: &'a mut [EfiHandle],
    ) -> EfiStatusEnum<&'a [EfiHandle], usize> {
        self.protocol_handler
            .locate_handle(search_type, protocol_guid, search_key, buffer)
    }

    fn locate_device_path(
        &self,
        protocol_guid: &EfiGuid,
        device_path: &mut EfiDevicePathProcotol,
    ) -> EfiStatusEnum<EfiHandle> {
        self.protocol_handler
            .locate_device_path(protocol_guid, device_path)
    }

    fn install_configuration_table(
        &self,
        table_guid: &EfiGuid,
        table_data: VoidPtr,
    ) -> EfiStatusEnum {
        self.protocol_handler
            .install_configuration_table(table_guid, table_data)
    }
}

impl EfiTaskPriority for EfiBootServicesRevision_1_0_Raw {
    fn raise_priority_level(&self, new_priority_level: EfiTaskPriorityLevel) -> EfiStatusEnum {
        self.task_priority.raise_priority_level(new_priority_level)
    }

    fn restore_priority_level(&self, old_priority_level: EfiTaskPriorityLevel) {
        self.task_priority
            .restore_priority_level(old_priority_level)
    }
}
