use crate::*;

#[repr(C)]
#[derive(Clone, Copy)]
pub(super) struct EfiMiscellaneousRaw {
    get_next_monotonic_count: extern "efiapi" fn(*mut u64) -> EfiStatus,
    stall: extern "efiapi" fn(usize) -> EfiStatus,
    set_watchdog_timer: extern "efiapi" fn(usize, u64, usize, *const u16) -> EfiStatus,
}

impl EfiMiscellaneousRaw {
    pub(super) fn get_next_monotonic_count(&self) -> EfiStatusEnum<(u32, u32)> {
        let mut count: u64 = 0;

        (self.get_next_monotonic_count)(&mut count)
            .into_enum_data(|| ((count >> 32) as u32, count as u32))
    }

    pub(super) fn stall(&self, microseconds: usize) -> EfiStatusEnum {
        (self.stall)(microseconds).into_enum()
    }

    pub(super) fn set_watchdog_timer(
        &self,
        timeout: usize,
        watchdog_code: u64,
        watchdog_data: Option<&[u16]>,
    ) -> EfiStatusEnum {
        let (watchdog_data_ptr, watchdog_data_len): (*const u16, usize) =
            if let Some(watchdog_data) = watchdog_data {
                (watchdog_data.as_ptr(), watchdog_data.len() * 2)
            } else {
                (0 as _, 0)
            };

        (self.set_watchdog_timer)(timeout, watchdog_code, watchdog_data_len, watchdog_data_ptr)
            .into_enum()
    }
}

pub trait EfiMiscellaneous {
    fn get_next_monotonic_count(&self) -> EfiStatusEnum<(u32, u32)>;

    fn stall(&self, microseconds: usize) -> EfiStatusEnum;

    fn set_watchdog_timer(
        &self,
        timeout: usize,
        watchdog_code: u64,
        watchdog_data: Option<&[u16]>,
    ) -> EfiStatusEnum;
}
