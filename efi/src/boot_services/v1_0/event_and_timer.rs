use crate::*;

#[repr(u32)]
#[derive(Clone, Copy)]
#[non_exhaustive]
pub enum EfiEventType {
    #[allow(overflowing_literals)]
    Timer = 0x80000000,
    Runtime = 0x40000000,
    NotifyWait = 0x100,
    NotifySignal = 0x200,
    SignalExitBootServices = 0x201,
    SignalVirtualAddressChange = 0x60000202,
}

#[repr(C)]
#[derive(Clone, Copy)]
#[non_exhaustive]
pub enum EfiTimerDelay {
    Cancel,
    Pediodic,
    Relative,
}

pub type EfiEventNotifyCallback = extern "efiapi" fn(EfiEvent, VoidPtr);

use super::task_priority::EfiTaskPriorityLevel;

#[repr(C)]
#[derive(Clone, Copy)]
pub(super) struct EfiEventAndTimerRaw {
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
}

impl EfiEventAndTimerRaw {
    #[inline(always)]
    pub(super) fn create_event(
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
            .into_enum_data(event)
    }

    #[inline(always)]
    pub(super) fn set_timer(
        &self,
        event: EfiEvent,
        timer_type: EfiTimerDelay,
        trigger_time: u64,
    ) -> EfiStatusEnum {
        (self.set_timer)(event, timer_type, trigger_time).into_enum()
    }

    #[inline(always)]
    pub(super) fn wait_for_event(&self, events: &[EfiEvent]) -> EfiStatusEnum<usize> {
        let mut index: usize = 0;

        (self.wait_for_event)(events.len(), events.as_ptr(), &mut index).into_enum_data(index)
    }

    #[inline(always)]
    pub(super) fn signal_event(&self, event: EfiEvent) -> EfiStatusEnum {
        (self.signal_event)(event).into_enum()
    }

    #[inline(always)]
    pub(super) fn close_event(&self, event: EfiEvent) -> EfiStatusEnum {
        (self.close_event)(event).into_enum()
    }

    #[inline(always)]
    pub(super) fn check_event(&self, event: EfiEvent) -> EfiStatusEnum {
        (self.check_event)(event).into_enum()
    }
}

pub trait EfiEventAndTimer {
    fn create_event(
        &self,
        event_type: EfiEventType,
        tpl: EfiTaskPriorityLevel,
        notify: Option<(EfiEventNotifyCallback, VoidPtr)>,
    ) -> EfiStatusEnum<EfiEvent>;

    fn set_timer(
        &self,
        event: EfiEvent,
        timer_type: EfiTimerDelay,
        trigger_time: u64,
    ) -> EfiStatusEnum;

    fn wait_for_event(&self, events: &[EfiEvent]) -> EfiStatusEnum<usize>;

    fn signal_event(&self, event: EfiEvent) -> EfiStatusEnum;

    fn close_event(&self, event: EfiEvent) -> EfiStatusEnum;

    fn check_event(&self, event: EfiEvent) -> EfiStatusEnum;
}
