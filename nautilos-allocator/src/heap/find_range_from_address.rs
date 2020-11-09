use crate::heap::{Heap, HeapEntry};

impl Heap {
    pub(super) fn find_range_from_address(&self, address: usize) -> Option<(usize, HeapEntry)> {
        self.entries
            .binary_search_by_key(&address, |(_, range): &HeapEntry| range.start())
            .ok()
            .and_then(|index: usize| {
                self.entries
                    .get(index)
                    .map(|&entry: &HeapEntry| (index, entry))
            })
    }
}
