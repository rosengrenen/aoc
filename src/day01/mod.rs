use crate::lib::Solver;
use std::cmp;

pub struct Day1Solver;

impl Solver for Day1Solver {
  fn solve(&self, lines: Vec<String>, part_two: bool) -> String {
    let weights: Vec<i32> = lines.iter().map(|s| s.parse::<i32>().unwrap()).collect();
    weights
      .iter()
      .map(|&w| calculate_fuel_cost(w, part_two))
      .sum::<i32>()
      .to_string()
  }
}

fn calculate_fuel_cost(weight: i32, part_two: bool) -> i32 {
  let cost = cmp::max(weight / 3 - 2, 0);
  if part_two && cost > 0 {
    cost + calculate_fuel_cost(cost, part_two)
  } else {
    cost
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn part_one_test_cases() {
    assert_eq!(calculate_fuel_cost(12, false), 2);
    assert_eq!(calculate_fuel_cost(14, false), 2);
    assert_eq!(calculate_fuel_cost(1969, false), 654);
    assert_eq!(calculate_fuel_cost(100756, false), 33583);
  }

  #[test]
  fn part_two_test_cases() {
    assert_eq!(calculate_fuel_cost(14, true), 2);
    assert_eq!(calculate_fuel_cost(1969, true), 966);
    assert_eq!(calculate_fuel_cost(100756, true), 50346);
  }
}
