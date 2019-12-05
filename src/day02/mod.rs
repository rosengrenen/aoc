use crate::lib::Solver;

pub struct Day2Solver;

impl Solver for Day2Solver {
  fn solve(&self, lines: Vec<String>, part_two: bool) -> String {
    let program_raw = &lines[0];
    let program_orig: Vec<i32> = program_raw
      .split(',')
      .map(|s| s.parse::<i32>().unwrap())
      .collect();
    if part_two {
      for noun in 0..=99 {
        for verb in 0..=99 {
          let mut program = program_orig.clone();
          if run_program(&mut program, noun, verb) == 19690720 {
            return (100 * noun + verb).to_string();
          }
        }
      }
      panic!("No solution found for part 2");
    } else {
      let mut program = program_orig.clone();
      run_program(&mut program, 12, 2).to_string()
    }
  }
}

fn run_program(program: &mut Vec<i32>, noun: i32, verb: i32) -> i32 {
  program[1] = noun;
  program[2] = verb;
  let mut instruction_pointer: usize = 0;
  loop {
    let op_code = program[instruction_pointer];

    if op_code == 99 {
      return program[0];
    }

    let a = program[program[instruction_pointer + 1] as usize];
    let b = program[program[instruction_pointer + 2] as usize];
    let c = program[instruction_pointer + 3];

    match op_code {
      1 => {
        program[c as usize] = a + b;
        instruction_pointer += 4;
      }
      2 => {
        program[c as usize] = a * b;
        instruction_pointer += 4;
      }
      _ => panic!("Invalid op code: {} at {}", op_code, instruction_pointer),
    };
  }
}
