use core::{
    cmp::Ordering,
    fmt::{Debug, Formatter, Result as FmtResult},
    ops::{Deref, DerefMut, Index, IndexMut, RangeBounds},
    ptr::{read_unaligned, write_unaligned},
    slice::Iter,
};

use crate::list::List;

#[repr(transparent)]
struct Wrapper<T>(T);

impl<T> Wrapper<T> {
    #[must_use]
    pub const fn new(inner: T) -> Self {
        Self(inner)
    }
}

impl<T> From<T> for Wrapper<T> {
    fn from(inner: T) -> Self {
        Self(inner)
    }
}

impl<T> Deref for Wrapper<T> {
    type Target = T;

    fn deref(&self) -> &<Self as Deref>::Target {
        &self.0
    }
}

impl<T> DerefMut for Wrapper<T> {
    fn deref_mut(&mut self) -> &mut <Self as Deref>::Target {
        &mut self.0
    }
}

impl<T> Debug for Wrapper<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "{}", core::any::type_name::<T>())
    }
}

type SortFn<T> = fn(&T, &T) -> Ordering;

#[derive(Debug)]
pub struct SortedList<'a, T> {
    buffer: List<'a, T>,
    sort_fn: Wrapper<SortFn<T>>,
}

impl<'a, T> SortedList<'a, T> {
    pub const fn new(buffer: &'a mut [T], sort_fn: SortFn<T>) -> Self {
        Self {
            buffer: List::new(buffer),
            sort_fn: Wrapper::new(sort_fn),
        }
    }

    pub const fn len(&self) -> usize {
        self.buffer.len()
    }

    pub const fn capacity(&self) -> usize {
        self.buffer.capacity()
    }

    pub(crate) fn internal_buffer(&mut self) -> *mut T {
        self.buffer.internal_buffer()
    }

    pub(crate) fn set_internal_buffer(&mut self, reference: &'a mut [T]) {
        self.buffer.set_internal_buffer(reference)
    }

    pub fn buffer(&self) -> Range<T> {
        Range::new(self.buffer.buffer())
    }

    fn sort_list(&mut self) {
        for index in 0..self.len().saturating_sub(1) {
            let mut min_value_index: usize = index;

            for index in (index + 1)..self.len() {
                if (*self.sort_fn)(
                    &self.buffer.buffer()[index],
                    &self.buffer.buffer()[min_value_index],
                ) == Ordering::Less
                {
                    min_value_index = index;
                }
            }

            unsafe {
                let value: T = read_unaligned(&self.buffer.buffer()[min_value_index]);
                write_unaligned(
                    &mut self.buffer.buffer_mut()[min_value_index],
                    read_unaligned(&self.buffer.buffer()[index]),
                );
                write_unaligned(&mut self.buffer.buffer_mut()[index], value);
            }
        }
    }

    pub fn insert(&mut self, value: T) -> Result<(), T> {
        if let error @ Err(_) = self.buffer.insert(value) {
            return error;
        }

        self.sort_list();

        Ok(())
    }

    pub fn remove(&mut self, index: usize) {
        self.buffer.remove(index);

        self.sort_list();
    }

    pub fn remove_range<R>(&mut self, range: R)
    where
        R: RangeBounds<usize>,
    {
        self.buffer.remove_range(range);

        self.sort_list();
    }

    pub const fn get(&self, index: usize) -> Option<&T> {
        self.buffer.get(index)
    }

    pub const fn get_mut(&mut self, index: usize) -> Option<Element<'a, '_, T>> {
        Element::new(self, index)
    }

