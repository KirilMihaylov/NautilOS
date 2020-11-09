use crate::heap_error::HeapError;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[non_exhaustive]
pub enum InitializationError {
    InternalError,

    NotEnoughInitMemory,

    HeapError(HeapError),
}
