use aoc_util::{Solver, SolverOutput};

#[derive(Default)]
pub struct Day15;

impl Solver for Day15 {
  fn part_one(&self, input: &str) -> SolverOutput {
    let input = parse_numbers(input);
    SolverOutput::Num(find_nth_van_eck(&input, 2020))
  }

  fn part_two(&self, input: &str) -> SolverOutput {
    let input = parse_numbers(input);
    SolverOutput::Num(find_nth_van_eck(&input, 30_000_000))
  }
}

fn parse_numbers(input: &str) -> Vec<i64> {
  input.split(',').map(|c| c.parse().unwrap()).collect()
}

fn find_nth_van_eck(starting_numbers: &[i64], n: i64) -> i64 {
  let mut last_pos = vec![0; n as usize];
  let mut last_num = 0;
  for (index, &num) in starting_numbers.iter().enumerate() {
    last_pos[num as usize] = index as i64 + 1;
    last_num = num;
  }

  let mut index = starting_numbers.len() as i64;
  while index < n {
    let mut next_num = 0;
    let last_num_pos = last_pos[last_num as usize];
    if last_num_pos != 0 {
      next_num = index - last_num_pos;
      last_pos[last_num as usize] = index;
    } else {
      last_pos[last_num as usize] = index;
    }

    last_num = next_num;
    index += 1;
  }

  last_num
}
