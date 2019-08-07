use alloc::vec::Vec;

#[derive(Eq, PartialEq)]
pub struct ClassifyIT<'a, TT: 'a, TC, FnT, IT>
where
    TC: Copy + Default + PartialEq,
    FnT: FnMut(&TT) -> TC,
    IT: Iterator<Item = TT>,
{
    inner: &'a mut IT,
    fnx: FnT,
    edge: (Option<TC>, Option<TT>),
}

impl<'a, TT: 'a, TC, FnT, IT> ClassifyIT<'a, TT, TC, FnT, IT>
where
    TC: Copy + Default + PartialEq,
    FnT: FnMut(&TT) -> TC,
    IT: Iterator<Item = TT>,
{
    pub fn new(inner: &'a mut IT, fnx: FnT) -> Self {
        Self {
            inner,
            fnx,
            edge: (Some(Default::default()), None),
        }
    }
}

impl<TT, TC, FnT, IT> Iterator for ClassifyIT<'_, TT, TC, FnT, IT>
where
    TC: Copy + Default + PartialEq,
    FnT: FnMut(&TT) -> TC,
    IT: Iterator<Item = TT>,
{
    type Item = (TC, Vec<TT>);

    fn next(&mut self) -> Option<Self::Item> {
        let mut ccl = self.edge.0?;
        let mut last = Vec::<TT>::new();

        if let Some(x) = self.edge.1.take() {
            last.push(x);
        }
        let fnx = &mut self.fnx;
        for x in &mut self.inner {
            let new_ccl = fnx(&x);
            if new_ccl != ccl {
                if last.is_empty() {
                    ccl = new_ccl;
                    last.push(x);
                } else {
                    self.edge = (Some(new_ccl), Some(x));
                    return Some((ccl, last));
                }
            } else {
                last.push(x);
            }
        }

        // we reached the end of the inner iterator
        self.edge = (None, None);
        if last.is_empty() {
            None
        } else {
            Some((ccl, last))
        }
    }
}

impl<TT, TC, FnT, IT> core::iter::FusedIterator for ClassifyIT<'_, TT, TC, FnT, IT>
where
    TC: Copy + Default + PartialEq,
    FnT: FnMut(&TT) -> TC,
    IT: Iterator<Item = TT>,
{
}

pub trait Classify<'a, TT: 'a>
where
    Self: Sized + Iterator<Item = TT> + 'a,
{
    fn classify<TC, FnT>(&'a mut self, fnx: FnT) -> ClassifyIT<'a, TT, TC, FnT, Self>
    where
        TC: Copy + Default + PartialEq,
        FnT: FnMut(&TT) -> TC;
}

impl<'a, IT, TT: 'a> Classify<'a, TT> for IT
where
    Self: Sized + Iterator<Item = TT> + 'a,
{
    fn classify<TC, FnT>(&'a mut self, fnx: FnT) -> ClassifyIT<'a, TT, TC, FnT, Self>
    where
        TC: Copy + Default + PartialEq,
        FnT: FnMut(&TT) -> TC,
    {
        ClassifyIT::new(self, fnx)
    }
}

pub fn classify<Input, FnT, TT, TC, TRes>(input: Input, fnx: FnT) -> TRes
where
    Input: IntoIterator<Item = TT>,
    FnT: FnMut(&TT) -> TC,
    TC: Copy + Default + PartialEq,
    TRes: core::iter::FromIterator<(TC, Vec<TT>)>,
{
    input.into_iter().classify(fnx).collect()
}

pub fn classify_as_vec<Input, FnT, TT, TC>(input: Input, fnx: FnT) -> Vec<(TC, Vec<TT>)>
where
    Input: IntoIterator<Item = TT>,
    FnT: FnMut(&TT) -> TC,
    TC: Copy + Default + PartialEq,
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
            vec![
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
            vec![
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
            vec![
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
            vec![
                (true, vec![Some(vec![0, 0, 1]), Some(vec![0, 1])]),
                (false, vec![None, None]),
                (true, vec![Some(vec![2])]),
                (false, vec![None]),
            ]
        );
    }
}
