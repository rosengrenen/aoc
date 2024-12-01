use std::collections::HashMap;

use aoc_util::{Solver, SolverOutput};

#[derive(Default)]
pub struct Day1;

impl Solver for Day1 {
  fn part_one(&self, input: &str) -> SolverOutput {
    let (mut left, mut right) = parse_lists(input);
    left.sort_unstable();
    right.sort_unstable();

    SolverOutput::Num(
      left
        .into_iter()
        .zip(right)
        .map(|(left, right)| (left - right).abs())
        .sum(),
    )
  }

  fn part_two(&self, input: &str) -> SolverOutput {
    let (left, right) = parse_lists(input);
    let mut frequency = HashMap::<i64, i64>::new();
    right
      .into_iter()
      .for_each(|n| *frequency.entry(n).or_default() += 1);
    SolverOutput::Num(
      left
        .into_iter()
        .map(|n| n * frequency.get(&n).unwrap_or(&0))
        .sum(),
    )
  }
}

fn parse_lists(input: &str) -> (Vec<i64>, Vec<i64>) {
  let mut left_list = Vec::new();
  let mut right_list = Vec::new();
  for line in input.lines() {
    let (left, right) = line.split_once("   ").unwrap();
    left_list.push(left.parse().unwrap());
    right_list.push(right.parse().unwrap());
  }

  (left_list, right_list)
}
