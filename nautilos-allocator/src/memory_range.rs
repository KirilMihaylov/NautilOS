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
        Some(Self {
            start: if let Some(start) = if start <= end {
                NonZeroUsize::new(start)
            } else {
                None
            } {
                start
            } else {
                return None;
            },
            end: if let Some(end) = if start <= end {
                NonZeroUsize::new(end)
            } else {
                None
            } {
                end
            } else {
                return None;
            },
        })
    }

    #[must_use]
    pub const fn new_with_length(start: usize, length: usize) -> Option<Self> {
        Self::new(
            start,
            if let Some(end) = start.checked_add(if let Some(offset) = length.checked_sub(1) {
                offset
            } else {
                return None;
            }) {
                end
            } else {
                return None;
            },
        )
    }

    #[must_use]
    pub const fn len(&self) -> usize {
        // Safety:
        // Will not overflow because "end" is bigger than "start" by definition and the minimal value of "start" is 1.
        // Border case:
        //     max(end) = max(usize)
        //     min(start) = 1
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
    pub const fn ajasoned_left(&self, other: MemoryRange) -> bool {
        other.end() == self.start() - 1
    }

    #[must_use]
    pub const fn ajasoned_right(&self, other: MemoryRange) -> bool {
        self.end() == other.start() - 1
    }

    #[must_use]
    pub const fn ajasoned(&self, other: MemoryRange) -> bool {
        self.ajasoned_left(other) || self.ajasoned_right(other)
    }

    #[must_use]
    pub const fn overlapped_or_ajasoned(&self, other: MemoryRange) -> bool {
        self.is_overlapped(other) || self.ajasoned(other)
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
        if self.overlapped_or_ajasoned(other) {
            Self::new(
                if self.start() < other.start() {
                    self.start()
                } else {
                    other.start()
                },
                if self.end() < other.end() {
                    other.end()
                } else {
                    self.end()
                },
            )
        } else {
            None
        }
    }

    #[must_use]
    pub fn subtract(&self, other: MemoryRange) -> (Option<MemoryRange>, Option<MemoryRange>) {
        self.overlapped(other).map_or_else(
            || (Some(*self), None),
            |overlap| {
                (
                    if self.start() < overlap.start() {
                        MemoryRange::new(self.start(), overlap.start() - 1)
                    } else {
                        None
                    },
                    if overlap.end() < self.end() {
                        MemoryRange::new(overlap.end() + 1, self.end())
                    } else {
                        None
                    },
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
