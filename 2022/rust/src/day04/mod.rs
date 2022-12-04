use aoc_util::{Solver, SolverOutput};

#[derive(Default)]
pub struct Day4;

impl Solver for Day4 {
  fn part_one(&self, input: &str) -> SolverOutput {
    SolverOutput::Num(
      input
        .lines()
        .map(parse_sched)
        .filter(|&(r0, r1)| r0.contains(&r1) || r1.contains(&r0))
        .count() as i64,
    )
  }

  fn part_two(&self, input: &str) -> SolverOutput {
    SolverOutput::Num(
      input
        .lines()
        .map(parse_sched)
        .filter(|&(r0, r1)| r0.overlaps(&r1))
        .count() as i64,
    )
  }
}

type Range = (i64, i64);

trait RangeExt {
  fn contains(&self, other: &Range) -> bool;
  fn overlaps(&self, other: &Range) -> bool;
}

impl RangeExt for Range {
  fn contains(&self, other: &Range) -> bool {
    self.0 <= other.0 && self.1 >= other.1
  }

  fn overlaps(&self, other: &Range) -> bool {
    !(other.0 > self.1 || self.1 < other.0) && !(self.0 > other.1 || other.1 < self.0)
  }
}

fn parse_sched(input: &str) -> (Range, Range) {
  let (elf1, elf2) = input.split_once(",").unwrap();
  let (s1, e1) = elf1.split_once("-").unwrap();
  let (s2, e2) = elf2.split_once("-").unwrap();
  (
    (s1.parse().unwrap(), e1.parse().unwrap()),
    (s2.parse().unwrap(), e2.parse().unwrap()),
  )
}
