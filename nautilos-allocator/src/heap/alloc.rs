use {
    crate::{
        heap::{AllocatedHeapEntries, Heap},
        heap_error::HeapError,
        memory_range::MemoryRange,
        sorted_list::SortedList,
    },
    core::{alloc::Layout, mem::align_of, ptr::NonNull},
};

impl Heap<'_> {
    fn find_allocation_address(&self, layout: Layout) -> Result<(usize, MemoryRange), HeapError> {
        Self::find_best_fit(layout, self.free.iter().copied().enumerate())
            .ok_or(HeapError::OutOfMemory)
    }

    #[must_use = "Dropping this value may cause a memory leak."]
    fn allocate_memory(
        &mut self,
        layout: Layout,
        index: usize,
        range: MemoryRange,
    ) -> Result<NonNull<u8>, HeapError> {
        type AllocatedHeapEntriesBufferType =
            [MemoryRange; AllocatedHeapEntries::PREALLOCATED_ENTRIES_COUNT];
        const ALLOCATED_HEAP_ENTRIES_LAYOUT: Layout =
            Layout::new::<AllocatedHeapEntriesBufferType>();

        let allocated_store_index: usize = match self.allocated.binary_search_by(
            |(alignment, _): &(usize, AllocatedHeapEntries<'_>)| alignment.cmp(&layout.align()),
        ) {
            Ok(index) => index,
            Err(_) if layout.align() == align_of::<AllocatedHeapEntries>() => {
                return Err(HeapError::InternalError);
            }
            Err(_) => {
                let buffer: &mut [MemoryRange] =
                    match self.alloc_inner(ALLOCATED_HEAP_ENTRIES_LAYOUT) {
                        Ok(ptr) => unsafe {
                            &mut *ptr.cast::<AllocatedHeapEntriesBufferType>().as_ptr()
                        },
                        error @ Err(_) => return error,
                    };

                let inner: SortedList<MemoryRange> =
                    SortedList::new(buffer, |x: &MemoryRange, y: &MemoryRange| {
                        x.start().cmp(&y.start())
                    });

                if self
                    .allocated
                    .insert((layout.align(), AllocatedHeapEntries::new(inner)))
                    .is_err()
                {
                    return Err(HeapError::InternalError);
                }

                let search_fn = |(alignment, _): &(usize, AllocatedHeapEntries<'_>)| {
                    alignment.cmp(&layout.align())
                };

                if self.allocated.binary_search_by(search_fn).is_err() {
                    return Err(HeapError::InternalError);
                }

                return self.alloc_inner(layout);
            }
        };

        let memory: MemoryRange = if let Some(&range) = self.free.get(index) {
            range
        } else {
            return Err(HeapError::InternalError);
        };

        match memory.subtract(range) {
            (Some(left_range), Some(right_range)) => {
                if let Some(mut range) = self.free.get_mut(index) {
                    *range = left_range;
                } else {
                    return Err(HeapError::InternalError);
                }

                if self.free.insert(right_range).is_err() {
                    return Err(HeapError::InternalError);
                }
            }
            (Some(new_range), None) | (None, Some(new_range)) => {
                if let Some(mut range) = self.free.get_mut(index) {
                    *range = new_range;
                } else {
                    return Err(HeapError::InternalError);
                }
            }
            (None, None) => {
                self.free.remove(index);
            }
        };

        let is_err: bool = if let Some(mut store) = self.allocated.get_mut(allocated_store_index) {
            store.1.insert(range).is_err()
        } else {
            return Err(HeapError::InternalError);
        };

        if is_err {
            return Err(HeapError::InternalError);
        }

        unsafe {
            Heap::zero_out(range.start(), range.len());
        }

        NonNull::new(range.start() as *mut u8).ok_or(HeapError::InternalError)
    }

    #[must_use = "Dropping this value may cause a memory leak."]
    pub(in crate::heap) fn alloc_inner(
        &mut self,
        layout: Layout,
    ) -> Result<NonNull<u8>, HeapError> {
        let (index, range): (usize, MemoryRange) = match self.find_allocation_address(layout) {
            Ok(data) => data,
            Err(error) => return Err(error),
        };

        self.allocate_memory(layout, index, range)
    }

    /// Allocates memory from a given layout.
    /// # Errors
    /// Returns [`Err`] when a memory allocation error occures.
    #[must_use = "Dropping this value may cause a memory leak."]
    pub fn alloc_from_layout(&mut self, layout: Layout) -> Result<NonNull<u8>, HeapError> {
        if let Err(error) = self.reallocate_self() {
            return Err(error);
        };

        self.alloc_inner(layout)
    }

    /// Allocates memory for a given, known at compile time, type.
    /// # Errors
    /// Returns [`Err`] when a memory allocation error occures.
    #[must_use = "Dropping this value may cause a memory leak."]
    pub fn alloc<T>(&mut self) -> Result<NonNull<T>, HeapError> {
        self.alloc_from_layout(Layout::new::<T>())
            .map(NonNull::cast)
    }
}
