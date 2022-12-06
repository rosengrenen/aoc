use std::collections::HashSet;

use aoc_util::{Solver, SolverOutput};

#[derive(Default)]
pub struct Day6;

impl Solver for Day6 {
  fn part_one(&self, input: &str) -> SolverOutput {
    SolverOutput::Num(first_marker(input, 4) as i64)
  }

  fn part_two(&self, input: &str) -> SolverOutput {
    SolverOutput::Num(first_marker(input, 14) as i64)
  }
}

fn first_marker(input: &str, len: usize) -> usize {
  for i in 0.. {
    if input[i..i + len].bytes().collect::<HashSet<_>>().len() == len {
      return i + len;
    }
  }

  unreachable!()
}
