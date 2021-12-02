use aoc_util::{Solver, SolverOutput};

#[derive(Default)]
pub struct Day2;

impl Solver for Day2 {
	fn part_one(&self, input: &str) -> SolverOutput {
		let commands = parse_commands(input);
		let mut h_pos = 0;
		let mut v_pos = 0;
		for (dir, amount) in commands {
			match dir {
				Direction::Forward => h_pos += amount,
				Direction::Down => v_pos += amount,
				Direction::Up => v_pos -= amount,
			}
		}

		SolverOutput::Num(h_pos * v_pos)
	}

	fn part_two(&self, input: &str) -> SolverOutput {
		let commands = parse_commands(input);
		let mut h_pos = 0;
		let mut v_pos = 0;
		let mut aim = 0;
		for (dir, amount) in commands {
			match dir {
				Direction::Forward => {
					h_pos += amount;
					v_pos += aim * amount;
				}
				Direction::Down => aim += amount,
				Direction::Up => aim -= amount,
			}
		}

		SolverOutput::Num(h_pos * v_pos)
	}
}

enum Direction {
	Forward,
	Down,
	Up,
}

fn parse_commands(input: &str) -> Vec<(Direction, i64)> {
	input
		.lines()
		.map(|line| {
			let (dir, amount) = line.split_once(' ').unwrap();
			let dir = match dir {
				"forward" => Direction::Forward,
				"down" => Direction::Down,
				"up" => Direction::Up,
				_ => panic!("Unknown direction {}", dir),
			};
			(dir, amount.parse().unwrap())
		})
		.collect()
}

#[cfg(test)]
mod benches {
	use super::*;
	use aoc_util::get_input;
	use test::{black_box, Bencher};

	#[bench]
	fn parse(bencher: &mut Bencher) {
		let input = get_input(2021, 2).unwrap();
		bencher.iter(|| parse_commands(black_box(&input)));
	}
}
