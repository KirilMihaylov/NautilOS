#[repr(C)]
pub enum OsMemoryType {
    LoaderHeap,
    HandlesBuffer,
}

impl From<OsMemoryType> for [u8; core::mem::size_of::<OsMemoryType>()] {
    fn from(data: OsMemoryType) -> Self {
        unsafe { core::mem::transmute(data) }
    }
}
