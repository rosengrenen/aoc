use std::collections::HashMap;

use aoc_util::{Solver, SolverOutput};

#[derive(Default)]
pub struct Day11;

impl Solver for Day11 {
  fn part_one(&self, input: &str) -> SolverOutput {
    let mut cache = HashMap::new();
    SolverOutput::Num(parse(input).map(|stone| blink(&mut cache, stone, 25)).sum())
  }

  fn part_two(&self, input: &str) -> SolverOutput {
    let mut cache = HashMap::new();
    SolverOutput::Num(parse(input).map(|stone| blink(&mut cache, stone, 75)).sum())
  }
}

fn blink(cache: &mut HashMap<(i64, i64), i64>, stone: i64, blinks: i64) -> i64 {
  if blinks == 0 {
    return 1;
  }

  if let Some(num_stones) = cache.get(&(stone, blinks)) {
    return *num_stones;
  }

  let num_stones = match stone {
    0 => blink(cache, 1, blinks - 1),
    stone => {
      let stone_str = stone.to_string();
      let stone_len = stone_str.len();
      if stone_len % 2 == 0 {
        let stone1 = stone_str[..stone_len / 2].parse().unwrap();
        let stone2 = stone_str[stone_len / 2..].parse().unwrap();
        blink(cache, stone1, blinks - 1) + blink(cache, stone2, blinks - 1)
      } else {
        blink(cache, stone * 2024, blinks - 1)
      }
    }
  };

  cache.insert((stone, blinks), num_stones);
  return num_stones;
}

fn parse(input: &str) -> impl Iterator<Item = i64> + '_ {
  input.split_whitespace().map(|p| p.parse().unwrap())
}
