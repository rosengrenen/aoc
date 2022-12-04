use std::collections::HashSet;

use aoc_util::{Solver, SolverOutput};

#[derive(Default)]
pub struct Day3;

impl Solver for Day3 {
  fn part_one(&self, input: &str) -> SolverOutput {
    SolverOutput::Num(
      input
        .lines()
        .map(|line| {
          let bytes = line.as_bytes();
          let comp1 = &bytes[..line.len() / 2];
          let comp2 = &bytes[line.len() / 2..];
          let comp1 = comp1.into_iter().copied().collect::<HashSet<_>>();
          let comp2 = comp2.into_iter().copied().collect::<HashSet<_>>();
          *comp1.intersection(&comp2).next().unwrap()
        })
        .map(priority)
        .sum(),
    )
  }

  fn part_two(&self, input: &str) -> SolverOutput {
    SolverOutput::Num(
      input
        .lines()
        .map(|line| line.as_bytes().into_iter().copied().collect::<HashSet<_>>())
        .array_chunks::<3>()
        .map(|[inv1, inv2, inv3]| {
          *inv1
            .intersection(&inv2)
            .copied()
            .collect::<HashSet<_>>()
            .intersection(&inv3)
            .next()
            .unwrap()
        })
        .map(|a| {
          // println!("{}", a);
          a
        })
        .map(priority)
        .map(|a| {
          // println!("{}", a);
          a
        })
        .sum(),
    )
  }
}

fn priority(c: u8) -> i64 {
  if c <= b'Z' {
    c as i64 - b'A' as i64 + 27
  } else {
    c as i64 - b'a' as i64 + 1
  }
}
