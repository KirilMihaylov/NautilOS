use {
    crate::{heap::Heap, heap::HeapEntry, heap_error::HeapError, memory_range::MemoryRange},
    core::{alloc::Layout, cmp::Ordering, ptr::NonNull},
};

impl Heap {
    fn find_allocation_address(&self, layout: Layout) -> Result<MemoryRange, HeapError> {
        Self::find_best_fit(
            layout,
            self.entries.iter().filter_map(
                |&(free, memory): &HeapEntry| -> Option<MemoryRange> {
                    if free {
                        Some(memory)
                    } else {
                        None
                    }
                },
            ),
        )
        .ok_or(HeapError::OutOfMemory)
    }

    #[must_use = "Dropping this value may cause a memory leak."]
    pub(super) fn allocate_memory(&mut self, range: MemoryRange) -> Result<NonNull<u8>, HeapError> {
        let index: usize = match self.entries.binary_search_by(|(_, memory): &HeapEntry| {
            match memory.overlapped(range) {
                Some(overlap) if overlap == range => Ordering::Equal,
                _ => memory.cmp(&range),
            }
        }) {
            Ok(index) => index,
            Err(_) => return Err(HeapError::NoSuchMemoryRange),
        };

        let memory: MemoryRange = {
            let (free, memory) = self.entries[index];

            if !free {
                return Err(HeapError::InternalError);
            }

            memory
        };

        match memory.subtract(range) {
            (Some(left_range), Some(right_range)) => {
                *self
                    .entries
                    .get_mut(index)
                    .ok_or(HeapError::InternalError)? = (true, left_range);

                if self.entries.insert((false, range)).is_err() {
                    return Err(HeapError::InternalError);
                }

                if self.entries.insert((true, right_range)).is_err() {
                    return Err(HeapError::InternalError);
                }
            }
            (Some(new_range), None) | (None, Some(new_range)) => {
                *self
                    .entries
                    .get_mut(index)
                    .ok_or(HeapError::InternalError)? = (false, range);

                if self.entries.insert((true, new_range)).is_err() {
                    return Err(HeapError::InternalError);
                }
            }
            _ => {
                self.entries
                    .get_mut(index)
                    .ok_or(HeapError::InternalError)?
                    .0 = false
            }
        };

        NonNull::new(range.start() as *mut u8).ok_or(HeapError::InternalError)
    }

    #[must_use = "Dropping this value may cause a memory leak."]
    pub(super) fn alloc_inner(&mut self, layout: Layout) -> Result<NonNull<u8>, HeapError> {
        self.allocate_memory(self.find_allocation_address(layout)?)
    }

    /// Allocates memory from a given layout.
    /// # Errors
    /// Returns [`Err`] when a memory allocation error occures.
    #[must_use = "Dropping this value may cause a memory leak."]
    pub fn alloc_from_layout(&mut self, layout: Layout) -> Result<NonNull<u8>, HeapError> {
        self.reallocate_self()?;

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
