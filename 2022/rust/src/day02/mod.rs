use aoc_util::{Solver, SolverOutput};

#[derive(Default)]
pub struct Day2;

impl Solver for Day2 {
  fn part_one(&self, input: &str) -> SolverOutput {
    SolverOutput::Num(parse_matches(input).map(|(elf, me)| score(elf, me)).sum())
  }

  fn part_two(&self, input: &str) -> SolverOutput {
    SolverOutput::Num(
      parse_matches(input)
        .map(|(elf, me)| selectscore(elf, me))
        .sum(),
    )
  }
}

fn parse_matches<'a>(input: &'a str) -> impl Iterator<Item = (i64, i64)> + 'a {
  input.lines().map(|line| {
    (
      line.as_bytes()[0] as i64 - b'A' as i64,
      line.as_bytes()[2] as i64 - b'X' as i64,
    )
  })
}

fn score(elf: i64, me: i64) -> i64 {
  ((me - elf + 4) % 3) * 3 + me + 1
}

fn selectscore(elf: i64, me: i64) -> i64 {
  score(elf, (elf + me + 2) % 3)
}
