pub(crate) mod initialization_error;
mod split_memory;
mod static_assert;

use {
    crate::{
        heap::{AllocatedHeapEntries, AllocatedHeapEntriesLists, FreeHeapEntries, Heap},
        memory_range::MemoryRange,
        sorted_list::SortedList,
    },
    core::{cmp::Ordering, mem::align_of, ptr::write},
    initialization_error::InitializationError,
    split_memory::split_memory,
};

impl<'a> Heap<'a> {
    /// # Errors
    /// ### [`Internal Error`](variant@InitializationError::InternalError)
    /// * Returned in case of malfunction of the function or it's logic's components.
    /// ### [`Not Enough Initialization Memory`](variant@InitializationError::NotEnoughInitMemory)
    /// * Returned when the provided memory buffer is too small to hold the preallocated heap entries.
    pub fn init_in_place(memory: &'a mut [u8]) -> Result<&'a mut Self, InitializationError> {
        let (prefix_memory, heap, suffix_memory): (&mut [u8], &mut [Self], &mut [u8]) =
            match unsafe { split_memory(memory, 1) } {
                Ok(data) => data,
                Err(error) => return Err(error),
            };

        if heap.len() != 1 {
            return Err(InitializationError::InternalError);
        }

        let heap: &mut Self = &mut heap[0];

        // The prefix memory is asserted at compile time to be equal to zero.
        let (_, free_entries, suffix_memory): (&mut [u8], &mut [MemoryRange], &mut [u8]) = match unsafe {
            split_memory(suffix_memory, FreeHeapEntries::PREALLOCATED_ENTRIES_COUNT)
        } {
            Ok(data) => data,
            Err(error) => return Err(error),
        };

        let free_entries_ptr: usize = free_entries.as_ptr() as usize;

        let mut free_entries: FreeHeapEntries<'a> = FreeHeapEntries::new(SortedList::new(
            free_entries,
            |x: &MemoryRange, y: &MemoryRange| -> Ordering { x.start().cmp(&y.start()) },
        ));

        // The prefix memory is asserted at compile time to be equal to zero.
        let (_, allocated_entries_lists, suffix_memory): (
            &mut [u8],
            &mut [(usize, AllocatedHeapEntries<'a>)],
            &mut [u8],
        ) = match unsafe {
            split_memory(
                suffix_memory,
                AllocatedHeapEntriesLists::PREALLOCATED_ENTRIES_COUNT,
            )
        } {
            Ok(data) => data,
            Err(error) => return Err(error),
        };

        let allocated_entries_lists_ptr: usize = allocated_entries_lists.as_ptr() as usize;

        let mut allocated_entries_lists: AllocatedHeapEntriesLists<'a> =
            AllocatedHeapEntriesLists::new(SortedList::new(
                allocated_entries_lists,
                |(x, _): &(usize, AllocatedHeapEntries<'a>),
                 (y, _): &(usize, AllocatedHeapEntries<'a>)|
                 -> Ordering { x.cmp(y) },
            ));

        // The prefix memory is asserted at compile time to be equal to zero.
        let (_, allocated_entries, suffix_memory): (&mut [u8], &mut [MemoryRange], &mut [u8]) =
            match unsafe {
                split_memory(
                    suffix_memory,
                    AllocatedHeapEntries::PREALLOCATED_ENTRIES_COUNT,
                )
            } {
                Ok(data) => data,
                Err(error) => return Err(error),
            };

        let allocated_entries_ptr: usize = allocated_entries.as_ptr() as usize;

        let mut allocated_entries: AllocatedHeapEntries<'a> =
            AllocatedHeapEntries::new(SortedList::new(
                allocated_entries,
                |x: &MemoryRange, y: &MemoryRange| -> Ordering { x.start().cmp(&y.start()) },
            ));

        for entry in &[prefix_memory, suffix_memory] {
            if let Some(range) = MemoryRange::new_with_length(entry.as_ptr() as usize, entry.len())
            {
                if free_entries.insert(range).is_err() {
                    return Err(InitializationError::InternalError);
                }
            }
        }

        for entry in &[
            (free_entries_ptr, FreeHeapEntries::REQUIRED_INITIAL_SIZE),
            (
                allocated_entries_lists_ptr,
                AllocatedHeapEntriesLists::REQUIRED_INITIAL_SIZE,
            ),
            (
                allocated_entries_ptr,
                AllocatedHeapEntries::REQUIRED_INITIAL_SIZE,
            ),
        ] {
            if let Some(range) = MemoryRange::new_with_length(entry.0, entry.1) {
                if allocated_entries.insert(range).is_err() {
                    return Err(InitializationError::InternalError);
                }
            }
        }

        if allocated_entries_lists
            .insert((align_of::<[MemoryRange; 0]>(), allocated_entries))
            .is_err()
        {
            return Err(InitializationError::InternalError);
        }

        unsafe {
            write(heap, Self::new(free_entries, allocated_entries_lists));
        }

        if let Err(error) = heap.reallocate_self() {
            return Err(InitializationError::HeapError(error));
        }

        // Sanity check
        if heap.defragment().and(heap.reallocate_self()).is_err() {
            return Err(InitializationError::InternalError);
        }

        Ok(heap)
    }
}
