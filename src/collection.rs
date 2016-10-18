pub fn uniq<T: PartialEq + Ord + Clone, U: Into<Vec<T>>>(xs: U) -> Vec<T> {
    let mut xs: Vec<T> = xs.into();
    xs.sort();
    xs.dedup();
    xs
}

#[test]
fn test() {
    assert_eq!(uniq(vec![3, 2, 1, 2, 3, 1]), vec![1, 2, 3]);
}
