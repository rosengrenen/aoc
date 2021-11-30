use aoc_util::{Solver, SolverOutput};

#[derive(Default)]
pub struct Day2;

impl Solver for Day2 {
	fn part_one(&self, input: &str) -> SolverOutput {
		let mut program: Vec<i64> = input.split(',').map(|s| s.parse().unwrap()).collect();
		let result = run_program(&mut program, 12, 2);
		SolverOutput::Num(result)
	}

	fn part_two(&self, input: &str) -> SolverOutput {
		let program_orig: Vec<i64> = input
			.split(',')
			.map(|s| s.parse::<i64>().unwrap())
			.collect();
		for noun in 0..=99 {
			for verb in 0..=99 {
				let mut program = program_orig.clone();
				if run_program(&mut program, noun, verb) == 19_690_720 {
					return SolverOutput::Num(100 * noun + verb);
				}
			}
		}

		panic!("No solution found for part 2");
	}
}

fn run_program(program: &mut Vec<i64>, noun: i64, verb: i64) -> i64 {
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

// #[cfg(test)]
// mod tests {
// 	use super::*;

// 	fn test_program(mut input: &mut Vec<i32>, expected_answer: &Vec<i32>) {
// 		let noun = input[1];
// 		let verb = input[2];
// 		run_program(&mut input, noun, verb);
// 		assert_eq!(input, expected_answer);
// 	}

// 	#[test]
// 	fn part_one_test_cases() {
// 		test_program(&mut vec![1, 0, 0, 0, 99], &mut vec![2, 0, 0, 0, 99]);
// 		test_program(&mut vec![2, 3, 0, 3, 99], &mut vec![2, 3, 0, 6, 99]);
// 		test_program(
// 			&mut vec![2, 4, 4, 5, 99, 0],
// 			&mut vec![2, 4, 4, 5, 99, 9801],
// 		);
// 		test_program(
// 			&mut vec![1, 1, 1, 4, 99, 5, 6, 0, 99],
// 			&mut vec![30, 1, 1, 4, 2, 5, 6, 0, 99],
// 		);
// 	}
// }
