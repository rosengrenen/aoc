use crate::lib::Solver;

pub struct Day8Solver;

impl Solver for Day8Solver {
	fn solve_part_one(&self, input: &str) -> i64 {
		let program = parse_program(input);
		let (accumulator, _) = run_program(&program);
		accumulator
	}

	fn solve_part_two(&self, input: &str) -> i64 {
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
				return acc;
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
				_ => panic!(format!("Invalid instruction {}", instruction)),
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

#[cfg(test)]
mod tests {
	use super::*;
	use crate::lib::fetch_input;
	use test::Bencher;

	#[test]
	fn part_one_test_cases() {
		let input = include_str!("input.test1.txt");
		let solver = Day8Solver {};
		assert_eq!(solver.solve_part_one(input), 5);
	}

	#[test]
	fn part_two_test_cases() {
		let input = include_str!("input.test1.txt");
		let solver = Day8Solver {};
		assert_eq!(solver.solve_part_two(input), 8);
	}

	#[bench]
	fn bench_parse_instructions(bencher: &mut Bencher) {
		let input = fetch_input(8);
		bencher.iter(|| parse_program(&input));
	}

	#[bench]
	fn bench_part_one(bencher: &mut Bencher) {
		let input = fetch_input(8);
		let solver = Day8Solver {};
		bencher.iter(|| solver.solve_part_one(&input));
	}

	#[bench]
	fn bench_part_two(bencher: &mut Bencher) {
		let input = fetch_input(8);
		let solver = Day8Solver {};
		bencher.iter(|| solver.solve_part_two(&input));
	}
}
