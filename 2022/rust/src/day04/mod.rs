use std::ops::RangeInclusive;

use aoc_util::{Solver, SolverOutput};

#[derive(Default)]
pub struct Day4;

impl Solver for Day4 {
  fn part_one(&self, input: &str) -> SolverOutput {
    SolverOutput::Num(
      input
        .lines()
        .map(parse_sched)
        .filter(|(r0, r1)| r0.contains_r(r1) || r1.contains_r(r0))
        .count() as i64,
    )
  }

  fn part_two(&self, input: &str) -> SolverOutput {
    SolverOutput::Num(
      input
        .lines()
        .map(parse_sched)
        .filter(|(r0, r1)| r0.overlaps_r(r1))
        .count() as i64,
    )
  }
}

trait RangeExt<T> {
  fn contains_r(&self, other: &RangeInclusive<T>) -> bool;
  fn overlaps_r(&self, other: &RangeInclusive<T>) -> bool;
}

impl<T: PartialOrd> RangeExt<T> for RangeInclusive<T> {
  fn contains_r(&self, other: &RangeInclusive<T>) -> bool {
    self.start() <= other.start() && self.end() >= other.end()
  }

  fn overlaps_r(&self, other: &RangeInclusive<T>) -> bool {
    !(other.start() > self.end() || self.end() < other.start())
      && !(self.start() > other.end() || other.end() < self.start())
  }
}

fn parse_sched(input: &str) -> (RangeInclusive<i64>, RangeInclusive<i64>) {
  let (elf1, elf2) = input.split_once(",").unwrap();
  let (start0, end0) = elf1.split_once("-").unwrap();
  let (start1, end1) = elf2.split_once("-").unwrap();
  (
    (start0.parse::<i64>().unwrap()..=end0.parse::<i64>().unwrap()),
    (start1.parse::<i64>().unwrap()..=end1.parse::<i64>().unwrap()),
  )
}
