use crate::{
    heap::{AllocatedHeapEntries, Heap},
    memory_range::MemoryRange,
    sorted_list::Element,
};

pub(in crate::heap) struct RangeFromAddress {
    store_index: usize,
    entry_index: usize,
    range: MemoryRange,
}

impl RangeFromAddress {
    const fn new(store_index: usize, entry_index: usize, range: MemoryRange) -> Self {
        Self {
            store_index,
            entry_index,
            range,
        }
    }

    pub const fn unzip(self) -> (usize, usize, MemoryRange) {
        (self.store_index, self.entry_index, self.range)
    }
}

impl Heap<'_> {
    pub(super) fn find_range_from_address(
        &mut self,
        range_alignment: usize,
        address: usize,
    ) -> Option<RangeFromAddress> {
        let store_index: usize = if let Ok(data) = self.allocated.binary_search_by_key(
            &range_alignment,
            |(alignment, _): &(usize, AllocatedHeapEntries<'_>)| *alignment,
        ) {
            data
        } else {
            return None;
        };

        let mut store: Element<'_, '_, (usize, AllocatedHeapEntries<'_>)> =
            if let Some(data) = self.allocated.get_mut(store_index) {
                data
            } else {
                return None;
            };
        let store: &mut AllocatedHeapEntries<'_> = &mut store.1;

        let entry_index: usize =
            if let Ok(data) = store.binary_search_by_key(&address, MemoryRange::start) {
                data
            } else {
                return None;
            };

        Some(RangeFromAddress::new(
            store_index,
            entry_index,
            store[entry_index],
        ))
    }
}
