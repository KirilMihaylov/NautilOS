mod alloc;
mod dealloc;
mod defragment;
mod find_best_fit;
mod find_range_from_address;
mod fold_elements;
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
        ops::{Deref, DerefMut},
    },
    list_debug_formatter::ListDebugFormatter,
};

#[must_use]
pub struct Heap<'a> {
    free: FreeHeapEntries<'a>,
    allocated: AllocatedHeapEntriesLists<'a>,
}

impl<'a> Heap<'a> {
    pub const ALIGNED_REQUIRED_INITIAL_SIZE: usize = size_of::<Self>()
        + (size_of::<Self>() % align_of::<[MemoryRange; 0]>())
        + (size_of::<[MemoryRange; FreeHeapEntries::PREALLOCATED_ENTRIES_COUNT]>()
            % align_of::<[(usize, AllocatedHeapEntries<'_>); 0]>())
        + (size_of::<
            [(usize, AllocatedHeapEntries<'_>);
                AllocatedHeapEntriesLists::PREALLOCATED_ENTRIES_COUNT],
        >() % align_of::<[MemoryRange; 0]>())
        + FreeHeapEntries::REQUIRED_INITIAL_SIZE
        + AllocatedHeapEntriesLists::REQUIRED_INITIAL_SIZE
        + AllocatedHeapEntries::REQUIRED_INITIAL_SIZE;

    pub const UNALIGNED_REQUIRED_INITIAL_SIZE: usize =
        Self::ALIGNED_REQUIRED_INITIAL_SIZE + align_of::<Self>();

    fn new(free: FreeHeapEntries<'a>, allocated: AllocatedHeapEntriesLists<'a>) -> Self {
        Self { free, allocated }
    }
}

impl Debug for Heap<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        struct AllocatedHeapEntriesListsFormatter<'a, 'b>(&'b AllocatedHeapEntriesLists<'a>)
        where
            'a: 'b;

        impl<'a, 'b> AllocatedHeapEntriesListsFormatter<'a, 'b>
        where
            'a: 'b,
        {
            fn new(inner: &'b AllocatedHeapEntriesLists<'a>) -> Self {
                Self(inner)
            }
        }

        impl Debug for AllocatedHeapEntriesListsFormatter<'_, '_> {
            fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
                f.debug_map()
                    .entries(self.0.iter().map(
                        |&(alignment, ref entries): &(usize, AllocatedHeapEntries<'_>)| {
                            (alignment, AllocatedHeapEntriesFormatter::new(entries))
                        },
                    ))
                    .finish()
            }
        }

        struct AllocatedHeapEntriesFormatter<'a, 'b>(&'b AllocatedHeapEntries<'a>)
        where
            'a: 'b;

        impl<'a, 'b> AllocatedHeapEntriesFormatter<'a, 'b>
        where
            'a: 'b,
        {
            fn new(inner: &'b AllocatedHeapEntries<'a>) -> Self {
                Self(inner)
            }
        }

        impl Debug for AllocatedHeapEntriesFormatter<'_, '_> {
            fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
                f.debug_list().entries(self.0.iter()).finish()
            }
        }

        f.debug_struct("Heap")
            .field(
                "Total Memory Length",
                &(self
                    .free
                    .iter()
                    .fold(0, |accumulated: usize, range: &MemoryRange| {
                        accumulated + range.len()
                    })
                    + self
                        .allocated
                        .iter()
                        .flat_map(|(_, ranges): &(usize, AllocatedHeapEntries<'_>)| ranges.iter())
                        .fold(0, |accumulated: usize, range: &MemoryRange| {
                            accumulated + range.len()
                        })),
            )
            .field(
                "Allocated",
                &AllocatedHeapEntriesListsFormatter::new(&self.allocated),
            )
            .field("Free", &ListDebugFormatter::new(&self.free.iter()))
            .finish()
    }
}

#[repr(transparent)]
struct FreeHeapEntries<'a>(SortedList<'a, MemoryRange>);

impl<'a> FreeHeapEntries<'a> {
    const PREALLOCATED_ENTRIES_COUNT: usize = 64;

    const REALLOCATION_THRESHOLD: usize = 16;

    const REALLOCATION_COEFFICIENT: usize = 4;

