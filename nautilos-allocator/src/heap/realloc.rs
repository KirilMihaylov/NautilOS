use {
    crate::{
        heap::{Heap, HeapEntry},
        heap_error::HeapError,
        memory_range::MemoryRange,
    },
    core::{
        alloc::Layout,
        iter::{from_fn, FromFn},
        ptr::{copy, write_bytes, NonNull},
    },
};

impl Heap {
    fn find_reallocation_address(
        &self,
        range_index: usize,
        layout: Layout,
    ) -> Result<MemoryRange, HeapError> {
        let mut iter = self
            .entries
            .iter()
            .enumerate()
            .filter(|&(index, (free, _)): &(usize, &HeapEntry)| *free || index == range_index)
            .peekable();

        let iter: FromFn<_> = from_fn(move || -> Option<MemoryRange> {
            let (index, (_, mut range)): (usize, HeapEntry) = iter
                .next()
                .map(|(index, &entry): (usize, &HeapEntry)| (index, entry))?;

            'sum_ranges_loop: while let Some(&(next_index, (free, next_range))) = iter.peek() {
                if next_index == range_index || (*free && index == range_index) {
                    if let Some(new_range) = range.add_loose(*next_range) {
                        range = new_range;

                        let _: Option<(usize, &(bool, MemoryRange))> = iter.next();
                    } else {
                        break 'sum_ranges_loop;
                    }
                } else {
                    break 'sum_ranges_loop;
                }
            }

            Some(range)
        });

        Self::find_best_fit(layout, iter).ok_or(HeapError::OutOfMemory)
    }

    fn update_ranges(
        &mut self,
        range_index: usize,
        old_range: MemoryRange,
        new_range: MemoryRange,
    ) -> Result<(), HeapError> {
        let mut remove_range: bool = false;
        let mut add_range: Option<MemoryRange> = None;

        {
            let ranges: &mut [HeapEntry] = {
                let end_index: usize = self.entries.len().min(range_index + 2);

                &mut self.entries.buffer_mut()[range_index..end_index]
            };

            match ranges {
                [(_, range), (true, next_range)] => {
                    *range = new_range;

                    match next_range.subtract(new_range) {
                        (Some(_), _) => return Err(HeapError::InternalError),
                        (None, Some(range)) => *next_range = range,
                        (None, None) => remove_range = true,
                    }
                }
                [(_, range)] | [(_, range), _] if new_range.len() <= old_range.len() => {
                    *range = new_range;

                    match old_range.subtract(new_range) {
                        (Some(_), _) => return Err(HeapError::InternalError),
                        (None, range @ Some(_)) => add_range = range,
                        (None, None) => (),
                    }
                }
                _ => return Err(HeapError::InternalError),
            }
        }

        match (remove_range, add_range) {
            (true, Some(range)) => {
                if let Some(mut entry) = self.entries.get_mut(range_index + 1) {
                    *entry = (true, range);
                } else {
                    return Err(HeapError::InternalError);
                }
            }
            (true, None) => self.entries.remove(range_index + 1),
            (false, Some(range)) => {
                if self.entries.insert((true, range)).is_err() {
                    return Err(HeapError::InternalError);
                }
            }
            _ => (),
        }

        Ok(())
    }

    unsafe fn zero_out_delta(old_range: MemoryRange, new_range: MemoryRange) {
        let (min_end, min_len, max_len): (usize, usize, usize) = (
            old_range.end().min(new_range.end()) + 1,
            old_range.len().min(new_range.len()),
            old_range.len().max(new_range.len()),
        );

        write_bytes(min_end as *mut u8, 0, max_len - min_len);
    }

    #[must_use = "Dropping this value may cause a memory leak."]
    pub(super) fn realloc_inner(
        &mut self,
        address: usize,
        old_layout: Layout,
        new_layout: Layout,
    ) -> Result<NonNull<u8>, HeapError> {
        if old_layout.align() != new_layout.align() {
            return Err(HeapError::DifferentAlignment);
        }

        if address == 0 {
            return self.alloc_from_layout(new_layout);
        }

        let (range_index, old_range) = self
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

        let new_range: MemoryRange = self.find_reallocation_address(range_index, new_layout)?;

        if old_range.start() == new_range.start() {
            if old_range != new_range {
                self.update_ranges(range_index, old_range, new_range)?;

                // Safety: Both ranges are checked and guaranteed to be valid.
                unsafe {
                    Self::zero_out_delta(old_range, new_range);
                }
            }

            NonNull::new(new_range.start() as *mut u8).ok_or(HeapError::InternalError)
        } else {
            let pointer: NonNull<u8> = self.allocate_memory(new_range)?;

            unsafe {
                copy(
                    old_range.start() as *const u8,
                    pointer.as_ptr(),
                    old_range.len().min(new_range.len()),
                );

                match old_range.overlapped(new_range) {
                    Some(overlapped_range) => {
                        let ranges: [Option<MemoryRange>; 2] = {
                            let (left, right): (Option<MemoryRange>, Option<MemoryRange>) =
                                old_range.subtract(overlapped_range);

                            [left, right]
                        };

                        for range in &ranges {
                            if let Some(range) = range {
                                write_bytes(range.start() as *mut u8, 0, range.len());
                            }
                        }
                    }
                    None => write_bytes(old_range.start() as *mut u8, 0, old_range.len()),
                }

                if old_range.len() < new_range.len() {
                    write_bytes(
                        (new_range.end() + 1) as *mut u8,
                        0,
                        new_range.len() - old_range.len(),
                    );
                }
            }

            self.dealloc_from_layout(address as *mut u8, old_layout)?;

            Ok(pointer)
        }
    }

    /// # Errors
    /// TODO
    #[must_use = "Dropping this value may cause a memory leak."]
    pub fn realloc_from_layout(
        &mut self,
        address: *mut u8,
        old_layout: Layout,
        new_layout: Layout,
    ) -> Result<NonNull<u8>, HeapError> {
        self.reallocate_self()?;

        self.realloc_inner(address as usize, old_layout, new_layout)
    }

    /// # Errors
    /// TODO
    #[must_use = "Dropping this value may cause a memory leak."]
    pub fn realloc<T, U>(&mut self, address: *mut U) -> Result<NonNull<T>, HeapError> {
        self.realloc_from_layout(address as *mut u8, Layout::new::<U>(), Layout::new::<U>())
            .map(NonNull::cast)
    }
}
