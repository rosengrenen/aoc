use aoc_util::{Solver, SolverOutput};

#[derive(Default)]
pub struct Day6;

impl Solver for Day6 {
  fn part_one(&self, input: &str) -> SolverOutput {
    let fish_state = parse_fish_state(input);
    let fish_state = simulate_days(fish_state, 80);
    SolverOutput::Num(fish_state.iter().sum())
  }

  fn part_two(&self, input: &str) -> SolverOutput {
    let fish_state = parse_fish_state(input);
    let fish_state = simulate_days(fish_state, 256);
    SolverOutput::Num(fish_state.iter().sum())
  }
}

fn simulate_days(mut fish_state: [i64; 9], days: i64) -> [i64; 9] {
  for _ in 0..days {
    let new_fish = fish_state[0];
    for state in 0..8 {
      fish_state[state] = fish_state[state + 1];
    }

    fish_state[6] += new_fish;
    fish_state[8] = new_fish;
  }
  fish_state
}

fn parse_fish_state(input: &str) -> [i64; 9] {
  let mut fish_state = [0; 9];
  input
    .split(",")
    .map(|num| num.parse().unwrap())
    .for_each(|num: usize| {
      fish_state[num] += 1;
    });
  fish_state
}

#[cfg(test)]
mod benches {
  use super::*;
  use aoc_util::get_input;
  use test::{black_box, Bencher};

  #[bench]
  fn parse(bencher: &mut Bencher) {
    let input = get_input(2021, 6).unwrap();
    bencher.iter(|| parse_fish_state(black_box(&input)));
  }
}
