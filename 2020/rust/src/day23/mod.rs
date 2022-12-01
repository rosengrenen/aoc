use aoc_util::{Solver, SolverOutput};

#[derive(Default)]
pub struct Day23;

impl Solver for Day23 {
  fn part_one(&self, input: &str) -> SolverOutput {
    let input = parse_cups(input);
    let mut forward_links = vec![0; input.len()];
    for i in 0..input.len() - 1 {
      forward_links[input[i] as usize - 1] = input[i + 1];
    }
    forward_links[input[input.len() - 1] as usize - 1] = input[0];

    let mut current = input[0];
    let input_max = *input.iter().max().unwrap();
    for _ in 0..100 {
      let a = forward_links[current as usize - 1];
      let b = forward_links[a as usize - 1];
      let c = forward_links[b as usize - 1];

      let mut destination = current;
      loop {
        if destination == 1 {
          destination = input_max;
        } else {
          destination -= 1;
        }

        if destination != a && destination != b && destination != c {
          break;
        }
      }

      // "remove" picked values
      let next_current = forward_links[c as usize - 1];
      forward_links[current as usize - 1] = next_current;

      // "insert" picked values after destination
      let after_destination = forward_links[destination as usize - 1];
      forward_links[destination as usize - 1] = a;
      forward_links[c as usize - 1] = after_destination;

      current = next_current;
    }

    let mut num = 0;
    let mut next = forward_links[0];
    while next != 1 {
      num *= 10;
      num += next;
      next = forward_links[next as usize - 1];
    }

    SolverOutput::Num(num)
  }

  fn part_two(&self, input: &str) -> SolverOutput {
    const FILL_SIZE: usize = 1_000_000;
    let input = parse_cups(input);
    let mut forward_links = vec![0; FILL_SIZE];
    let input_max = *input.iter().max().unwrap();
    for i in 0..input.len() - 1 {
      forward_links[input[i] as usize - 1] = input[i + 1];
    }
    forward_links[input[input.len() - 1] as usize - 1] = input_max + 1;

    for i in input_max + 1..FILL_SIZE as i64 {
      forward_links[i as usize - 1] = i + 1;
    }
    forward_links[FILL_SIZE - 1] = input[0];

    let mut current = input[0];
    let input_max = FILL_SIZE as i64;
    for _ in 0..10_000_000 {
      let a = forward_links[current as usize - 1];
      let b = forward_links[a as usize - 1];
      let c = forward_links[b as usize - 1];

      let mut destination = current;
      loop {
        if destination == 1 {
          destination = input_max;
        } else {
          destination -= 1;
        }

        if destination != a && destination != b && destination != c {
          break;
        }
      }

      // "remove" picked values
      let next_current = forward_links[c as usize - 1];
      forward_links[current as usize - 1] = next_current;

      // "insert" picked values after destination
      let after_destination = forward_links[destination as usize - 1];
      forward_links[destination as usize - 1] = a;
      forward_links[c as usize - 1] = after_destination;

      current = next_current;
    }

    let first_number = forward_links[0];
    let second_number = forward_links[first_number as usize - 1];

    SolverOutput::Num(first_number * second_number)
  }
}

fn parse_cups(input: &str) -> Vec<i64> {
  input
    .as_bytes()
    .iter()
    .map(|c| (*c - b'0') as i64)
    .collect()
}
