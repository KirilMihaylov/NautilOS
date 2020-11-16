pub(crate) mod initialization_error;
mod split_memory;

use {
    crate::{
        heap::{Heap, HeapEntry},
        memory_range::MemoryRange,
        sorted_list::SortedList,
    },
    core::{cmp::Ordering, mem::size_of, ptr::write},
    initialization_error::InitializationError,
    split_memory::split_memory,
};

impl Heap {
    /// # Errors
    /// ### [`Internal Error`](variant@InitializationError::InternalError)
    /// * Returned in case of malfunction of the function or it's logic's components.
    /// ### [`Not Enough Initialization Memory`](variant@InitializationError::NotEnoughInitMemory)
    /// * Returned when the provided memory buffer is too small to hold the preallocated heap entries.
    pub fn init_in_place(
        memory: &'static mut [u8],
    ) -> Result<&'static mut Self, InitializationError> {
        let (prefix_memory_1, heap, suffix_memory): (&mut [u8], &mut [Self], &mut [u8]) =
            unsafe { split_memory(memory, 1)? };

        if heap.len() != 1 {
            return Err(InitializationError::InternalError);
        }

        let heap: &mut Self = &mut heap[0];

        let (prefix_memory_2, entries_memory, suffix_memory): (
            &mut [u8],
            &mut [HeapEntry],
            &mut [u8],
        ) = unsafe { split_memory(suffix_memory, Self::PREALLOCATED_ENTRIES_COUNT)? };

        let mut entries: SortedList<'static, HeapEntry> = SortedList::new(
            entries_memory,
            |(_, x): &HeapEntry, (_, y): &HeapEntry| -> Ordering { x.start().cmp(&y.start()) },
        );

        for (free, start, length) in &[
            (
                true,
                prefix_memory_1.as_ptr() as usize,
                prefix_memory_1.len(),
            ),
            (
                true,
                prefix_memory_2.as_ptr() as usize,
                prefix_memory_2.len(),
            ),
            (true, suffix_memory.as_ptr() as usize, suffix_memory.len()),
            (false, heap as *mut Self as usize, size_of::<Self>()),
            (
                false,
                entries.internal_buffer() as usize,
                entries.capacity() * size_of::<HeapEntry>(),
            ),
        ] {
            if *length != 0 {
                if let Some(range) = MemoryRange::new_with_length(*start, *length) {
                    if entries.insert((*free, range)).is_err() {
                        return Err(InitializationError::InternalError);
                    }
                }
            }
        }

        unsafe {
            write(heap, Self::new(entries));
        }

        heap.reallocate_self()
            .map_err(InitializationError::HeapError)?;

        Ok(heap)
    }
}
