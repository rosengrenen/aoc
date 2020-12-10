use crate::lib::{Solver, SolverResult};

pub struct Day2Solver;

impl Solver for Day2Solver {
	fn solve_part_one(&self, input: &str) -> SolverResult {
		let mut program = parse_program(input);
		program.set_noun(12);
		program.set_verb(2);
		SolverResult::Num(program.run())
	}

	fn solve_part_two(&self, input: &str) -> SolverResult {
		let program = parse_program(input);
		for noun in 0..=99 {
			for verb in 0..=99 {
				let mut program = program.clone();
				program.set_noun(noun);
				program.set_verb(verb);
				if program.run() == 19_690_720 {
					return SolverResult::Num(100 * noun + verb);
				}
			}
		}

		panic!("No solution found for part 2");
	}
}

#[derive(Clone)]
struct IntcodeComputer {
	instructions: Vec<i64>,
	instruction_pointer: usize,
}

impl IntcodeComputer {
	fn new(instructions: Vec<i64>) -> IntcodeComputer {
		IntcodeComputer {
			instructions,
			instruction_pointer: 0,
		}
	}

	fn run(&mut self) -> i64 {
		loop {
			let op_code = self.op_code();
			if op_code == 99 {
				return self.instructions[0];
			}

			let arg0 = self.op_arg(0) as usize;
			let arg1 = self.op_arg(1) as usize;
			let arg2 = self.op_arg(2) as usize;

			match op_code {
				1 => {
					let sum = self.read_mem_value(arg0) + self.read_mem_value(arg1);
					self.write_mem_value(arg2, sum);
					self.instruction_pointer += 4;
				}
				2 => {
					let prod = self.read_mem_value(arg0) * self.read_mem_value(arg1);
					self.write_mem_value(arg2, prod);
					self.instruction_pointer += 4;
				}
				_ => panic!(
					"Invalid op code: {} at {}",
					op_code, self.instruction_pointer
				),
			};
		}
	}

	fn op_code(&self) -> i64 {
		self.read_mem_value(self.instruction_pointer)
	}

	fn op_arg(&self, arg: usize) -> i64 {
		self.read_mem_value(self.instruction_pointer + arg + 1)
	}

	fn read_mem_value(&self, address: usize) -> i64 {
		self.instructions[address]
	}

	fn write_mem_value(&mut self, address: usize, value: i64) {
		self.instructions[address] = value;
	}

	fn set_noun(&mut self, noun: i64) {
		self.instructions[1] = noun;
	}

	fn set_verb(&mut self, verb: i64) {
		self.instructions[2] = verb;
	}
}

fn parse_program(input: &str) -> IntcodeComputer {
	let instructions: Vec<i64> = input
		.split(',')
		.map(|raw_instruction| raw_instruction.parse().unwrap())
		.collect();
	IntcodeComputer::new(instructions)
}

#[cfg(test)]
mod tests {
	use super::*;
	use test::Bencher;

	#[test]
	fn part_one_test_cases() {
		let input = include_str!("input.test1.txt");
		let solver = Day2Solver {};
		assert_eq!(solver.solve_part_one(input), SolverResult::Num(3500));
	}

	#[bench]
	fn bench_part_one(bencher: &mut Bencher) {
		let input = include_str!("input.txt");
		let solver = Day2Solver {};
		bencher.iter(|| solver.solve_part_one(input));
	}

	#[bench]
	fn bench_part_two(bencher: &mut Bencher) {
		let input = include_str!("input.txt");
		let solver = Day2Solver {};
		bencher.iter(|| solver.solve_part_two(input));
	}
}
