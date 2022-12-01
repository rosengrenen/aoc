use aoc_util::{Solver, SolverOutput};

#[derive(Default)]
pub struct Day5;

impl Solver for Day5 {
  fn part_one(&self, input: &str) -> SolverOutput {
    let mut program: Vec<i64> = input.split(',').map(|s| s.parse().unwrap()).collect();
    let result = run_program(&mut program, 1);
    SolverOutput::Num(result)
  }

  fn part_two(&self, input: &str) -> SolverOutput {
    let mut program: Vec<i64> = input.split(',').map(|s| s.parse().unwrap()).collect();
    let result = run_program(&mut program, 5);
    SolverOutput::Num(result)
  }
}

fn get_value(program: &Vec<i64>, value: i64, mode: i64) -> i64 {
  match mode {
    0 => program[value as usize],
    1 => value,
    _ => panic!("Invalid mode: {}", mode),
  }
}

fn run_program(program: &mut Vec<i64>, input: i64) -> i64 {
  let mut output = -1;
  let mut instruction_pointer: usize = 0;
  loop {
    let instruction_options = program[instruction_pointer];
    let op_code = instruction_options % 100;
    let a_mode = (instruction_options / 100) % 10;
    let b_mode = (instruction_options / 1000) % 10;

    if op_code == 99 {
      break;
    }

    let a_value = program[instruction_pointer + 1];
    let b_value = program[instruction_pointer + 2];
    let c_value = program[instruction_pointer + 3];

    match op_code {
      1 => {
        let a = get_value(program, a_value, a_mode);
        let b = get_value(program, b_value, b_mode);
        program[c_value as usize] = a + b;
        instruction_pointer += 4;
      }
      2 => {
        let a = get_value(program, a_value, a_mode);
        let b = get_value(program, b_value, b_mode);
        program[c_value as usize] = a * b;
        instruction_pointer += 4;
      }
      3 => {
        program[a_value as usize] = input;
        instruction_pointer += 2;
      }
      4 => {
        output = get_value(program, a_value, a_mode);
        instruction_pointer += 2;
      }
      5 => {
        let a = get_value(program, a_value, a_mode);
        let b = get_value(program, b_value, b_mode);
        if a != 0 {
          instruction_pointer = b as usize;
        } else {
          instruction_pointer += 3;
        }
      }
      6 => {
        let a = get_value(program, a_value, a_mode);
        let b = get_value(program, b_value, b_mode);
        if a == 0 {
          instruction_pointer = b as usize;
        } else {
          instruction_pointer += 3;
        }
      }
      7 => {
        let a = get_value(program, a_value, a_mode);
        let b = get_value(program, b_value, b_mode);
        program[c_value as usize] = if a < b { 1 } else { 0 };
        instruction_pointer += 4;
      }
      8 => {
        let a = get_value(program, a_value, a_mode);
        let b = get_value(program, b_value, b_mode);
        program[c_value as usize] = if a == b { 1 } else { 0 };
        instruction_pointer += 4;
      }
      _ => panic!("Invalid op code: {} at {}", op_code, instruction_pointer),
    };
  }
  return output;
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn part_two_test1() {
    // Position mode
    assert_eq!(
      run_program(
        &mut vec![3, 12, 6, 12, 15, 1, 13, 14, 13, 4, 13, 99, -1, 0, 1, 9],
        0
      ),
      0
    );
    assert_eq!(
      run_program(
        &mut vec![3, 12, 6, 12, 15, 1, 13, 14, 13, 4, 13, 99, -1, 0, 1, 9],
        1
      ),
      1
    );
    assert_eq!(
      run_program(
        &mut vec![3, 12, 6, 12, 15, 1, 13, 14, 13, 4, 13, 99, -1, 0, 1, 9],
        17
      ),
      1
    );

    // Immediate mode
    assert_eq!(
      run_program(
        &mut vec![3, 3, 1105, -1, 9, 1101, 0, 0, 12, 4, 12, 99, 1],
        0
      ),
      0
    );
    assert_eq!(
      run_program(
        &mut vec![3, 3, 1105, -1, 9, 1101, 0, 0, 12, 4, 12, 99, 1],
        1
      ),
      1
    );
    assert_eq!(
      run_program(
        &mut vec![3, 3, 1105, -1, 9, 1101, 0, 0, 12, 4, 12, 99, 1],
        17
      ),
      1
    );

    // Larger sample
    assert_eq!(
      run_program(
        &mut vec![
          3, 21, 1008, 21, 8, 20, 1005, 20, 22, 107, 8, 21, 20, 1006, 20, 31, 1106, 0, 36, 98, 0,
          0, 1002, 21, 125, 20, 4, 20, 1105, 1, 46, 104, 999, 1105, 1, 46, 1101, 1000, 1, 20, 4,
          20, 1105, 1, 46, 98, 99
        ],
        6
      ),
      999
    );
    assert_eq!(
      run_program(
        &mut vec![
          3, 21, 1008, 21, 8, 20, 1005, 20, 22, 107, 8, 21, 20, 1006, 20, 31, 1106, 0, 36, 98, 0,
          0, 1002, 21, 125, 20, 4, 20, 1105, 1, 46, 104, 999, 1105, 1, 46, 1101, 1000, 1, 20, 4,
          20, 1105, 1, 46, 98, 99
        ],
        8
      ),
      1000
    );
    assert_eq!(
      run_program(
        &mut vec![
          3, 21, 1008, 21, 8, 20, 1005, 20, 22, 107, 8, 21, 20, 1006, 20, 31, 1106, 0, 36, 98, 0,
          0, 1002, 21, 125, 20, 4, 20, 1105, 1, 46, 104, 999, 1105, 1, 46, 1101, 1000, 1, 20, 4,
          20, 1105, 1, 46, 98, 99
        ],
        13
      ),
      1001
    );
  }
}
