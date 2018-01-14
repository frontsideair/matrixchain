use std::ops::Index;
use std::ops::IndexMut;
use std::usize;

#[derive(Debug)]
pub struct Matrix {
  data: Vec<usize>,
  rows: usize,
  cols: usize,
}

impl Matrix {
  fn new(rows: usize, cols: usize) -> Matrix {
    Matrix {
      data: vec![0; rows * cols],
      rows,
      cols,
    }
  }
}

impl Index<(usize, usize)> for Matrix {
  type Output = usize;
  fn index(&self, (i, j): (usize, usize)) -> &usize {
    &self.data[i * self.rows + j]
  }
}

impl IndexMut<(usize, usize)> for Matrix {
  fn index_mut(&mut self, (i, j): (usize, usize)) -> &mut usize {
    &mut self.data[i * self.rows + j]
  }
}

pub fn matrix_chain_order(p: &Vec<usize>) -> (Matrix, Matrix) {
  let n = p.len() - 1;
  let mut m = Matrix::new(n + 1, n + 1);
  let mut s = Matrix::new(n, n + 1);

  for i in 1..n + 1 {
    m[(i, i)] = 0;
  }

  for l in 2..n + 1 {
    // l is the chain length
    for i in 1..n - l + 2 {
      let j = i + l - 1;
      m[(i, j)] = usize::MAX;

      for k in i..j {
        let q = m[(i, k)] + m[(k + 1, j)] + p[i - 1] * p[k] * p[j];

        if q < m[(i, j)] {
          m[(i, j)] = q;
          s[(i, j)] = k;
        }
      }
    }
  }
  return (m, s);
}

pub fn print_optimal_parens(s: &Matrix, i: usize, j: usize) {
  if i == j {
    print!("A{}", i);
  } else {
    print!("(");
    print_optimal_parens(&s, i, s[(i, j)]);
    print_optimal_parens(&s, s[(i, j)] + 1, j);
    print!(")");
  }
}

pub fn memoized_matrix_chain(p: &Vec<usize>) -> usize {
  let n = p.len() - 1;
  let mut m = Matrix::new(n + 1, n + 1);
  for i in 1..n + 1 {
    for j in i..n + 1 {
      m[(i, j)] = usize::MAX;
    }
  }
  lookup_chain(&mut m, p, 1, n)
}

fn lookup_chain(m: &mut Matrix, p: &Vec<usize>, i: usize, j: usize) -> usize {
  if m[(i, j)] < usize::MAX {
    return m[(i, j)];
  }
  if i == j {
    m[(i, j)] = 0;
  } else {
    for k in i..j {
      let q = lookup_chain(m, p, i, k) + lookup_chain(m, p, k + 1, j) + p[i - 1] * p[k] * p[j];

      if q < m[(i, j)] {
        m[(i, j)] = q;
      }
    }
  }

  return m[(i, j)];
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test1() {
    let input = vec![30, 35, 15, 5, 10, 20, 25];
    let (m, s) = matrix_chain_order(&input);
    println!("{:?}\n{:?}", m, s);
    assert_eq!(m[(1, 6)], 15_125);
  }

  #[test]
  fn test2() {
    let input = vec![30, 35, 15, 5, 10, 20, 25];
    assert_eq!(memoized_matrix_chain(&input), 15_125);
  }
}
