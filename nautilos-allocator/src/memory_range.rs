#![allow(dead_code)]

use core::{
    fmt::{Debug, Formatter, Result as FmtResult},
    num::NonZeroUsize,
};

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub(crate) struct MemoryRange {
    start: NonZeroUsize,
    end: NonZeroUsize,
}

#[must_use]
impl MemoryRange {
    #[must_use]
    pub const fn new(start: usize, end: usize) -> Option<Self> {
        if start > end {
            return None;
        }

        let (non_zero_start, non_zero_end): (NonZeroUsize, NonZeroUsize);

        match NonZeroUsize::new(start) {
            Some(start) => non_zero_start = start,
            None => return None,
        }

        match NonZeroUsize::new(end) {
            Some(end) => non_zero_end = end,
            None => return None,
        }

        Some(Self {
            start: non_zero_start,
            end: non_zero_end,
        })
    }

    #[must_use]
    pub const fn new_with_length(start: usize, length: usize) -> Option<Self> {
        let length: usize;
        
        match length.checked_sub(1) {
            Some(calculated_length) => length = calculated_length,
            None => return None,
        }

        match start.checked_add(length) {
            Some(end) => Self::new(start, end),
            None => return None,
        }
    }

    #[must_use]
    pub const fn len(&self) -> usize {
        // Safety:
        // Will not overflow because "end" is strongly bigger than "start" by definition and the minimal value of "start" is 1.
        // Border case:
        //     min(start) = 1
        //     max(end) = max(usize)
        //     ===> max(usize) - min(start) + 1 = max(usize) - 1 + 1 = max(usize)
        self.end.get() - self.start.get() + 1
    }

    #[must_use]
    pub const fn start(&self) -> usize {
        self.start.get()
    }

    #[must_use]
    pub const fn end(&self) -> usize {
        self.end.get()
    }

    #[must_use]
    pub const fn contains(&self, address: usize) -> bool {
        self.start.get() <= address && address <= self.end.get()
    }

    #[must_use]
    pub const fn is_overlapped(&self, other: MemoryRange) -> bool {
        self.contains(other.start()) || self.contains(other.end())
    }

    #[must_use]
    pub const fn overlapped(&self, other: MemoryRange) -> Option<MemoryRange> {
        match (self.contains(other.start()), self.contains(other.end())) {
            (true, true) => Some(other),
            (true, false) => MemoryRange::new(other.start(), self.end()),
            (false, true) => MemoryRange::new(self.start(), other.end()),
            _ if other.contains(self.start()) && other.contains(self.end()) => Some(*self),
            _ => None,
        }
    }

    #[must_use]
    pub const fn is_ajasoned_on_left(&self, other: MemoryRange) -> bool {
        other.end() == self.start() - 1
    }

    #[must_use]
    pub const fn is_ajasoned_on_right(&self, other: MemoryRange) -> bool {
        self.end() == other.start() - 1
    }

    #[must_use]
    pub const fn is_ajasoned(&self, other: MemoryRange) -> bool {
        self.is_ajasoned_on_left(other) || self.is_ajasoned_on_right(other)
    }

    #[must_use]
    pub const fn is_overlapped_or_ajasoned(&self, other: MemoryRange) -> bool {
        self.is_overlapped(other) || self.is_ajasoned(other)
    }

    #[must_use]
    pub const fn add(&self, other: MemoryRange) -> Option<MemoryRange> {
        if self.is_overlapped(other) {
            Self::new(
                if self.start() < other.start() {
                    self.start()
                } else {
                    other.start()
                },
                if other.end() < self.end() {
                    self.end()
                } else {
                    other.end()
                },
            )
        } else {
            None
        }
    }

    #[must_use]
    pub const fn add_loose(&self, other: MemoryRange) -> Option<MemoryRange> {
        if self.is_overlapped_or_ajasoned(other) {
            let (start, end): (usize, usize);

            start = if self.start() < other.start() {
                self.start()
            } else {
                other.start()
            };

            end = if self.end() < other.end() {
                    other.end()
                } else {
                    self.end()
                };

            Self::new(
                start,
                end,
            )
        } else {
            None
        }
    }

    #[must_use]
    pub fn subtract(&self, other: MemoryRange) -> (Option<MemoryRange>, Option<MemoryRange>) {
        self.overlapped(other).map_or_else(
            || (Some(*self), None),
            |overlap: MemoryRange| {
                (
                    MemoryRange::new(self.start(), overlap.start() - 1),
                    MemoryRange::new(overlap.end() + 1, self.end()),
                )
            },
        )
    }

    #[must_use]
    pub const fn limit_length(&self, length: usize) -> Option<Self> {
        Self::new_with_length(
            self.start(),
            if length < self.len() {
                length
            } else {
                self.len()
            },
        )
    }
}

impl Debug for MemoryRange {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        f.debug_struct("MemoryRange")
            .field("Start", &(self.start.get() as *const ()))
            .field("End", &(self.end.get() as *const ()))
            .field("Length", &self.len())
            .finish()
    }
}
