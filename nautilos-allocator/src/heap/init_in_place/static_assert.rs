use {
    crate::{
        heap::{AllocatedHeapEntries, AllocatedHeapEntriesLists, FreeHeapEntries, Heap},
        memory_range::MemoryRange,
    },
    core::mem::{align_of, size_of},
};

macro_rules! static_assert {
    ($x:expr) => {
        const _: [(); 1 - ($x) as usize] = [];
    };
}

macro_rules! static_assert_eq {
    ($x:expr, $y:expr $(,)?) => {
        static_assert!($x == $y);
    };
    ($x:expr, $y:expr, $($z:expr),+ $(,)?) => {
        static_assert_eq!($x, $y);
        static_assert_eq!($y, $($z),+);
    };
}

static_assert_eq!(size_of::<Heap>(), size_of::<[Heap; 1]>(),);

static_assert_eq!(
    0,
    size_of::<Heap>() % align_of::<MemoryRange>(),
    size_of::<[MemoryRange; FreeHeapEntries::PREALLOCATED_ENTRIES_COUNT]>()
        % align_of::<(usize, AllocatedHeapEntries)>(),
    size_of::<[(usize, AllocatedHeapEntries); AllocatedHeapEntriesLists::PREALLOCATED_ENTRIES_COUNT]>(
    ) % align_of::<[MemoryRange; AllocatedHeapEntries::PREALLOCATED_ENTRIES_COUNT]>(),
);

static_assert_eq!(
    align_of::<Heap>(),
    align_of::<[Heap; 0]>(),
    align_of::<MemoryRange>(),
    align_of::<[MemoryRange; 0]>(),
    align_of::<FreeHeapEntries>(),
    align_of::<[FreeHeapEntries; 0]>(),
    align_of::<AllocatedHeapEntries>(),
    align_of::<[AllocatedHeapEntries; 0]>(),
    align_of::<AllocatedHeapEntriesLists>(),
    align_of::<[AllocatedHeapEntriesLists; 0]>(),
);
