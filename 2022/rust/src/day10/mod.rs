use aoc_util::{Solver, SolverOutput};

#[derive(Default)]
pub struct Day10;

impl Solver for Day10 {
  fn part_one(&self, input: &str) -> SolverOutput {
    let signal_strengths = parse_signal_strengths(input);
    SolverOutput::Num(
      signal_strengths
        .into_iter()
        .skip(20)
        .step_by(40)
        .take(6)
        .sum(),
    )
  }

  fn part_two(&self, input: &str) -> SolverOutput {
    SolverOutput::String(parse_crt(input))
  }
}

fn parse_signal_strengths(input: &str) -> Vec<i64> {
  let mut reg = 1;
  let mut signal_strengths = vec![];
  let mut cycle = 1;
  signal_strengths.push(reg * cycle);
  for line in input.lines() {
    signal_strengths.push(reg * cycle);
    cycle += 1;
    if line.starts_with("addx") {
      let add_val: i64 = line[5..].parse().unwrap();
      signal_strengths.push(reg * cycle);
      reg += add_val;
      cycle += 1;
    }
  }

  signal_strengths
}

fn parse_crt(input: &str) -> String {
  let mut reg = 1;
  let mut cycle: usize = 0;
  let mut crt = [[false; 40]; 6];
  for line in input.lines() {
    if (reg - (cycle as i64 % 40)).abs() < 2 {
      crt[(cycle) / 40][(cycle) % 40] = true;
    }

    cycle += 1;
    if line.starts_with("addx") {
      let add_val: i64 = line[5..].parse().unwrap();
      if (reg - (cycle as i64 % 40)).abs() < 2 {
        crt[(cycle) / 40][(cycle) % 40] = true;
      }

      reg += add_val;
      cycle += 1;
    }
  }

  crt
    .into_iter()
    .map(|line| {
      line
        .into_iter()
        .map(|lit| if lit { '#' } else { '.' })
        .collect::<String>()
    })
    .intersperse("\n".to_string())
    .collect()
}
