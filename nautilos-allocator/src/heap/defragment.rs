use crate::{heap::Heap, heap_error::HeapError, memory_range::MemoryRange, sorted_list::Element};

impl Heap<'_> {
    pub(in crate::heap) fn defragment(&mut self) -> Result<(), HeapError> {
        let mut index: usize = 1;

        while index < self.free.len() {
            let remove_entry: bool;

            {
                let range: MemoryRange = if let Some(&range) = self.free.get(index) {
                    range
                } else {
                    return Err(HeapError::InternalError);
                };

                let mut left_entry: Element<'_, '_, MemoryRange> =
                    if let Some(range) = self.free.get_mut(index - 1) {
                        range
                    } else {
                        return Err(HeapError::InternalError);
                    };
                let left_entry: &mut MemoryRange = &mut *left_entry;

                if let Some(new_range) = left_entry.add_loose(range) {
                    *left_entry = new_range;

                    remove_entry = true;
                } else {
                    remove_entry = false;
                }
            }

            if remove_entry {
                self.free.remove(index);

                continue;
            }

            // Safety: Cannot overflow because implicitly checked for being less than `usize::MAX` in the `while` condition.
            index += 1;
        }

        Ok(())
    }
}
