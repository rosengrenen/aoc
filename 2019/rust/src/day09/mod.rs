use aoc_util::{Solver, SolverOutput};

#[derive(Default)]
pub struct Day9;

impl Solver for Day9 {
  fn part_one(&self, input: &str) -> SolverOutput {
    let program_orig: Vec<i64> = input
      .split(',')
      .map(|s| s.parse::<i64>().unwrap())
      .collect();
    let output = collect_outputs(&program_orig, &vec![1])[0];
    SolverOutput::Num(output)
  }

  fn part_two(&self, input: &str) -> SolverOutput {
    let program_orig: Vec<i64> = input
      .split(',')
      .map(|s| s.parse::<i64>().unwrap())
      .collect();
    let output = collect_outputs(&program_orig, &vec![2])[0];
    SolverOutput::Num(output)
  }
}

fn collect_outputs(program: &Vec<i64>, input: &Vec<i64>) -> Vec<i64> {
  let mut computer = IntcodeComputer::new(program);
  let mut outputs: Vec<i64> = Vec::new();
  loop {
    computer.run(true, |input_index| input[input_index]);
    if !computer.is_pending() {
      break;
    }
    outputs.push(computer.get_output());
  }
  outputs
}

enum State {
  Pending(bool),
  Halted,
}

struct IntcodeComputer {
  instruction_pointer: usize,
  output_register: i64,
  program: Vec<i64>,
  input_index: usize,
  state: State,
  relative_base: i64,
}

impl IntcodeComputer {
  fn new(program: &Vec<i64>) -> IntcodeComputer {
    let mut program = program.clone();
    program.append(&mut vec![0; 1000]);
    IntcodeComputer {
      instruction_pointer: 0,
      output_register: 0,
      program,
      input_index: 0,
      state: State::Pending(false),
      relative_base: 0,
    }
  }

  fn get_n_args(&self, n: usize) -> Vec<(i64, i64)> {
    let mut shift = 100;
    let mut args = Vec::new();
    for offset in 1..=n {
      let mode = (self.program[self.instruction_pointer] / shift) % 10;
      let value = self.program[self.instruction_pointer + offset];
      match mode {
        0 => args.push((value, self.program[value as usize])),
        1 => args.push((value, value)),
        2 => args.push((
          self.relative_base + value,
          self.program[(self.relative_base + value) as usize],
        )),
        _ => panic!("Invalid mode: {}", mode),
      };
      shift *= 10;
    }
    args
  }

  fn get_output(&self) -> i64 {
    self.output_register
  }

  fn is_pending(&self) -> bool {
    match self.state {
      State::Halted => false,
      _ => true,
    }
  }

  fn run<F>(&mut self, pause_on_output: bool, input: F)
  where
    F: Fn(usize) -> i64,
  {
    loop {
      self.step(&input);
      match self.state {
        State::Halted => return,
        State::Pending(true) => {
          if pause_on_output {
            return;
          }
        }
        State::Pending(false) => (),
      }
    }
  }

  fn step<F>(&mut self, input: &F)
  where
    F: Fn(usize) -> i64,
  {
    let instruction_options = self.program[self.instruction_pointer];
    let op_code = instruction_options % 100;

    if op_code == 99 {
      self.state = State::Halted;
      return;
    }

    match op_code {
      1 => {
        let args = self.get_n_args(3);
        self.program[args[2].0 as usize] = args[0].1 + args[1].1;
        self.instruction_pointer += 4;
        self.state = State::Pending(false);
      }
      2 => {
        let args = self.get_n_args(3);
        self.program[args[2].0 as usize] = args[0].1 * args[1].1;
        self.instruction_pointer += 4;
        self.state = State::Pending(false);
      }
      3 => {
        let args = self.get_n_args(1);
        self.program[args[0].0 as usize] = input(self.input_index);
        self.input_index += 1;
        self.instruction_pointer += 2;
        self.state = State::Pending(false);
      }
      4 => {
        let args = self.get_n_args(1);
        self.output_register = args[0].1;
        self.instruction_pointer += 2;
        self.state = State::Pending(true);
      }
      5 => {
        let args = self.get_n_args(2);
        if args[0].1 != 0 {
          self.instruction_pointer = args[1].1 as usize;
        } else {
          self.instruction_pointer += 3;
        }
        self.state = State::Pending(false);
      }
      6 => {
        let args = self.get_n_args(2);
        if args[0].1 == 0 {
          self.instruction_pointer = args[1].1 as usize;
        } else {
          self.instruction_pointer += 3;
        }
        self.state = State::Pending(false);
      }
      7 => {
        let args = self.get_n_args(3);
        self.program[args[2].0 as usize] = if args[0].1 < args[1].1 { 1 } else { 0 };
        self.instruction_pointer += 4;
        self.state = State::Pending(false);
      }
      8 => {
        let args = self.get_n_args(3);
        self.program[args[2].0 as usize] = if args[0].1 == args[1].1 { 1 } else { 0 };
        self.instruction_pointer += 4;
        self.state = State::Pending(false);
      }
      9 => {
        let args = self.get_n_args(1);
        self.relative_base += args[0].1;
        self.instruction_pointer += 2;
        self.state = State::Pending(false);
      }
      _ => panic!(
        "Invalid op code: {} at {}",
        op_code, self.instruction_pointer
      ),
    };
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn part_one_test_cases() {
    assert_eq!(
      collect_outputs(
        &vec![109, 1, 204, -1, 1001, 100, 1, 100, 1008, 100, 16, 101, 1006, 101, 0, 99],
        &vec![]
      ),
      vec![109, 1, 204, -1, 1001, 100, 1, 100, 1008, 100, 16, 101, 1006, 101, 0, 99]
    );
    assert_eq!(
      collect_outputs(&vec![1102, 34915192, 34915192, 7, 4, 7, 99, 0], &vec![]),
      vec![34915192 * 34915192]
    );
    assert_eq!(
      collect_outputs(&vec![104, 1125899906842624, 99], &vec![]),
      vec![1125899906842624]
    );
  }
}
