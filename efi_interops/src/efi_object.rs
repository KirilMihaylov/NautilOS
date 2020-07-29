use core::slice::from_raw_parts;

pub struct EfiObject {
    data: *const u8,
    length: usize,
}

impl EfiObject {
    pub fn new(object: &[u8]) -> Self {
        Self {
            data: object.as_ptr(),
            length: object.len(),
        }
    }

    pub fn is_empty(&self) -> bool {
        self.length == 0
    }

    pub fn len(&self) -> usize {
        self.length
    }

    pub fn copy_over(&self, buffer: &mut [u8]) -> bool {
        if buffer.len() < self.len() {
            false
        } else {
            buffer[..self.len()].copy_from_slice(unsafe { from_raw_parts(self.data, self.len()) });
            true
        }
    }
}