    const REALLOCATION_ADDITIONAL_SIZE: usize =
        FreeHeapEntries::REALLOCATION_THRESHOLD * FreeHeapEntries::REALLOCATION_COEFFICIENT;

    const REQUIRED_INITIAL_SIZE: usize =
        size_of::<[MemoryRange; FreeHeapEntries::PREALLOCATED_ENTRIES_COUNT]>();

    const fn new(inner: SortedList<'a, MemoryRange>) -> Self {
        Self(inner)
    }

    fn needs_reallocation(&self) -> bool {
        self.capacity() - self.len() < Self::REALLOCATION_THRESHOLD
    }
}

impl<'a> Deref for FreeHeapEntries<'a> {
    type Target = SortedList<'a, MemoryRange>;

    fn deref(&self) -> &<Self as Deref>::Target {
        &self.0
    }
}

impl DerefMut for FreeHeapEntries<'_> {
    fn deref_mut(&mut self) -> &mut <Self as Deref>::Target {
        &mut self.0
    }
}

impl Debug for FreeHeapEntries<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        self.0.fmt(f)
    }
}

#[repr(transparent)]
struct AllocatedHeapEntries<'a>(SortedList<'a, MemoryRange>);

impl<'a> AllocatedHeapEntries<'a> {
    const PREALLOCATED_ENTRIES_COUNT: usize = 64;

    const REALLOCATION_THRESHOLD: usize = 16;

    const REALLOCATION_COEFFICIENT: usize = 4;

    const REALLOCATION_ADDITIONAL_SIZE: usize = AllocatedHeapEntries::REALLOCATION_THRESHOLD
        * AllocatedHeapEntries::REALLOCATION_COEFFICIENT;

    const REQUIRED_INITIAL_SIZE: usize =
        size_of::<[MemoryRange; AllocatedHeapEntries::PREALLOCATED_ENTRIES_COUNT]>();

    const fn new(inner: SortedList<'a, MemoryRange>) -> Self {
        Self(inner)
    }

    fn needs_reallocation(&self) -> bool {
        self.capacity() - self.len() < Self::REALLOCATION_THRESHOLD
    }
}

impl<'a> Deref for AllocatedHeapEntries<'a> {
    type Target = SortedList<'a, MemoryRange>;

    fn deref(&self) -> &<Self as Deref>::Target {
        &self.0
    }
}

impl DerefMut for AllocatedHeapEntries<'_> {
    fn deref_mut(&mut self) -> &mut <Self as Deref>::Target {
        &mut self.0
    }
}

impl Debug for AllocatedHeapEntries<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        self.0.fmt(f)
    }
}

#[repr(transparent)]
struct AllocatedHeapEntriesLists<'a>(SortedList<'a, (usize, AllocatedHeapEntries<'a>)>);

impl<'a> AllocatedHeapEntriesLists<'a> {
    const PREALLOCATED_ENTRIES_COUNT: usize = 16;

    const REALLOCATION_THRESHOLD: usize = 4;

    const REALLOCATION_COEFFICIENT: usize = 4;

    const REALLOCATION_ADDITIONAL_SIZE: usize = AllocatedHeapEntriesLists::REALLOCATION_THRESHOLD
        * AllocatedHeapEntriesLists::REALLOCATION_COEFFICIENT;

    const REQUIRED_INITIAL_SIZE: usize = size_of::<
        [(usize, AllocatedHeapEntries); AllocatedHeapEntriesLists::PREALLOCATED_ENTRIES_COUNT],
    >();

    const fn new(inner: SortedList<'a, (usize, AllocatedHeapEntries<'a>)>) -> Self {
        Self(inner)
    }

    fn needs_reallocation(&self) -> bool {
        self.capacity() - self.len() < Self::REALLOCATION_THRESHOLD
    }
}

impl<'a> Deref for AllocatedHeapEntriesLists<'a> {
    type Target = SortedList<'a, (usize, AllocatedHeapEntries<'a>)>;

    fn deref(&self) -> &<Self as Deref>::Target {
        &self.0
    }
}

impl DerefMut for AllocatedHeapEntriesLists<'_> {
    fn deref_mut(&mut self) -> &mut <Self as Deref>::Target {
        &mut self.0
    }
}

impl Debug for AllocatedHeapEntriesLists<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        self.0.fmt(f)
    }
}
