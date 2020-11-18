use {
    crate::{
        heap::{Heap, HeapEntry},
        heap_error::HeapError,
        memory_range::MemoryRange,
    },
    core::alloc::Layout,
};

impl Heap {
    /// Deallocates a previous allocated memory on the heap while requiring the alignment to be the same as it was during allocation.
    /// # Errors
    /// TODO
    pub fn dealloc_from_layout(
        &mut self,
        address: *mut u8,
        _layout: Layout,
    ) -> Result<(), HeapError> {
        let (mut index, range): (usize, MemoryRange) = self
            .find_range_from_address(address as usize)
            .and_then(
                |(index, (free, range)): (usize, HeapEntry)| {
                    if free {
                        None
                    } else {
                        Some((index, range))
                    }
                },
            )
            .ok_or(HeapError::NoSuchMemoryRange)?;

        unsafe {
            Self::zero_out(address as usize, range.len());
        }

        self.entries
            .get_mut(index)
            .ok_or(HeapError::InternalError)?
            .0 = true;

        index = index.saturating_sub(1);

        while let Some(entry) = self
            .entries
            .get(index.saturating_add(1))
            .and_then(|entry: &HeapEntry| if entry.0 { Some(*entry) } else { None })
        {
            let remove_entry: bool;

            {
                let mut left_entry: _ = self
                    .entries
                    .get_mut(index)
                    .ok_or(HeapError::InternalError)?;
                let left_entry: &mut HeapEntry = &mut *left_entry;

                // "entry" already checked in the ".and_then" call
                if left_entry.0 {
                    if let Some(new_range) = left_entry.1.add_loose(entry.1) {
                        left_entry.1 = new_range;

                        remove_entry = true;
                    } else {
                        remove_entry = false;
                    }
                } else {
                    remove_entry = false;
                }
            }

            if remove_entry {
                self.entries.remove(index.saturating_add(1));
            } else if let Some(new_index) = index.checked_add(1) {
                index = new_index;
            } else {
                break;
            }
        }

        self.reallocate_self()?;

        Ok(())
    }

    /// # Errors
    /// TODO
    #[must_use = "Return error may indicate that the heap is poisoned!"]
    pub fn dealloc<T>(&mut self, address: *mut T) -> Result<(), HeapError> {
        self.dealloc_from_layout(address as *mut u8, Layout::new::<T>())
    }
}
