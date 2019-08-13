use alloc::vec::Vec;

pub trait Classification: Copy + Default + PartialEq {}

impl<TC> Classification for TC where TC: Copy + Default + PartialEq {}

#[derive(Debug, Eq, Hash, PartialEq)]
pub struct ClassifyIT<'a, TT: 'a, TC, FnT, IT>
where
    TC: Classification,
    FnT: FnMut(&TT) -> TC,
    IT: ?Sized + Iterator<Item = TT>,
{
    inner: &'a mut IT,
    fnx: FnT,
    edge: (Option<TC>, Option<TT>),
}

impl<'a, TT: 'a, TC, FnT, IT> ClassifyIT<'a, TT, TC, FnT, IT>
where
    TC: Classification,
    FnT: FnMut(&TT) -> TC,
    IT: Iterator<Item = TT>,
{
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
    TC: Classification,
    FnT: FnMut(&TT) -> TC,
    IT: Iterator<Item = TT>,
{
    type Item = (TC, Vec<TT>);

    fn next(&mut self) -> Option<Self::Item> {
        let mut ccl = self.edge.0.take()?;
        let fnx = &mut self.fnx;
        let mut last = if let Some(x) = self.edge.1.take() {
            vec![x]
        } else {
            vec![]
        };

        for x in &mut self.inner {
            let old_ccl = ccl;
            ccl = fnx(&x);
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
    TC: Classification,
    FnT: FnMut(&TT) -> TC,
    IT: Iterator<Item = TT>,
{
}

pub trait Classify<'a, TT: 'a>: Iterator<Item = TT> + 'a {
    fn classify<TC, FnT>(&'a mut self, fnx: FnT) -> ClassifyIT<'a, TT, TC, FnT, Self>
    where
        TC: Classification,
        FnT: FnMut(&TT) -> TC;
}

impl<'a, IT, TT: 'a> Classify<'a, TT> for IT
where
    IT: Iterator<Item = TT> + 'a,
{
    #[inline]
    fn classify<TC, FnT>(&'a mut self, fnx: FnT) -> ClassifyIT<'a, TT, TC, FnT, Self>
    where
        TC: Classification,
        FnT: FnMut(&TT) -> TC,
    {
        ClassifyIT::new(self, fnx)
    }
}

#[inline]
pub fn classify<Input, TT, TC, TRes>(input: Input, fnx: impl FnMut(&TT) -> TC) -> TRes
where
    Input: IntoIterator<Item = TT>,
    TC: Classification,
    TRes: core::iter::FromIterator<(TC, Vec<TT>)>,
{
    input.into_iter().classify(fnx).collect()
}

#[inline]
pub fn classify_as_vec<Input, TT, TC>(input: Input, fnx: impl FnMut(&TT) -> TC) -> Vec<(TC, Vec<TT>)>
where
    Input: IntoIterator<Item = TT>,
    TC: Classification,
{
    classify(input, fnx)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_clsf0() {
        let input: Vec<u8> = vec![0, 0, 1, 1, 2, 2, 3, 0, 5, 5, 5];
        let res: Vec<_> = classify(input, |&curc| curc);
        assert_eq!(
            res,
            &[
                (0, vec![0, 0]),
                (1, vec![1, 1]),
                (2, vec![2, 2]),
                (3, vec![3]),
                (0, vec![0]),
                (5, vec![5, 5, 5]),
            ]
        );
    }

    #[test]
    fn test_clsf1() {
        let input: Vec<Option<u8>> = vec![
            Some(0),
            Some(1),
            Some(5),
            Some(5),
            None,
            None,
            Some(0),
            None,
        ];
        let res: Vec<_> = classify(input, |curo| curo.is_some());
        assert_eq!(
            res,
            &[
                (true, vec![Some(0), Some(1), Some(5), Some(5)]),
                (false, vec![None, None]),
                (true, vec![Some(0)]),
                (false, vec![None]),
            ]
        );
    }

    #[test]
    fn test_clsf2() {
        let input: Vec<Option<Vec<u8>>> = vec![
            Some(vec![0, 0, 1]),
            Some(vec![0, 1]),
            None,
            None,
            Some(vec![2]),
            None,
        ];
        let res: Vec<_> = classify(input, |curo| curo.is_some());
        assert_eq!(
            res,
            &[
                (true, vec![Some(vec![0, 0, 1]), Some(vec![0, 1])]),
                (false, vec![None, None]),
                (true, vec![Some(vec![2])]),
                (false, vec![None]),
            ]
        );
    }

    #[test]
    fn test_clsfit2() {
        let input: Vec<Option<Vec<u8>>> = vec![
            Some(vec![0, 0, 1]),
            Some(vec![0, 1]),
            None,
            None,
            Some(vec![2]),
            None,
        ];
        let res =
            ClassifyIT::new(&mut input.into_iter(), |curo| curo.is_some()).collect::<Vec<_>>();
        assert_eq!(
            res,
            &[
                (true, vec![Some(vec![0, 0, 1]), Some(vec![0, 1])]),
                (false, vec![None, None]),
                (true, vec![Some(vec![2])]),
                (false, vec![None]),
            ]
        );
    }
}
