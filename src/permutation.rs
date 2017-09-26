/// Finds next lexicographical permutation
pub fn permute(xs: &mut [usize]) {
    let i = xs.iter().zip(xs.iter().skip(1)).enumerate()
        .fold(0, |memo, (i, (a, b))| if a < b { i } else { memo });
    let j = xs.iter().enumerate()
        .fold(0, |memo, (j, a)| if a > &xs[i] { j } else { memo });
    xs.swap(i, j);
    xs[i+1..].reverse();
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_permute() {
        let mut xs = [1, 2, 3, 4];
        let expected_list = [
            [1,2,3,4],
            [1,2,4,3],
            [1,3,2,4],
            [1,3,4,2],
            [1,4,2,3],
            [1,4,3,2],
            [2,1,3,4],
            [2,1,4,3],
            [2,3,1,4],
            [2,3,4,1],
            [2,4,1,3],
            [2,4,3,1],
            [3,1,2,4],
            [3,1,4,2],
            [3,2,1,4],
            [3,2,4,1],
            [3,4,1,2],
            [3,4,2,1],
            [4,1,2,3],
            [4,1,3,2],
            [4,2,1,3],
            [4,2,3,1],
            [4,3,1,2],
            [4,3,2,1],
            ];
        for expected in expected_list.iter().skip(1) {
            permute(&mut xs);
            assert_eq!(*expected, xs);
        }
    }
}
