use std::collections::BinaryHeap;

use aoc_util::{Solver, SolverOutput};

#[derive(Default)]
pub struct Day1;

impl Solver for Day1 {
  fn part_one(&self, input: &str) -> SolverOutput {
    SolverOutput::Num(parse_invs(input).max().unwrap())
  }

  fn part_two(&self, input: &str) -> SolverOutput {
    let mut invs = parse_invs(input).collect::<BinaryHeap<_>>();
    SolverOutput::Num(
      [invs.pop(), invs.pop(), invs.pop()]
        .into_iter()
        .flatten()
        .sum(),
    )
  }
}

fn parse_invs<'a>(input: &'a str) -> impl Iterator<Item = i64> + 'a {
  input.split("\n\n").map(|inv| {
    inv
      .lines()
      .map(|line| line.parse::<i64>().unwrap())
      .sum::<i64>()
  })
}
