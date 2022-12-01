use std::cmp::Ordering;

use aoc_util::{Solver, SolverOutput};

pub struct Day9 {
  preamble_length: i64,
}

impl Default for Day9 {
  fn default() -> Self {
    Self {
      preamble_length: 25,
    }
  }
}

impl Solver for Day9 {
  fn part_one(&self, input: &str) -> SolverOutput {
    println!("preamble length: {}", self.preamble_length);
    let numbers = parse_numbers(input);
    SolverOutput::Num(find_first_invalid(&numbers, self.preamble_length as usize))
  }

  fn part_two(&self, input: &str) -> SolverOutput {
    let numbers = parse_numbers(input);
    SolverOutput::Num(find_first_invalid_contiguous_min_max(
      &numbers,
      self.preamble_length as usize,
    ))
  }
}

fn parse_numbers(input: &str) -> Vec<i64> {
  input.lines().map(|line| line.parse().unwrap()).collect()
}

fn find_first_invalid(numbers: &[i64], preamble_length: usize) -> i64 {
  for i in preamble_length..numbers.len() {
    let current_number = numbers[i];
    if !has_sum(&numbers[(i - preamble_length)..i], current_number) {
      return current_number;
    }
  }

  panic!("No invalid number in sequence");
}

fn find_first_invalid_contiguous_min_max(numbers: &[i64], preamble_length: usize) -> i64 {
  for i in preamble_length..numbers.len() {
    let current_number = numbers[i];
    if !has_sum(&numbers[(i - preamble_length)..i], current_number) {
      let slice = find_contiguous_sum(&numbers[0..i as usize - 1], current_number);

      return slice.iter().min().unwrap() + slice.iter().max().unwrap();
    }
  }

  panic!("No invalid number in sequence");
}

fn has_sum(list: &[i64], target: i64) -> bool {
  for i in list {
    for j in list {
      if i != j && i + j == target {
        return true;
      }
    }
  }

  false
}

fn find_contiguous_sum(list: &[i64], target: i64) -> &[i64] {
  let mut lower_index = 0;
  let mut upper_index = 1;
  let mut sum = list[lower_index] + list[upper_index];
  while upper_index < list.len() {
    match sum.cmp(&target) {
      Ordering::Equal => return &list[lower_index..=upper_index],
      Ordering::Greater => {
        sum -= list[lower_index];
        lower_index += 1;
      }
      Ordering::Less => {
        upper_index += 1;
        sum += list[upper_index];
      }
    };
  }

  panic!("Could not find contiguous sum in list");
}

#[cfg(test)]
mod tests {
  use super::*;
  use aoc_util::test_solver;

  #[test]
  fn part_one_test1() {
    let solver: Box<dyn Solver> = Box::new(Day9 { preamble_length: 5 });
    test_solver(&solver, 9, true, "custom.test1");
  }

  #[test]
  fn part_two_test1() {
    let solver: Box<dyn Solver> = Box::new(Day9 { preamble_length: 5 });
    test_solver(&solver, 9, false, "custom.test1");
  }
}
