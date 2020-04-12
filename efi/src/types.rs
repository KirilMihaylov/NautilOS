pub type EfiStatusRaw = usize;
pub type EfiHandle = *const ();
pub type EfiEvent = *const ();
pub type EfiPhysicalAddress = u64;
pub type EfiVirtualAddress = u64;
pub type EfiGuidTuple = (u32, u16, u16, [u8; 8]);
