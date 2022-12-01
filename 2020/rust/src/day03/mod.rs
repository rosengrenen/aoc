use aoc_util::{Solver, SolverOutput};

#[derive(Default)]
pub struct Day3;

impl Solver for Day3 {
  fn part_one(&self, input: &str) -> SolverOutput {
    let lines: Vec<&str> = input.lines().collect();
    SolverOutput::Num(traverse_map(&lines, 3, 1))
  }

  fn part_two(&self, input: &str) -> SolverOutput {
    let lines: Vec<&str> = input.lines().collect();
    SolverOutput::Num(
      traverse_map(&lines, 1, 1)
        * traverse_map(&lines, 3, 1)
        * traverse_map(&lines, 5, 1)
        * traverse_map(&lines, 7, 1)
        * traverse_map(&lines, 1, 2),
    )
  }
}

fn traverse_map(map: &[&str], x_step: usize, y_step: usize) -> i64 {
  map.iter().enumerate().fold(0, |hits, (mut index, row)| {
    if index % y_step != 0 {
      return hits;
    }

    // This line is necessary, else the index is y_step times higher than it should be
    index /= y_step;
    if row.as_bytes()[index * x_step % row.len()] == b'#' {
      hits + 1
    } else {
      hits
    }
  })
}
