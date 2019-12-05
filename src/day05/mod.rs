use crate::lib::Solver;

pub struct Day5Solver;

impl Solver for Day5Solver {
  fn solve(&self, lines: Vec<String>, part_two: bool) -> String {
    let program_raw = &lines[0];
    let program_orig: Vec<i32> = program_raw
      .split(',')
      .map(|s| s.parse::<i32>().unwrap())
      .collect();
    let mut program = program_orig.clone();
    if part_two {
      run_program(&mut program, 5)
    } else {
      run_program(&mut program, 1)
    }
    .to_string()
  }
}

fn get_value(program: &Vec<i32>, value: i32, mode: i32) -> i32 {
  match mode {
    0 => program[value as usize],
    1 => value,
    _ => panic!("Invalid mode: {}", mode),
  }
}

fn run_program(program: &mut Vec<i32>, input: i32) -> i32 {
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
        output = program[a_value as usize];
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
