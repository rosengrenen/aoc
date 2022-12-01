use aoc_util::{Solver, SolverOutput};

#[derive(Default)]
pub struct Day5;

impl Solver for Day5 {
  fn part_one(&self, input: &str) -> SolverOutput {
    SolverOutput::Num(input.lines().map(|line| get_seat_id(line)).max().unwrap())
  }

  fn part_two(&self, input: &str) -> SolverOutput {
    let mut seat_ids: Vec<i64> = input.lines().map(|line| get_seat_id(line)).collect();
    seat_ids.sort_unstable();
    let mut prev_id = -999;
    for &current_id in seat_ids.iter() {
      if current_id - prev_id == 2 {
        break;
      }

      prev_id = current_id;
    }

    SolverOutput::Num(prev_id + 1)
  }
}

fn get_seat_id(partition_data: &str) -> i64 {
  let mut seat_id = 0;
  for &c in partition_data.as_bytes() {
    seat_id <<= 1;
    if c == b'B' || c == b'R' {
      seat_id |= 1;
    }
  }
  seat_id
}
