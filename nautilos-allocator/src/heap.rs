mod alloc;
mod dealloc;
mod find_best_fit;
mod find_range_from_address;
pub(crate) mod init_in_place;
mod list_debug_formatter;
mod realloc;
mod reallocate_self;
mod zero_out;

use {
    crate::{memory_range::MemoryRange, sorted_list::SortedList},
    core::{
        fmt::{Debug, Formatter, Result as FmtResult},
        mem::{align_of, size_of},
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

    const REALLOCATION_ADDITIONAL_SIZE: usize =
        Self::REALLOCATION_THRESHOLD * Self::REALLOCATION_COEFFICIENT;

    pub const UNALIGNED_REQUIRED_INITIAL_SIZE: usize = size_of::<Self>()
        + align_of::<SortedList<HeapEntry>>()
        + (size_of::<HeapEntry>() * Self::PREALLOCATED_ENTRIES_COUNT);

    pub const ALIGNED_REQUIRED_INITIAL_SIZE: usize =
        Self::UNALIGNED_REQUIRED_INITIAL_SIZE - align_of::<Self>();

    fn new(entries: SortedList<'static, HeapEntry>) -> Self {
        Self { entries }
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
