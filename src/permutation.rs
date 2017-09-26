/// Mutates into lexicographical permutation
pub fn permute(xs: &mut [usize]) {
    let i = xs.iter().zip(xs.iter().skip(1)).enumerate()
        .fold(0, |memo, (i, (a, b))| if a < b { i } else { memo });
    let j = xs.iter().enumerate()
        .fold(0, |memo, (j, a)| if a > &xs[i] { j } else { memo });
    xs.swap(i, j);
    xs[i+1..].reverse();
}

/// Returns next lexicographical permutation
pub fn next_permutation(xs: &[usize]) -> Vec<usize> {
    let mut xs: Vec<_> = xs.to_vec();
    permute(&mut xs);
    xs
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXPECTED_LIST: [[usize; 4]; 24] = [
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

    #[test]
    fn test_permute() {
        let mut xs = [1, 2, 3, 4];
        for expected in EXPECTED_LIST.iter().skip(1) {
            permute(&mut xs);
            assert_eq!(*expected, xs);
        }
    }

    #[test]
    fn test_next_permutation() {
        for (a, b) in EXPECTED_LIST.iter().zip(EXPECTED_LIST.iter().skip(1)) {
            assert_eq!(next_permutation(a), b.to_vec());
        }
    }
}
