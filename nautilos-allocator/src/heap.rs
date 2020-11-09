mod alloc;
mod find_best_fit;
mod find_range_from_address;
pub(crate) mod init_in_place;
mod list_debug_formatter;
mod realloc;
mod reallocate_self;
mod zero_out;

use {
    crate::{heap_error::HeapError, memory_range::MemoryRange, sorted_list::SortedList},
    core::{
        alloc::Layout,
        fmt::{Debug, Formatter, Result as FmtResult},
    },
    list_debug_formatter::ListDebugFormatter,
};

type HeapEntry = (bool, MemoryRange);

#[repr(transparent)]
#[must_use]
pub struct Heap {
    entries: SortedList<'static, HeapEntry>,
}

impl Heap {
    const PREALLOCATED_ENTRIES_COUNT: usize = Self::REALLOCATION_THRESHOLD * 32;

    const REALLOCATION_THRESHOLD: usize = 16;

    const REALLOCATION_COEFFICIENT: usize = 4;

    const REALLOCATION_ADDITIONAL_LENGTH: usize =
        Self::REALLOCATION_THRESHOLD * Self::REALLOCATION_COEFFICIENT;

    fn new(entries: SortedList<'static, HeapEntry>) -> Self {
        Self { entries }
    }

    /// # Errors
    /// TODO
    pub fn dealloc_from_layout(
        &mut self,
        address: *mut u8,
        _layout: Layout,
    ) -> Result<(), HeapError> {
        let (index, range): (usize, MemoryRange) = self
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

        // TODO: Combine free ranges

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

impl Debug for Heap {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        f.debug_struct("Heap")
            .field(
                "Total Memory Length",
                &self
                    .entries
                    .iter()
                    .fold(0, |accumulated: usize, (_, range): &HeapEntry| {
                        accumulated + range.len()
                    }),
            )
            .field(
                "Allocated",
                &ListDebugFormatter::new(&self.entries.iter().filter_map(
                    |&(free, range): &HeapEntry| if free { None } else { Some(range) },
                )),
            )
            .field(
                "Free",
                &ListDebugFormatter::new(&self.entries.iter().filter_map(
                    |&(free, range): &HeapEntry| if free { Some(range) } else { None },
                )),
            )
            .finish()
    }
}
