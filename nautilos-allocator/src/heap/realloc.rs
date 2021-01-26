use {
    crate::{
        heap::{fold_elements::FoldElements, Heap},
        heap_error::HeapError,
        memory_range::MemoryRange,
    },
    core::{
        alloc::Layout,
        ops::RangeInclusive,
        ptr::{copy, write_bytes, NonNull},
    },
};

impl Heap<'_> {
    fn find_reallocation_address(
        &self,
        old_range: MemoryRange,
        layout: Layout,
    ) -> Result<(RangeInclusive<usize>, MemoryRange), HeapError> {
        Self::find_best_fit(
            layout,
            self.free
                .iter()
                .enumerate()
                .map(|(index, &range): (usize, &MemoryRange)| {
                    (
                        index..=index,
                        range
                            .add_loose(old_range)
                            .map_or(range, |range: MemoryRange| range),
                    )
                })
                .fold_elements(
                    |cumulative: (RangeInclusive<usize>, MemoryRange),
                     element: (RangeInclusive<usize>, MemoryRange)| {
                        cumulative.1.add_loose(element.1).map(|range: MemoryRange| {
                            (*cumulative.0.start()..=*element.0.end(), range)
                        })
                    },
                ),
        )
        .ok_or(HeapError::OutOfMemory)
    }

    unsafe fn zero_out_delta(old_range: MemoryRange, new_range: MemoryRange) {
        match old_range.subtract(new_range) {
            (Some(left), Some(right)) => {
                write_bytes(left.start() as *mut u8, 0, left.len());
                write_bytes(right.start() as *mut u8, 0, right.len());
            }
            (Some(range), None) | (None, Some(range)) => {
                write_bytes(range.start() as *mut u8, 0, range.len());
            }
            (None, None) => (),
        }
    }

    #[must_use = "Dropping this value may cause a memory leak."]
    pub(super) fn realloc_inner(
        &mut self,
        address: usize,
        layout: Layout,
    ) -> Result<NonNull<u8>, HeapError> {
        if address == 0 {
            return self.alloc_from_layout(layout);
        }

        let (store_index, range_index, old_range): (usize, usize, MemoryRange) =
            if let Some(range) = self.find_range_from_address(layout.align(), address as usize) {
                range.unzip()
            } else {
                return Err(HeapError::NoSuchMemoryRange);
            };

        let (indexes_range, new_range): (RangeInclusive<usize>, MemoryRange) =
            match self.find_reallocation_address(old_range, layout) {
                Ok(data) => data,
                Err(error) => return Err(error),
            };

        let (left_free_range, right_free_range): (Option<MemoryRange>, Option<MemoryRange>) =
            MemoryRange::new(
                self.free[*indexes_range.start()].start(),
                self.free[*indexes_range.end()].end(),
            )
            .map_or((None, None), |range: MemoryRange| range.subtract(new_range));

        self.free.remove_range(indexes_range);

        if let Some(range) = left_free_range {
            if self.free.insert(range).is_err() {
                return Err(HeapError::InternalError);
            }
        }

        if let Some(range) = right_free_range {
            if self.free.insert(range).is_err() {
                return Err(HeapError::InternalError);
            }
        }

        if let Some(mut store) = self.allocated.get_mut(store_index) {
            if let Some(mut range) = store.1.get_mut(range_index) {
                *range = new_range;
            } else {
                return Err(HeapError::InternalError);
            }
        } else {
            return Err(HeapError::InternalError);
        }

        unsafe {
            copy(
                old_range.start() as *const u8,
                new_range.start() as *mut u8,
                old_range.len().min(new_range.len()),
            );

            Self::zero_out_delta(old_range, new_range);
        }

        Ok(
            if let Some(ptr) = NonNull::new(new_range.start() as *mut u8) {
                ptr
            } else {
                return Err(HeapError::InternalError);
            },
        )
    }

    /// # Errors
    /// TODO
    #[must_use = "Dropping this value may cause a memory leak."]
    pub fn realloc_from_layout(
        &mut self,
        address: *mut u8,
        layout: Layout,
    ) -> Result<NonNull<u8>, HeapError> {
        if let Err(error) = self.reallocate_self() {
            return Err(error);
        }

        self.realloc_inner(address as usize, layout)
    }

    /// # Errors
    /// TODO
    #[must_use = "Dropping this value may cause a memory leak."]
    pub fn realloc<T, U>(&mut self, address: *mut T) -> Result<NonNull<U>, HeapError> {
        let layout: Layout = Layout::new::<U>();

        if layout.align() != Layout::new::<T>().align() {
            return Err(HeapError::DifferentAlignment);
        }

        self.realloc_from_layout(address as *mut u8, layout)
            .map(NonNull::cast)
    }
}
