use core::{
    cmp::Ordering,
    fmt::{Debug, Formatter, Result as FmtResult},
    ops::{Deref, DerefMut, Index, IndexMut},
    ptr::{read, write},
};

pub struct List<'a, T> {
    buffer: &'a mut [T],
    length: usize,
}

impl<'a, T> List<'a, T> {
    pub const fn new(buffer: &'a mut [T]) -> Self {
        Self { buffer, length: 0 }
    }

    pub const fn empty() -> Self {
        Self {
            buffer: &mut [],
            length: 0,
        }
    }

    pub const fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub const fn len(&self) -> usize {
        self.length
    }

    pub const fn capacity(&self) -> usize {
        self.buffer.len()
    }

    pub fn buffer(&self) -> &[T] {
        &self.buffer[..self.len()]
    }

    pub fn buffer_mut(&mut self) -> &mut [T] {
        let index: usize = self.len();

        &mut self.buffer[..index]
    }

    pub(crate) fn internal_buffer(&mut self) -> *mut T {
        self.buffer.as_mut_ptr()
    }

    pub(crate) fn set_internal_buffer(&mut self, reference: &'a mut [T]) {
        self.buffer = reference;
    }

    pub fn insert(&mut self, value: T) -> Result<(), T> {
        if self.len() == self.buffer.len() {
            Err(value)
        } else {
            let index: usize = self.len();

            self.length += 1;

            self.buffer_mut()[index] = value;

            Ok(())
        }
    }

    pub fn remove(&mut self, index: usize) {
        if !index < self.length {
            return;
        }

        unsafe {
            read(&self.buffer()[index]);
        }

        for index in index..self.len() {
            unsafe {
                write(&mut self.buffer[index], read(&self.buffer[index + 1]));
            }
        }

        self.length -= 1;
    }

    pub const fn get(&self, index: usize) -> Option<&T> {
        if index < self.length {
            Some(&self.buffer[index])
        } else {
            None
        }
    }

    pub const fn get_mut(&mut self, index: usize) -> Option<&mut T> {
        if index < self.length {
            Some(&mut self.buffer[index])
        } else {
            None
        }
    }
}

impl<T> Deref for List<'_, T> {
    type Target = [T];

    fn deref(&self) -> &<Self as Deref>::Target {
        &self.buffer()[..self.len()]
    }
}

impl<T> DerefMut for List<'_, T> {
    fn deref_mut(&mut self) -> &mut <Self as Deref>::Target {
        let index: usize = self.len();

        &mut self.buffer_mut()[..index]
    }
}

impl<T, U> Index<U> for List<'_, T>
where
    [T]: Index<U>,
{
    type Output = <[T] as Index<U>>::Output;

    fn index(&self, index: U) -> &<Self as Index<U>>::Output {
        &self.buffer[index]
    }
}

impl<T, U> IndexMut<U> for List<'_, T>
where
    [T]: Index<U> + IndexMut<U>,
{
    fn index_mut(&mut self, index: U) -> &mut <Self as Index<U>>::Output {
        &mut self.buffer[index]
    }
}

impl<T> PartialEq for List<'_, T>
where
    T: PartialEq,
{
    fn eq(&self, other: &Self) -> bool {
        self.buffer() == other.buffer()
    }
}

impl<T> PartialOrd for List<'_, T>
where
    T: PartialOrd,
{
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.buffer().partial_cmp(other.buffer())
    }
}

impl<T> Debug for List<'_, T>
where
    T: Debug,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        f.debug_struct("List")
            .field("buffer", &self.buffer())
            .finish()
    }
}
