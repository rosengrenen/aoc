use aoc_util::{Solver, SolverOutput};

#[derive(Default)]
pub struct Day25;

impl Solver for Day25 {
  fn part_one(&self, input: &str) -> SolverOutput {
    let (card_pub_key, door_pub_key) = parse_pub_keys(input);

    let mut door_loop_size = 1;
    let mut value = 1;
    loop {
      value *= 7;
      value %= 20_201_227;
      if value == door_pub_key {
        break;
      }

      door_loop_size += 1;
    }

    let mut value = 1;
    for _ in 0..door_loop_size {
      value *= card_pub_key;
      value %= 20_201_227;
    }

    SolverOutput::Num(value)
  }

  fn part_two(&self, _input: &str) -> SolverOutput {
    SolverOutput::Num(0)
  }
}

fn parse_pub_keys(input: &str) -> (i64, i64) {
  let (card_pub_key_raw, door_pub_key_raw) = input.split_once("\n").unwrap();

  (
    card_pub_key_raw.parse().unwrap(),
    door_pub_key_raw.parse().unwrap(),
  )
}
