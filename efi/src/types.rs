use core::ptr::NonNull;

pub type EfiStatusRaw = usize;
pub type EfiHandle = *const ();
pub type EfiEvent = *const ();
pub type EfiPhysicalAddress = u64;
pub type EfiVirtualAddress = u64;
pub type EfiLBA = u64;
pub type EfiGuidTuple = (u32, u16, u16, [u8; 8]);
pub type Void = core::ffi::c_void;
pub type VoidPtr = *const Void;
pub type VoidMutPtr = *mut Void;
pub type VoidPtrPtr = *const VoidPtr;
pub type VoidMutPtrPtr = *mut VoidPtr;
pub type NonNullVoidPtr = NonNull<Void>;

#[derive(Debug)]
pub struct EfiFirmwareFault;
