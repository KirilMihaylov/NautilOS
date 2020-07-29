use crate::*;

pub type EfiTaskPriorityLevel = usize;

#[repr(C)]
#[derive(Clone, Copy)]
pub(super) struct EfiTaskPriorityRaw {
    raise_tpl: extern "efiapi" fn(EfiTaskPriorityLevel) -> EfiStatus,
    restore_tpl: extern "efiapi" fn(EfiTaskPriorityLevel) -> Void,
}

impl EfiTaskPriorityRaw {
    #[inline(always)]
    pub(super) fn raise_priority_level(
        &self,
        new_priority_level: EfiTaskPriorityLevel,
    ) -> EfiStatusEnum {
        (self.raise_tpl)(new_priority_level).into_enum()
    }

    #[inline(always)]
    pub(super) fn restore_priority_level(&self, old_priority_level: EfiTaskPriorityLevel) {
        (self.restore_tpl)(old_priority_level);
    }
}

pub trait EfiTaskPriority {
    fn raise_priority_level(&self, new_priority_level: EfiTaskPriorityLevel) -> EfiStatusEnum;

    fn restore_priority_level(&self, old_priority_level: EfiTaskPriorityLevel);
}
