use aoc_util::{Solver, SolverOutput};

#[derive(Default)]
pub struct Day1;

impl Solver for Day1 {
  fn part_one(&self, input: &str) -> SolverOutput {
    let depths = input.lines().map(|line| line.parse::<i64>().unwrap());

    let mut times_increased = 0;
    let mut prev_depth = None;
    for depth in depths {
      if let Some(prev_depth) = prev_depth {
        if depth > prev_depth {
          times_increased += 1;
        }
      }

      prev_depth = Some(depth);
    }

    SolverOutput::Num(times_increased)
  }

  fn part_two(&self, input: &str) -> SolverOutput {
    let depths: Vec<i64> = input.lines().map(|line| line.parse().unwrap()).collect();

    let mut times_increased = 0;
    let mut prev_window = None;
    for window_end in 3..=depths.len() {
      let window_start = window_end - 3;
      let window: i64 = depths[window_start..window_end].iter().sum();

      if let Some(prev_sum) = prev_window {
        if window > prev_sum {
          times_increased += 1;
        }
      }

      prev_window = Some(window);
    }

    SolverOutput::Num(times_increased)
  }
}
