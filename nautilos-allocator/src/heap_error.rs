#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[non_exhaustive]
pub enum HeapError {
    InternalError,

    OutOfMemory,
    NoSuchMemoryRange,
    DifferentAlignment,
}
