use std::cmp::Ordering;

use aoc_util::{Solver, SolverOutput};
use permutohedron::heap_recursive;

#[derive(Default)]
pub struct Day7;

impl Solver for Day7 {
  fn part_one(&self, input: &str) -> SolverOutput {
    let program_orig: Vec<i64> = input.split(',').map(|s| s.parse().unwrap()).collect();
    let mut program = program_orig.clone();

    let mut input = [0, 1, 2, 3, 4];
    let mut permutations = Vec::new();
    heap_recursive(&mut input, |permutation| {
      permutations.push(permutation.to_vec());
    });
    let result = permutations.iter().fold(0, |previous, permutation| {
      let result = run_sequence(&mut program, permutation, false);
      match result.cmp(&previous) {
        Ordering::Greater => result,
        _ => previous,
      }
    });
    SolverOutput::Num(result)
  }

  fn part_two(&self, input: &str) -> SolverOutput {
    let program_orig: Vec<i64> = input.split(',').map(|s| s.parse().unwrap()).collect();
    let mut program = program_orig.clone();
    let mut input = [5, 6, 7, 8, 9];
    let mut permutations = Vec::new();
    heap_recursive(&mut input, |permutation| {
      permutations.push(permutation.to_vec());
    });
    let result = permutations.iter().fold(0, |previous, permutation| {
      let result = run_sequence(&mut program, permutation, true);
      match result.cmp(&previous) {
        Ordering::Greater => result,
        _ => previous,
      }
    });
    SolverOutput::Num(result)
  }
}

fn run_sequence(program: &mut Vec<i64>, sequence: &Vec<i64>, part_two: bool) -> i64 {
  let mut result = 0;
  if part_two {
    let mut index = 0;
    let mut machines: Vec<IntcodeComputer> = sequence
      .iter()
      .map(|_| IntcodeComputer::new(&program))
      .collect();
    let mut final_result = 0;
    loop {
      let current_machine = &mut machines[index % 5];
      current_machine.run(true, |input_index| {
        if input_index == 0 {
          sequence[index % 5]
        } else if index == 0 && input_index == 1 {
          0
        } else {
          result
        }
      });
      result = current_machine.get_output();
      if !current_machine.is_pending() {
        break;
      }
      if index % 5 == 4 {
        final_result = result;
      }
      index += 1;
    }
    result = final_result;
  } else {
    for &phase_setting in sequence.iter() {
      let mut machine = IntcodeComputer::new(&program);
      machine.run(true, |input_index| {
        if input_index == 0 {
          phase_setting
        } else {
          result
        }
      });
      result = machine.get_output();
    }
  }
  result
}

enum State {
  Pending(bool),
  Halted,
}

struct IntcodeComputer {
  instruction_pointer: usize,
  output_register: i64,
  program: Vec<i64>,
  input_index: i64,
  state: State,
}

impl IntcodeComputer {
  fn new(program: &Vec<i64>) -> IntcodeComputer {
    IntcodeComputer {
      instruction_pointer: 0,
      output_register: 0,
      program: program.clone(),
      input_index: 0,
      state: State::Pending(false),
    }
  }

  fn get_n_args(&self, n: usize) -> Vec<(i64, i64)> {
    let mut shift = 100;
    let mut args = Vec::new();
    for offset in 1..=n {
      let mode = (self.program[self.instruction_pointer] / shift) % 10;
      let value = self.program[self.instruction_pointer + offset];
      let resolved = match mode {
        0 => self.program[value as usize],
        1 => value,
        _ => panic!("Invalid mode: {}", mode),
      };
      args.push((value, resolved));
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
    F: Fn(i64) -> i64,
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
    F: Fn(i64) -> i64,
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
      run_sequence(
        &mut vec![3, 15, 3, 16, 1002, 16, 10, 16, 1, 16, 15, 15, 4, 15, 99, 0, 0],
        &vec![4, 3, 2, 1, 0],
        false
      ),
      43210
    );
    assert_eq!(
      run_sequence(
        &mut vec![
          3, 23, 3, 24, 1002, 24, 10, 24, 1002, 23, -1, 23, 101, 5, 23, 23, 1, 24, 23, 23, 4, 23,
          99, 0, 0
        ],
        &vec![0, 1, 2, 3, 4],
        false
      ),
      54321
    );
    assert_eq!(
      run_sequence(
        &mut vec![
          3, 31, 3, 32, 1002, 32, 10, 32, 1001, 31, -2, 31, 1007, 31, 0, 33, 1002, 33, 7, 33, 1,
          33, 31, 31, 1, 32, 31, 31, 4, 31, 99, 0, 0, 0
        ],
        &vec![1, 0, 4, 3, 2],
        false
      ),
      65210
    );
  }

  #[test]
  fn part_two_test_cases() {
    assert_eq!(
      run_sequence(
        &mut vec![
          3, 26, 1001, 26, -4, 26, 3, 27, 1002, 27, 2, 27, 1, 27, 26, 27, 4, 27, 1001, 28, -1, 28,
          1005, 28, 6, 99, 0, 0, 5
        ],
        &vec![9, 8, 7, 6, 5],
        true
      ),
      139629729
    );
    assert_eq!(
      run_sequence(
        &mut vec![
          3, 52, 1001, 52, -5, 52, 3, 53, 1, 52, 56, 54, 1007, 54, 5, 55, 1005, 55, 26, 1001, 54,
          -5, 54, 1105, 1, 12, 1, 53, 54, 53, 1008, 54, 0, 55, 1001, 55, 1, 55, 2, 53, 55, 53, 4,
          53, 1001, 56, -1, 56, 1005, 56, 6, 99, 0, 0, 0, 0, 10
        ],
        &vec![9, 7, 8, 5, 6],
        true
      ),
      18216
    );
  }
}
