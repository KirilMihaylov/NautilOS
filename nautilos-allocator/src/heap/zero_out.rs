use {crate::heap::Heap, core::ptr::write_bytes};

impl Heap<'_> {
    pub(super) unsafe fn zero_out(address: usize, length: usize) {
        write_bytes(address as *mut u8, 0, length);
    }
}
