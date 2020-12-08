//! This is a utility crate for encoding Rust's native (UTF-8) strings into UTF-16 encoded arrays.

#![no_std]
#![forbid(warnings, missing_docs, unsafe_code, clippy::pedantic)]

use core::fmt::{Arguments, Debug, Display, Error, Result, Write};

pub use macros;

enum EncoderIterator {
    Exhausted,
    TwoByte(u16),
    FourByte(u16, u16),
}

impl EncoderIterator {
    #[must_use]
    pub fn new(ch: char) -> Self {
        match ch.encode_utf16(&mut [0; 2]) {
            [unit] => Self::TwoByte(*unit),
            [unit1, unit2] => Self::FourByte(*unit1, *unit2),
            _ => unreachable!(),
        }
    }
}

impl Iterator for EncoderIterator {
    type Item = u16;

    fn next(&mut self) -> Option<<Self as Iterator>::Item> {
        match *self {
            Self::TwoByte(unit) => {
                *self = Self::Exhausted;

                Some(unit)
            }
            Self::FourByte(unit1, unit2) => {
                *self = Self::TwoByte(unit2);

                Some(unit1)
            }
            Self::Exhausted => None,
        }
    }
}

/// Used to encode UTF-16 strings into slices.
pub struct ArrayEncoder<'a> {
    buffer: &'a mut [u16],
    index: usize,
}

impl ArrayEncoder<'_> {
    #[must_use]
    /// Creates new instance that uses the passed slice as a buffer.
    pub fn new(buffer: &mut [u16]) -> ArrayEncoder<'_> {
        ArrayEncoder { buffer, index: 0 }
    }

    /// "Prints" a formatted string created by the [`format_args`] macro to the buffer.
    /// # Errors
    /// Forwards the errors, if any, originating from [`write_fmt`](core::fmt::Write::write_fmt).
    pub fn write_formatted(&mut self, args: Arguments) -> Result {
        self.write_fmt(args)
    }

    /// Formats the argument through the [`Display`] trait and "prints" the string to the buffer.
    /// # Errors
    /// Forwards the errors, if any, originating from [`write_fmt`](core::fmt::Write::write_fmt).
    pub fn write<T: Display>(&mut self, arg: T) -> Result {
        self.write_fmt(format_args!("{}", arg))
    }

    /// Formats the argument through the [`Debug`] trait and "prints" the string to the buffer.
    /// # Errors
    /// Forwards the errors, if any, originating from [`write_fmt`](core::fmt::Write::write_fmt).
    pub fn write_debug<T: Debug>(&mut self, arg: T) -> Result {
        self.write_fmt(format_args!("{:?}", arg))
    }

    /// Returns immutable reference to the buffer with which the instance was created.
    #[must_use]
    pub fn get_buffer(&self) -> &[u16] {
        &self.buffer[..self.index]
    }

    /// Returns mutable reference to the buffer with which the instance was created.
    #[must_use]
    pub fn get_buffer_mut(&mut self) -> &mut [u16] {
        &mut self.buffer[..self.index]
    }

    /// Resets the position in the buffer and therefore the next write will overwrite the current contents of the buffer.
    pub fn reset_buffer(&mut self) {
        self.index = 0;
    }
}

impl Write for ArrayEncoder<'_> {
    fn write_str(&mut self, s: &str) -> Result {
        let utf16_length: usize = s.chars().fold(0_usize, |byte_count: usize, ch: char| {
            byte_count + ch.len_utf16()
        });

        let buffer: &mut [u16] = &mut self.buffer[self.index..];

        if buffer.len() < utf16_length {
            Err(Error)
        } else {
            for (element, unit) in buffer
                .iter_mut()
                .zip(s.chars().flat_map(EncoderIterator::new))
            {
                *element = unit;
            }

            self.index += utf16_length;

            Ok(())
        }
    }
}
