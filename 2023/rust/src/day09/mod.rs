use aoc_util::{Solver, SolverOutput};

#[derive(Default)]
pub struct Day9;

impl Solver for Day9 {
  fn part_one(&self, input: &str) -> SolverOutput {
    let series = parse(input);
    let mut sum = 0;
    for serie in series {
      let a = calc_derivatives(serie);
      let mut prediction = *a.last().unwrap().last().unwrap();
      for i in (0..a.len() - 1).rev() {
        prediction = a[i].last().unwrap() + prediction;
      }

      sum += prediction;
    }
    SolverOutput::Num(sum)
  }

  fn part_two(&self, input: &str) -> SolverOutput {
    let series = parse(input);
    let mut sum = 0;
    for serie in series {
      let a = calc_derivatives(serie);
      let mut prediction = *a.last().unwrap().first().unwrap();
      for i in (0..a.len() - 1).rev() {
        prediction = a[i].first().unwrap() - prediction;
      }

      sum += prediction;
    }
    SolverOutput::Num(sum)
  }
}

fn calc_derivatives(serie: Vec<i64>) -> Vec<Vec<i64>> {
  let mut derivatives = vec![serie];
  while derivatives.last().unwrap().iter().any(|&n| n != 0) {
    let last = derivatives.last().unwrap();
    let mut a = Vec::new();
    a.resize(last.len() - 1, 0);
    for i in 0..a.len() {
      a[i] = last[i + 1] - last[i];
    }

    derivatives.push(a);
  }

  derivatives
}

fn parse<'a>(input: &'a str) -> Vec<Vec<i64>> {
  input
    .lines()
    .map(|line| {
      line
        .split_whitespace()
        .map(|n| n.parse().unwrap())
        .collect()
    })
    .collect()
}
