use core::fmt::{Debug, Formatter, Result as FmtResult};

pub(super) struct ListDebugFormatter<'a, T, U>
where
    T: Iterator<Item = U> + Clone,
    U: Debug,
{
    iter: &'a T,
}

impl<'a, T, U> ListDebugFormatter<'a, T, U>
where
    T: Iterator<Item = U> + Clone,
    U: Debug,
{
    #[must_use]
    pub fn new(iter: &'a T) -> Self {
        Self { iter }
    }
}

impl<T, U> Debug for ListDebugFormatter<'_, T, U>
where
    T: Iterator<Item = U> + Clone,
    U: Debug,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        f.debug_list().entries(self.iter.clone()).finish()
    }
}
