use std::cmp::Ordering;

use aoc_util::{Solver, SolverOutput};

#[derive(Default)]
pub struct Day1;

impl Solver for Day1 {
  fn part_one(&self, input: &str) -> SolverOutput {
    let mut numbers = parse_numbers(input);
    numbers.sort_unstable();
    if let Some((first, second)) = sum_in_list(&numbers, 2020) {
      SolverOutput::Num(first * second)
    } else {
      panic!("Could not find an answer");
    }
  }

  fn part_two(&self, input: &str) -> SolverOutput {
    let mut numbers = parse_numbers(input);
    numbers.sort_unstable();
    for &first in numbers.iter() {
      if let Some((second, third)) = sum_in_list(&numbers, 2020 - first) {
        return SolverOutput::Num(first * second * third);
      }
    }

    panic!("Could not find an answer");
  }
}

fn parse_numbers(input: &str) -> Vec<i64> {
  input
    .lines()
    .map(|line| line.parse::<i64>().unwrap())
    .collect()
}

fn sum_in_list(list: &[i64], target: i64) -> Option<(i64, i64)> {
  let mut lower_index = 0;
  let mut upper_index = list.len() - 1;
  while lower_index < upper_index {
    let sum = list[lower_index] + list[upper_index];
    match sum.cmp(&target) {
      Ordering::Equal => return Some((list[lower_index], list[upper_index])),
      Ordering::Greater => upper_index -= 1,
      Ordering::Less => lower_index += 1,
    }
  }

  None
}
