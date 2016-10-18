//! Taken from https://en.wikipedia.org/wiki/Knapsack_problem#0.2F1_knapsack_problem

use super::list;


/// Input:
/// Values (stored in vector v)
/// Weights (stored in vector w)
/// Knapsack capacity (max_w)
/// Returns the (maximum value, chosen items)
pub fn knapsack(
    v: &[usize],
    w: &[usize],
    max_w: usize
) -> (usize, Vec<usize>) {
    assert!(v.len() == w.len());
    // The number of distinct items (n) is the length of the Values vector.
    let n = v.len();
    let mut selections = vec![list::nil(); max_w + 1];
    let mut m: Vec<usize> = vec![0; max_w + 1];
    for i in 0..n {
        for j in (w[i]..max_w+1).into_iter().rev() {
            let new_value = m[j-w[i]] + v[i];
            if new_value > m[j] {
                m[j] = new_value;
                selections[j] = list::cons(i, selections[j - w[i]].clone());
            }
        }
    }
    let selections: Vec<usize> = list::iter(selections[max_w].clone())
        .map(|x| *x)
        .collect();
    (m[max_w], selections)
}


#[cfg(test)]
mod tests {
    use super::knapsack;

    #[test]
    fn test_knapsack() {
        assert_eq!(
            knapsack(
                &[2, 5, 6],
                &[5, 5, 5],
                10),
                (11, vec![2, 1]));

        assert_eq!(
            knapsack(
                &[2, 5, 6],
                &[5, 5, 5],
                15),
            (13, vec![2, 1, 0]));

        assert_eq!(
            knapsack(
                &[2, 5, 6],
                &[5, 5, 5],
                9),
                (6, vec![2]));

        assert_eq!(
            knapsack(
                &[3, 5, 6, 9],
                &[1, 5, 5, 9],
                10),
                (12, vec![3, 0]));
    }
}
