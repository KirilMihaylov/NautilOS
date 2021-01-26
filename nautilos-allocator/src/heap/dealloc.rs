use {
    crate::{heap::Heap, heap_error::HeapError, memory_range::MemoryRange},
    core::alloc::Layout,
};

impl Heap<'_> {
    /// Deallocates a previous allocated memory on the heap while requiring the alignment to be the same as it was during allocation.
    /// # Errors
    /// TODO
    pub fn dealloc_from_layout(
        &mut self,
        address: *mut u8,
        layout: Layout,
    ) -> Result<(), HeapError> {
        let (store_index, entry_index, range): (usize, usize, MemoryRange) =
            if let Some(data) = self.find_range_from_address(layout.align(), address as usize) {
                data.unzip()
            } else {
                return Err(HeapError::NoSuchMemoryRange);
            };

        if let Some(mut store) = self.allocated.get_mut(store_index) {
            store.1.remove(entry_index);
        } else {
            return Err(HeapError::InternalError);
        }

        unsafe {
            Self::zero_out(address as usize, range.len());
        }

        if self.free.insert(range).is_err() {
            return Err(HeapError::InternalError);
        }

        if let error @ Err(_) = self.defragment() {
            return error;
        }

        if let error @ Err(_) = self.reallocate_self() {
            return error;
        }

        Ok(())
    }

    /// # Errors
    /// TODO
    #[must_use = "Return error may indicate that the heap is poisoned!"]
    pub fn dealloc<T>(&mut self, address: *mut T) -> Result<(), HeapError> {
        self.dealloc_from_layout(address as *mut u8, Layout::new::<T>())
    }
}
