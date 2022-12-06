use aoc_util::{Solver, SolverOutput};

#[derive(Default)]
pub struct Day5;

impl Solver for Day5 {
  fn part_one(&self, input: &str) -> SolverOutput {
    let (stacks, instrs) = input.split_once("\n\n").unwrap();
    let mut stacks = parse_stacks(stacks);
    for instr in instrs.lines() {
      let parts = instr.split(" ").collect::<Vec<_>>();
      let num = parts[1].parse().unwrap();
      let from = parts[3].parse::<usize>().unwrap() - 1;
      let to = parts[5].parse::<usize>().unwrap() - 1;
      for _ in 0..num {
        let crt = stacks[from].pop().unwrap();
        stacks[to].push(crt);
      }
    }
    SolverOutput::String(
      stacks
        .into_iter()
        .map(|stack| *stack.last().unwrap())
        .collect(),
    )
  }

  fn part_two(&self, input: &str) -> SolverOutput {
    let (stacks, instrs) = input.split_once("\n\n").unwrap();
    let mut stacks = parse_stacks(stacks);
    for instr in instrs.lines() {
      let parts = instr.split(" ").collect::<Vec<_>>();
      let num = parts[1].parse().unwrap();
      let from = parts[3].parse::<usize>().unwrap() - 1;
      let to = parts[5].parse::<usize>().unwrap() - 1;
      let mut crts = vec![];
      for _ in 0..num {
        crts.push(stacks[from].pop().unwrap());
      }

      for crt in crts.into_iter().rev() {
        stacks[to].push(crt);
      }
    }

    SolverOutput::String(
      stacks
        .into_iter()
        .map(|stack| *stack.last().unwrap())
        .collect(),
    )
  }
}

fn parse_stacks(input: &str) -> Vec<Vec<char>> {
  let mut stacks = vec![vec![]; 9];
  for stack_line in input.lines().rev().skip(1) {
    for (i, c) in stack_line.chars().skip(1).step_by(4).enumerate() {
      if c != ' ' {
        stacks[i].push(c);
      }
    }
  }

  stacks
}
