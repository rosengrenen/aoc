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
