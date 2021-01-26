pub(in crate::heap) trait FoldElements
where
    Self: Iterator + Sized,
{
    fn fold_elements<F>(self, f: F) -> FoldElementsIter<Self, F>
    where
        Self::Item: Clone,
        F: FnMut(Self::Item, Self::Item) -> Option<Self::Item>,
    {
        FoldElementsIter::new(self, f)
    }
}

impl<T> FoldElements for T where T: Iterator + Sized {}

pub(in crate::heap) struct FoldElementsIter<I, F>
where
    I: Iterator,
    I::Item: Clone,
    F: FnMut(I::Item, I::Item) -> Option<I::Item>,
{
    iter: Option<I>,
    element: Option<I::Item>,
    func: F,
}

impl<I, F> FoldElementsIter<I, F>
where
    I: Iterator,
    I::Item: Clone,
    F: FnMut(I::Item, I::Item) -> Option<I::Item>,
{
    pub fn new(iter: I, func: F) -> Self {
        Self {
            iter: Some(iter),
            element: None,
            func,
        }
    }
}

impl<I, F> Iterator for FoldElementsIter<I, F>
where
    I: Iterator,
    I::Item: Clone,
    F: FnMut(I::Item, I::Item) -> Option<I::Item>,
{
    type Item = I::Item;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            if self.element.is_none() {
                if let Some(data) = self.iter.as_mut() {
                    self.element = data.next();
                }
            }

            if let Some(cumulative) = self.element.take() {
                if let Some(data) = self.iter.as_mut() {
                    if let Some(element) = data.next() {
                        let result: Option<I::Item> =
                            (self.func)(cumulative.clone(), element.clone());

                        let result_is_none: bool = result.is_none();

                        self.element = Some(result.unwrap_or(element));

                        if result_is_none {
                            return Some(cumulative);
                        }
                    } else {
                        return Some(cumulative);
                    }
                }
            } else {
                self.iter = None;

                return None;
            }
        }
    }
}
