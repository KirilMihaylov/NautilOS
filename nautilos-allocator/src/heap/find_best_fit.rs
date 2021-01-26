use {
    crate::{heap::Heap, memory_range::MemoryRange},
    core::alloc::Layout,
};

impl Heap<'_> {
    pub(super) fn find_best_fit<T>(
        layout: Layout,
        iter: impl Iterator<Item = (T, MemoryRange)>,
    ) -> Option<(T, MemoryRange)> {
        iter
            // Filter out ranges smaller than the required size, align the rest and
            // filter out the newly aligned ranges that are smaller than the required size.
            .filter_map(
                |(data, range): (T, MemoryRange)| -> Option<(T, MemoryRange, usize)> {
                    match range.start().checked_rem(layout.align()) {
                        Some(align_offset)
                            if if let Some(data) = layout.size().checked_add(align_offset) {
                                data
                            } else {
                                return None;
                            } <= range.len() =>
                        {
                            MemoryRange::new(
                                if let Some(data) = range.start().checked_add(align_offset) {
                                    data
                                } else {
                                    return None;
                                },
                                range.end(),
                            )
                            .map(
                                |range: MemoryRange| -> (T, MemoryRange, usize) {
                                    (
                                        data,
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
                },
            )
            // Find the range with smallest memory space loss.
            .min_by_key(|(_, _, loss): &(T, MemoryRange, usize)| -> usize { *loss })
            .and_then(
                |(index, range, _): (T, MemoryRange, usize)| -> Option<(T, MemoryRange)> {
                    range
                        .limit_length(layout.size())
                        .map(|new_range: MemoryRange| (index, new_range))
                },
            )
    }
}
