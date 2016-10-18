use std::iter::IntoIterator;


#[derive(Debug)]
pub struct Matrix<T> {
  pub dimensions: usize,
  m: Vec<T>
}


impl<T> Matrix<T> {
  pub fn new(dimensions: usize) -> Matrix<T> {
    Matrix {
      dimensions: dimensions,
      m: Vec::with_capacity(dimensions * dimensions)
    }
  }

  pub fn with_data(dimensions: usize, data: Vec<T>) -> Matrix<T> {
    assert!(data.len() == dimensions * dimensions);
    Matrix {
      dimensions: dimensions,
      m: data
    }
  }

  pub fn with_fn<F>(dimensions: usize, f: F) -> Matrix<T>
    where F: Fn(usize, usize) -> T {
    let mut matrix = Self::new(dimensions);
    for row in 0..dimensions {
      for col in 0..dimensions {
        let val = f(row, col);
        matrix.m.push(val);
      }
    }
    matrix
  }

  pub fn transpose(&mut self) {
    for row in 0..self.dimensions {
      for col in row..self.dimensions {
        let a = row * self.dimensions + col;
        let b = col * self.dimensions + row;
        self.m.swap(a, b);
      }
    }
  }

  pub fn get_row(&self, row: usize) -> &[T] {
    let range = self.row_range(row);
    &self.m[range]
  }

  pub fn get_row_mut(&mut self, row: usize) -> &mut [T] {
    let range = self.row_range(row);
    &mut self.m[range]
  }

  pub fn rotate_cw(&mut self) {
    self.transpose();
    self.reverse_rows();
  }

  pub fn rotate_ccw(&mut self) {
    self.reverse_rows();
    self.transpose();
  }

  pub fn get(&self, row: usize, col: usize) -> &T {
    &self.m[self.dimensions * row + col]
  }

  pub fn set(&mut self, row: usize, col: usize, val: T) {
    self.m[self.dimensions * row + col] = val;
  }

  fn row_range(&self, row: usize) -> ::std::ops::Range<usize> {
    let offset = self.dimensions * row;
    offset..offset + self.dimensions
  }

  fn reverse_rows(&mut self) {
    for row_ix in 0..self.dimensions {
      let row = self.get_row_mut(row_ix);
      row.reverse();
    }
  }
}


pub struct IntoIter<'a, T> where T: 'a {
  matrix: &'a Matrix<T>,
  ix: usize
}

impl<'a, T> Iterator for IntoIter<'a, T> {
  type Item = &'a [T];
  fn next(&mut self) -> Option<&'a [T]> {
    let current_ix = self.ix;
    let is_done = current_ix >= self.matrix.dimensions;
    self.ix += 1;
    if is_done {
      None
    } else {
      Some(self.matrix.get_row(current_ix))
    }
  }
}

impl<'a, T> IntoIterator for &'a Matrix<T> {
  type Item = &'a [T];
  type IntoIter = IntoIter<'a, T>;

  fn into_iter(self) -> Self::IntoIter {
    IntoIter {
      matrix: self,
      ix: 0
    }
  }
}


#[cfg(test)]
mod tests {
  use super::Matrix;

  #[test]
  fn test_rotate_cw() {
    let mut m = Matrix::with_fn(3, |row, col| (row, col));
    m.rotate_cw();
    assert_eq!(m.get_row(0), &[(2, 0), (1, 0), (0, 0)]);
    assert_eq!(m.get_row(1), &[(2, 1), (1, 1), (0, 1)]);
    assert_eq!(m.get_row(2), &[(2, 2), (1, 2), (0, 2)]);
  }

  #[test]
  fn test_rotate_ccw() {
    let mut m = Matrix::with_fn(3, |row, col| (row, col));
    m.rotate_ccw();
    assert_eq!(m.get_row(0), &[(0, 2), (1, 2), (2, 2)]);
    assert_eq!(m.get_row(1), &[(0, 1), (1, 1), (2, 1)]);
    assert_eq!(m.get_row(2), &[(0, 0), (1, 0), (2, 0)]);
  }

  #[test]
  fn test_transpose() {
    let mut m = Matrix::with_fn(3, |row, col| (row, col));
    m.transpose();
    assert_eq!(m.get_row(0), &[(0, 0), (1, 0), (2, 0)]);
    assert_eq!(m.get_row(1), &[(0, 1), (1, 1), (2, 1)]);
    assert_eq!(m.get_row(2), &[(0, 2), (1, 2), (2, 2)]);
  }

  #[test]
  fn test_set_row() {
    let mut m = Matrix::with_fn(2, |_, _| 0);
    assert_eq!(m.dimensions, 2);
    assert_eq!(m.get_row(0), &[0, 0]);
    assert_eq!(m.get_row(1), &[0, 0]);
    m.set(0, 1, 99);
    assert_eq!(m.get_row(0), &[0, 99]);
    assert_eq!(m.get_row(1), &[0, 0]);
    m.set(1, 0, 99);
    assert_eq!(m.get_row(0), &[0, 99]);
    assert_eq!(m.get_row(1), &[99, 0]);
  }

  #[test]
  fn test_get_row() {
    let m = Matrix::with_fn(3, |row, col| (row, col));
    assert_eq!(m.get_row(0), &[(0, 0), (0, 1), (0, 2)]);
    assert_eq!(m.get_row(1), &[(1, 0), (1, 1), (1, 2)]);
    assert_eq!(m.get_row(2), &[(2, 0), (2, 1), (2, 2)]);
  }

  #[test]
  fn test_matrix_reverse_rows() {
    let mut m = Matrix::with_fn(3, |row, col| (row, col));
    m.reverse_rows();
    assert_eq!(m.get_row(0), &[(0, 2), (0, 1), (0, 0)]);
    assert_eq!(m.get_row(1), &[(1, 2), (1, 1), (1, 0)]);
    assert_eq!(m.get_row(2), &[(2, 2), (2, 1), (2, 0)]);
  }

  #[test]
  fn test_creating_matrix() {
    let m = Matrix::with_fn(3, |row, col| (row, col));
    for i in 0..3 {
      for j in 0..3 {
        let &(a, b) = m.get(i, j);
        assert_eq!(a, i);
        assert_eq!(b, j);
      }
    }
  }

  #[test]
  fn test_into_iterator() {
    let m = Matrix::with_fn(3, |row, col| (row, col));
    let collected: Vec<&[(usize, usize)]> = m.into_iter().collect();
    assert_eq!(collected.len(), 3);
    assert_eq!(collected[0], &[(0, 0), (0, 1), (0, 2)]);
    assert_eq!(collected[1], &[(1, 0), (1, 1), (1, 2)]);
    assert_eq!(collected[2], &[(2, 0), (2, 1), (2, 2)]);

    // Take a new iterator to confirm that the matrix wasn't consumed the first
    // time.
    let collected: Vec<&[(usize, usize)]> = m.into_iter().collect();
    assert_eq!(collected.len(), 3);
  }

  #[test]
  #[should_panic]
  fn test_with_data_panics_on_length_mismatch_in_input_vec() {
      Matrix::with_data(3, vec![1,2,3,4,5,6,7,8,9,10]);
  }

  #[test]
  fn test_with_data_works_when_input_vec_len_matches_dimensions() {
      Matrix::with_data(3, vec![1,2,3,4,5,6,7,8,9]);
  }
}
