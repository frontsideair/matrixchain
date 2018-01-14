extern crate matrixchain;

use matrixchain::*;

fn main() {
  let input = vec![30, 35, 15, 5, 10, 20, 25];
  let (_m, s) = matrix_chain_order(&input);
  print_optimal_parens(&s, 1, 6);
}
