use alloc::{vec, vec::Vec};

#[cfg(test)]
mod tests;

#[derive(Debug, Eq, PartialEq)]
#[must_use]
pub struct ClassifyIT<'a, TT: 'a, TC, FnT, IT: ?Sized> {
    inner: &'a mut IT,
    fnx: FnT,
    edge: (Option<TC>, Option<TT>),
}

impl<'a, TT: 'a, TC, FnT, IT> ClassifyIT<'a, TT, TC, FnT, IT>
where
    TC: Default,
    FnT: FnMut(&TT) -> TC,
    IT: Iterator<Item = TT>,
{
    #[inline]
    pub fn new(inner: &'a mut IT, fnx: FnT) -> Self {
        Self {
            inner,
            fnx,
            edge: (Some(<_>::default()), None),
        }
    }
}

impl<TT, TC, FnT, IT> Iterator for ClassifyIT<'_, TT, TC, FnT, IT>
where
    TC: PartialEq,
    FnT: FnMut(&TT) -> TC,
    IT: Iterator<Item = TT>,
{
    type Item = (TC, Vec<TT>);

    fn next(&mut self) -> Option<Self::Item> {
        let mut ccl = self.edge.0.take()?;
        let mut last = self.edge.1.take().map_or_else(Vec::new, |x| vec![x]);

        for x in &mut self.inner {
            let old_ccl = core::mem::replace(&mut ccl, (self.fnx)(&x));
            if ccl == old_ccl || last.is_empty() {
                last.push(x);
            } else {
                self.edge = (Some(ccl), Some(x));
                return Some((old_ccl, last));
            }
        }

        // we reached the end of the inner iterator
        if last.is_empty() {
            None
        } else {
            Some((ccl, last))
        }
    }

    /// This iterator probably produces lesser values than the inner iterator
    /// but it is still possible, that every element yields a different ccl,
    /// thus producing the same element count as the inner iterator
    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        self.inner.size_hint()
    }
}

impl<TT, TC, FnT, IT> core::iter::FusedIterator for ClassifyIT<'_, TT, TC, FnT, IT>
where
    TC: PartialEq,
    FnT: FnMut(&TT) -> TC,
    IT: Iterator<Item = TT>,
{
}

pub trait Classify<'a, TT: 'a>: Iterator<Item = TT> + 'a {
    fn classify<TC, FnT>(&'a mut self, fnx: FnT) -> ClassifyIT<'a, TT, TC, FnT, Self>
    where
        TC: Default + PartialEq,
        FnT: FnMut(&TT) -> TC;
}

impl<'a, IT, TT: 'a> Classify<'a, TT> for IT
where
    IT: Iterator<Item = TT> + 'a,
{
    #[inline]
    fn classify<TC, FnT>(&'a mut self, fnx: FnT) -> ClassifyIT<'a, TT, TC, FnT, Self>
    where
        TC: Default + PartialEq,
        FnT: FnMut(&TT) -> TC,
    {
        ClassifyIT::new(self, fnx)
    }
}

#[inline]
pub fn classify<Input, TT, TC, TRes>(input: Input, fnx: impl FnMut(&TT) -> TC) -> TRes
where
    Input: IntoIterator<Item = TT>,
    TC: Default + PartialEq,
    TRes: core::iter::FromIterator<(TC, Vec<TT>)>,
{
    input.into_iter().classify(fnx).collect()
}

#[inline]
pub fn classify_as_vec<Input, TT, TC>(
    input: Input,
    fnx: impl FnMut(&TT) -> TC,
) -> Vec<(TC, Vec<TT>)>
where
    Input: IntoIterator<Item = TT>,
    TC: Default + PartialEq,
{
    classify(input, fnx)
}
