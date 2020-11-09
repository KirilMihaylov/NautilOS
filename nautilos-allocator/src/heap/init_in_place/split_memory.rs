use {
    crate::heap::init_in_place::initialization_error::InitializationError,
    core::mem::{align_of, size_of},
};

pub(super) type SplitMemoryResult<'a, T> =
    Result<(&'a mut [u8], &'a mut [T], &'a mut [u8]), InitializationError>;

/// # Safety
/// Underlying logic is based on [`core::mem::transmute`].
/// Same safety requirements apply.
pub(super) unsafe fn split_memory<'a, T>(
    memory: &'a mut [u8],
    count: usize,
) -> SplitMemoryResult<'a, T> {
    let required_memory: usize = {
        let size: usize = count * size_of::<T>();
        let align_offset: usize = memory.as_ptr() as usize % align_of::<T>();

        size + if align_offset == 0 {
            0
        } else {
            align_of::<T>() - align_offset
        }
    };

    if memory.len() < required_memory {
        Err(InitializationError::NotEnoughInitMemory)
    } else {
        let (prefix, memory, suffix): (&'a mut [u8], &'a mut [T], &'a mut [u8]) = {
            let (left, right): (&'a mut [u8], &'a mut [u8]) = memory.split_at_mut(required_memory);

            // Safety: Memory will be used as buffer and will be overwritten the moment when element is inserted.
            let (prefix, buffer, suffix): (&'a mut [u8], &'a mut [T], &'a mut [u8]) =
                left.align_to_mut();

            if !suffix.is_empty() {
                return Err(InitializationError::InternalError);
            }

            (prefix, buffer, right)
        };

        if memory.len() != count {
            return Err(InitializationError::InternalError);
        }

        Ok((prefix, memory, suffix))
    }
}
