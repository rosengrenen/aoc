use aoc_util::{Solver, SolverOutput};

#[derive(Default)]
pub struct Day4;

impl Solver for Day4 {
  fn part_one(&self, input: &str) -> SolverOutput {
    SolverOutput::Num(
      input
        .lines()
        .map(parse_sched)
        .filter(|&(r0, r1)| range_within(r0, r1) || range_within(r1, r0))
        .count() as i64,
    )
  }

  fn part_two(&self, input: &str) -> SolverOutput {
    SolverOutput::Num(
      input
        .lines()
        .map(parse_sched)
        .filter(|&(r0, r1)| range_overlap(r0, r1))
        .count() as i64,
    )
  }
}

type Range = (i64, i64);

fn range_within((is, ie): Range, (os, oe): Range) -> bool {
  is >= os && ie <= oe
}

fn range_overlap((s0, e0): Range, (s1, e1): Range) -> bool {
  !(s1 > e0 || e0 < s1) && !(s0 > e1 || e1 < s0)
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
