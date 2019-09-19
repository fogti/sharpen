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
    let res = ClassifyIT::new(&mut input.into_iter(), |curo| curo.is_some()).collect::<Vec<_>>();
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
