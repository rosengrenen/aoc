use aoc_util::{Solver, SolverOutput};

#[derive(Default)]
pub struct Day8;

impl Solver for Day8 {
  fn part_one(&self, input: &str) -> SolverOutput {
    let program = parse_program(input);
    let (accumulator, _) = run_program(&program);
    SolverOutput::Num(accumulator)
  }

  fn part_two(&self, input: &str) -> SolverOutput {
    let mut program = parse_program(input);
    for i in 0..program.len() {
      program[i] = match program[i] {
        Instruction::Jmp(value) => Instruction::Nop(value),
        Instruction::Nop(value) => Instruction::Jmp(value),
        instruction => instruction,
      };
      let (acc, exited) = run_program(&program);
      program[i] = match program[i] {
        Instruction::Jmp(value) => Instruction::Nop(value),
        Instruction::Nop(value) => Instruction::Jmp(value),
        instruction => instruction,
      };

      if exited {
        return SolverOutput::Num(acc);
      }
    }

    panic!("Could not find a program that terminates");
  }
}

#[derive(Copy, Clone)]
enum Instruction {
  Acc(i64),
  Jmp(i64),
  Nop(i64),
}

fn parse_program(input: &str) -> Vec<Instruction> {
  input
    .lines()
    .map(|line| {
      let (instruction, value) = line.split_once(' ').unwrap();
      match instruction {
        "acc" => Instruction::Acc(value.parse().unwrap()),
        "jmp" => Instruction::Jmp(value.parse().unwrap()),
        "nop" => Instruction::Nop(value.parse().unwrap()),
        _ => panic!("Invalid instruction {}", instruction),
      }
    })
    .collect()
}

fn has_bit(bit_set: &[i64], bit_index: usize) -> bool {
  bit_set[bit_index / 64] & 1 << (bit_index % 64) != 0
}

fn set_bit(bit_set: &mut [i64], bit_index: usize) {
  bit_set[bit_index / 64] |= 1 << (bit_index % 64);
}

// Returns the accumulator and whether the program exited or not
fn run_program(program: &[Instruction]) -> (i64, bool) {
  let mut accumulator = 0;
  let mut run_instructions = [0i64; 16];
  let mut instruction_pointer: isize = 0;

  let program_length = program.len() as isize;
  while instruction_pointer < program_length {
    if has_bit(&run_instructions, instruction_pointer as usize) {
      return (accumulator, false);
    }

    set_bit(&mut run_instructions, instruction_pointer as usize);

    match program[instruction_pointer as usize] {
      Instruction::Acc(value) => accumulator += value,
      Instruction::Jmp(value) => {
        instruction_pointer += value as isize;
        continue;
      }
      Instruction::Nop(_) => (),
    }

    instruction_pointer += 1;
  }

  (accumulator, true)
}
