use aoc_util::{Solver, SolverOutput};

#[derive(Default)]
pub struct Day1;

impl Solver for Day1 {
  fn part_one(&self, input: &str) -> SolverOutput {
    let mut inventories = parse_inventories(input);
    inventories.sort_by_key(|inv| -inv);
    SolverOutput::Num(inventories[0])
  }

  fn part_two(&self, input: &str) -> SolverOutput {
    let mut inventories = parse_inventories(input);
    inventories.sort_by_key(|inv| -inv);
    SolverOutput::Num(inventories[0] + inventories[1] + inventories[2])
  }
}

fn parse_inventories(input: &str) -> Vec<i64> {
  input
    .split("\n\n")
    .map(|inv| {
      inv
        .lines()
        .map(|line| line.parse::<i64>().unwrap())
        .sum::<i64>()
    })
    .collect()
}