    pub fn iter(&'a self) -> Iter<'a, T> {
        self.buffer.buffer().iter()
    }

    pub fn binary_search_by<F>(&'a self, f: F) -> Result<usize, usize>
    where
        F: FnMut(&'a T) -> Ordering,
    {
        self.buffer.binary_search_by(f)
    }

    pub fn binary_search_by_key<B, F>(&'a self, b: &B, f: F) -> Result<usize, usize>
    where
        F: FnMut(&'a T) -> B,
        B: Ord,
    {
        self.buffer.binary_search_by_key(b, f)
    }
}

impl<'a, T, U> Index<U> for SortedList<'a, T>
where
    List<'a, T>: Index<U>,
{
    type Output = <List<'a, T> as Index<U>>::Output;

    fn index(&self, index: U) -> &<Self as Index<U>>::Output {
        &self.buffer[index]
    }
}

impl<T> PartialEq for SortedList<'_, T>
where
    T: PartialEq,
{
    fn eq(&self, other: &Self) -> bool {
        self.buffer() == other.buffer()
    }
}

impl<T> PartialOrd for SortedList<'_, T>
where
    T: PartialOrd,
{
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.buffer().partial_cmp(&other.buffer())
    }
}

pub struct Element<'a, 'b, T>
where
    'a: 'b,
{
    list: &'b mut SortedList<'a, T>,
    element: *mut T,
}

impl<'a, 'b, T> Element<'a, 'b, T> {
    #[must_use]
    pub const fn new(list: &'b mut SortedList<'a, T>, index: usize) -> Option<Self> {
        Some({
            let element: *mut T = if let Some(element) = list.buffer.get_mut(index) {
                element
            } else {
                return None;
            };

            Self { list, element }
        })
    }
}

impl<T> Drop for Element<'_, '_, T> {
    fn drop(&mut self) {
        self.list.sort_list()
    }
}

impl<T> Deref for Element<'_, '_, T> {
    type Target = T;

    fn deref(&self) -> &<Self as Deref>::Target {
        unsafe {
            // Safety: List is borrowed mutably and the index of the element is checked whether is in bounds.
            &*self.element
        }
    }
}

impl<T> DerefMut for Element<'_, '_, T> {
    fn deref_mut(&mut self) -> &mut <Self as Deref>::Target {
        unsafe {
            // Safety: List is borrowed mutably and the index of the element is checked whether is in bounds.
            &mut *self.element
        }
    }
}

#[repr(transparent)]
pub struct Range<'a, T> {
    elements: &'a [T],
}

impl<'a, T> Range<'a, T> {
    #[must_use]
    pub const fn new(elements: &'a [T]) -> Self {
        Self { elements }
    }
}

impl<T> PartialEq for Range<'_, T>
where
    T: PartialEq,
{
    fn eq(&self, other: &Self) -> bool {
        self.elements == other.elements
    }
}

impl<T> PartialOrd for Range<'_, T>
where
    T: PartialOrd,
{
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.elements.partial_cmp(other.elements)
    }
}

impl<T, U> Index<U> for Range<'_, T>
where
    [T]: Index<U>,
{
    type Output = <[T] as Index<U>>::Output;

    fn index(&self, index: U) -> &<Self as Index<U>>::Output {
        &self.elements[index]
    }
}

pub struct RangeMut<'a, 'b, T> {
    list: &'b mut SortedList<'a, T>,
    elements: *mut [T],
}

impl<T> Deref for RangeMut<'_, '_, T> {
    type Target = [T];

    fn deref(&self) -> &<Self as Deref>::Target {
        unsafe { &*self.elements }
    }
}

impl<T> DerefMut for RangeMut<'_, '_, T> {
    fn deref_mut(&mut self) -> &mut <Self as Deref>::Target {
        unsafe { &mut *self.elements }
    }
}

impl<T> Drop for RangeMut<'_, '_, T> {
    fn drop(&mut self) {
        self.list.sort_list()
    }
}

impl<T> PartialEq for RangeMut<'_, '_, T>
where
    T: PartialEq,
{
    fn eq(&self, other: &Self) -> bool {
        unsafe { *self.elements == *other.elements }
    }
}

impl<T> PartialOrd for RangeMut<'_, '_, T>
where
    T: PartialOrd,
{
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        unsafe { (&*self.elements).partial_cmp(&*other.elements) }
    }
}

impl<T, U> Index<U> for RangeMut<'_, '_, T>
where
    [T]: Index<U>,
{
    type Output = <[T] as Index<U>>::Output;

    fn index(&self, index: U) -> &<Self as Index<U>>::Output {
        &(unsafe { &*self.elements })[index]
    }
}

impl<T, U> IndexMut<U> for RangeMut<'_, '_, T>
where
    [T]: Index<U> + IndexMut<U>,
{
    fn index_mut(&mut self, index: U) -> &mut <Self as Index<U>>::Output {
        &mut (unsafe { &mut *self.elements })[index]
    }
}
