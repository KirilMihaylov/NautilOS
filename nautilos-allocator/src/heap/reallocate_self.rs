use {
    crate::{
        heap::{AllocatedHeapEntries, AllocatedHeapEntriesLists, FreeHeapEntries, Heap},
        heap_error::HeapError,
        memory_range::MemoryRange,
        sorted_list::Element,
    },
    core::{
        alloc::{Layout, LayoutError},
        slice::from_raw_parts_mut,
    },
};

impl Heap<'_> {
    pub(super) fn reallocate_self(&mut self) -> Result<(), HeapError> {
        if self.free.needs_reallocation() {
            if let error @ Err(_) = self.reallocate_free() {
                return error;
            }
        }

        if self.allocated.needs_reallocation() {
            if let error @ Err(_) = self.reallocate_allocated_lists() {
                return error;
            }
        }

        if let error @ Err(_) = self.reallocate_allocated() {
            return error;
        }

        Ok(())
    }

    fn reallocate_free(&mut self) -> Result<(), HeapError> {
        let new_size: usize = if let Some(data) = self
            .free
            .capacity()
            .checked_add(FreeHeapEntries::REALLOCATION_ADDITIONAL_SIZE)
        {
            data
        } else {
            return Err(HeapError::InternalError);
        };

        let mut address: *mut MemoryRange = self.free.internal_buffer();

        address = match self.realloc_inner(address as usize, {
            let layout: Result<Layout, LayoutError> = Layout::array::<MemoryRange>(new_size);

            if let Ok(layout) = layout {
                layout
            } else {
                return Err(HeapError::InternalError);
            }
        }) {
            Ok(ptr) => ptr.cast().as_ptr(),
            Err(error) => return Err(error),
        };

        self.free
            .set_internal_buffer(unsafe { from_raw_parts_mut(address, new_size) });

        Ok(())
    }

    fn reallocate_allocated_lists(&mut self) -> Result<(), HeapError> {
        let new_size: usize = if let Some(data) = self
            .allocated
            .capacity()
            .checked_add(AllocatedHeapEntriesLists::REALLOCATION_ADDITIONAL_SIZE)
        {
            data
        } else {
            return Err(HeapError::InternalError);
        };

        let mut address: *mut (usize, AllocatedHeapEntries<'_>) = self.allocated.internal_buffer();

        address = match self.realloc_inner(address as usize, {
            let layout: Result<Layout, LayoutError> =
                Layout::array::<(usize, AllocatedHeapEntries<'_>)>(new_size);

            if let Ok(layout) = layout {
                layout
            } else {
                return Err(HeapError::InternalError);
            }
        }) {
            Ok(ptr) => ptr.cast().as_ptr(),
            Err(error) => return Err(error),
        };

        self.allocated
            .set_internal_buffer(unsafe { from_raw_parts_mut(address, new_size) });

        Ok(())
    }

    fn reallocate_allocated(&mut self) -> Result<(), HeapError> {
        'allocated_stores: for index in 0..self.allocated.len() {
            let mut address: *mut MemoryRange;
            let capacity: usize;
            let new_size: usize;

            {
                let mut allocated: Element<'_, '_, (usize, AllocatedHeapEntries)> =
                    if let Some(store) = self.allocated.get_mut(index) {
                        store
                    } else {
                        return Err(HeapError::InternalError);
                    };
                let (_, allocated): &mut (usize, AllocatedHeapEntries) = &mut *allocated;

                if !allocated.needs_reallocation() {
                    continue 'allocated_stores;
                }

                address = allocated.internal_buffer();

                capacity = allocated.capacity();

                new_size = if let Some(data) =
                    capacity.checked_add(AllocatedHeapEntries::REALLOCATION_ADDITIONAL_SIZE)
                {
                    data
                } else {
                    return Err(HeapError::InternalError);
                };
            }

            address = match self.realloc_inner(address as usize, {
                let layout: Result<Layout, LayoutError> =
                    Layout::array::<(usize, AllocatedHeapEntries<'_>)>(new_size);

                if let Ok(layout) = layout {
                    layout
                } else {
                    return Err(HeapError::InternalError);
                }
            }) {
                Ok(ptr) => ptr.cast().as_ptr(),
                Err(error) => return Err(error),
            };

            if let Some(mut store) = self.allocated.get_mut(index) {
                store
                    .1
                    .set_internal_buffer(unsafe { from_raw_parts_mut(address, new_size) });
            } else {
                return Err(HeapError::InternalError);
            }
        }

        Ok(())
    }
}
