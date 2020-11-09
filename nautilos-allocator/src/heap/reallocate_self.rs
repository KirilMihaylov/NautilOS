use {
    crate::{
        heap::{Heap, HeapEntry},
        heap_error::HeapError,
    },
    core::{alloc::Layout, slice::from_raw_parts_mut},
};

impl Heap {
    const fn needs_reallocation(&self) -> bool {
        self.entries.capacity() - self.entries.len() < Self::REALLOCATION_THRESHOLD
    }

    pub(super) fn reallocate_self(&mut self) -> Result<(), HeapError> {
        if self.needs_reallocation() {
            let new_size: usize = self.entries.capacity() + Self::REALLOCATION_ADDITIONAL_LENGTH;

            unsafe {
                let mut address: *mut HeapEntry = self.entries.internal_buffer();

                address = self
                    .realloc_inner(
                        address as usize,
                        Layout::array::<HeapEntry>(self.entries.capacity())
                            .ok()
                            .map_or(Err(HeapError::InternalError), Ok)?,
                        Layout::array::<HeapEntry>(new_size)
                            .ok()
                            .map_or(Err(HeapError::InternalError), Ok)?,
                    )?
                    .cast()
                    .as_ptr();

                self.entries
                    .set_internal_buffer(from_raw_parts_mut(address, new_size));
            }
        }

        Ok(())
    }
}
