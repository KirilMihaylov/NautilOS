use {
    crate::{heap::Heap, memory_range::MemoryRange},
    core::alloc::Layout,
};

impl Heap {
    pub(super) fn find_best_fit(
        layout: Layout,
        iter: impl Iterator<Item = MemoryRange>,
    ) -> Option<MemoryRange> {
        iter
            // Filter out ranges smaller than the required size, align the rest and
            // filter out the newly aligned ranges that are smaller than the required size.
            .filter_map(|range: MemoryRange| -> Option<(MemoryRange, usize)> {
                match range.start().checked_rem(layout.align())? {
                    align_offset if layout.size().checked_add(align_offset)? <= range.len() => {
                        MemoryRange::new(range.start().checked_add(align_offset)?, range.end()).map(
                            |range: MemoryRange| -> (MemoryRange, usize) {
                                (
                                    range,
                                    // Calculate memory space loss.
                                    match align_offset {
                                        x if x < 0x10 => x,
                                        x if x < 0x1000 => x / 0x10,
                                        _ => 0x100,
                                    } + match range.len() - layout.size() {
                                        x if x < 0x10 => x,
                                        x if x < 0x1000 => x / 0x10,
                                        _ => 0x100,
                                    },
                                )
                            },
                        )
                    }
                    _ => None,
                }
            })
            // Find the range with smallest memory space loss.
            .min_by_key(|(_, loss): &(MemoryRange, usize)| -> usize { *loss })
            .and_then(|(range, _): (MemoryRange, usize)| -> Option<MemoryRange> {
                range.limit_length(layout.size())
            })
    }
}
