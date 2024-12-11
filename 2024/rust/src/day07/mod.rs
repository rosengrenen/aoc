use aoc_util::{Solver, SolverOutput};

#[derive(Default)]
pub struct Day7;

impl Solver for Day7 {
  fn part_one(&self, input: &str) -> SolverOutput {
    SolverOutput::Num(
      input
        .lines()
        .map(parse)
        .filter(|(answer, nums)| combinations(*answer, nums[0], &nums[1..]))
        .map(|(answer, _)| answer)
        .sum(),
    )
  }

  fn part_two(&self, input: &str) -> SolverOutput {
    SolverOutput::Num(
      input
        .lines()
        .map(parse)
        .filter(|(answer, nums)| combinations2(*answer, nums[0], &nums[1..]))
        .map(|(answer, _)| answer)
        .sum(),
    )
  }
}

fn combinations(answer: i64, acc: i64, inputs: &[i64]) -> bool {
  match inputs {
    [] => acc == answer,
    [next, rest @ ..] if acc <= answer => {
      combinations(answer, acc * next, rest) || combinations(answer, acc + next, rest)
    }
    _ => false,
  }
}

fn combinations2(answer: i64, acc: i64, inputs: &[i64]) -> bool {
  match inputs {
    [] => acc == answer,
    [next, rest @ ..] if acc <= answer => {
      combinations2(answer, acc * next, rest)
        || combinations2(answer, acc + next, rest)
        || combinations2(answer, format!("{}{}", acc, next).parse().unwrap(), rest)
    }
    _ => false,
  }
}

fn parse(line: &str) -> (i64, Vec<i64>) {
  let (answer, nums) = line.split_once(": ").unwrap();
  (
    answer.parse().unwrap(),
    nums
      .split_whitespace()
      .map(|num| num.parse().unwrap())
      .collect(),
  )
}
