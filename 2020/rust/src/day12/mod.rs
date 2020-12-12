use crate::lib::Solver;

pub struct Day12Solver;

impl Solver for Day12Solver {
	fn solve_part_one(&self, input: &str) -> i64 {
		let instructions = parse_instructions(input);
		let mut facing = (1, 0);
		let mut position = (0, 0);
		for (instruction, value) in instructions.iter() {
			match instruction {
				b'N' => position.1 += value,
				b'E' => position.0 += value,
				b'S' => position.1 -= value,
				b'W' => position.0 -= value,
				b'R' => {
					for _ in 0..(value / 90) {
						let tmp = facing.1;
						facing.1 = -facing.0;
						facing.0 = tmp;
					}
				}
				b'L' => {
					for _ in 0..(value / 90) * 3 {
						let tmp = facing.1;
						facing.1 = -facing.0;
						facing.0 = tmp;
					}
				}
				b'F' => {
					position.0 += facing.0 * value;
					position.1 += facing.1 * value
				}
				_ => panic!(),
			}
		}
		position.0.abs() + position.1.abs()
	}

	fn solve_part_two(&self, input: &str) -> i64 {
		let instructions = parse_instructions(input);
		let mut waypoint = (10, 1);
		let mut position = (0, 0);
		for (instruction, value) in instructions.iter() {
			match instruction {
				b'N' => waypoint.1 += value,
				b'E' => waypoint.0 += value,
				b'S' => waypoint.1 -= value,
				b'W' => waypoint.0 -= value,
				b'R' => {
					for _ in 0..(value / 90) {
						let tmp = waypoint.1;
						waypoint.1 = -waypoint.0;
						waypoint.0 = tmp;
					}
				}
				b'L' => {
					for _ in 0..(value / 90) * 3 {
						let tmp = waypoint.1;
						waypoint.1 = -waypoint.0;
						waypoint.0 = tmp;
					}
				}
				b'F' => {
					position.0 += waypoint.0 * value;
					position.1 += waypoint.1 * value
				}
				_ => panic!(),
			}
		}
		position.0.abs() + position.1.abs()
	}
}

fn parse_instructions(input: &str) -> Vec<(u8, i64)> {
	input
		.lines()
		.map(|line| {
			let (instruction, value) = line.split_at(1);
			(instruction.as_bytes()[0], value.parse().unwrap())
		})
		.collect()
}

#[cfg(test)]
mod tests {
	use super::*;
	use test::Bencher;

	#[test]
	fn part_one_test_cases() {
		let input = include_str!("input.test1.txt");
		let solver = Day12Solver {};
		assert_eq!(solver.solve_part_one(input), 25);
	}

	#[test]
	fn part_two_test_cases() {
		let input = include_str!("input.test1.txt");
		let solver = Day12Solver {};
		assert_eq!(solver.solve_part_two(input), 286);
	}

	#[bench]
	fn bench_parse_instructions(bencher: &mut Bencher) {
		let input = include_str!("input.txt");
		bencher.iter(|| parse_instructions(input));
	}

	#[bench]
	fn bench_part_one(bencher: &mut Bencher) {
		let input = include_str!("input.txt");
		let solver = Day12Solver {};
		bencher.iter(|| solver.solve_part_one(input));
	}

	#[bench]
	fn bench_part_two(bencher: &mut Bencher) {
		let input = include_str!("input.txt");
		let solver = Day12Solver {};
		bencher.iter(|| solver.solve_part_two(input));
	}
}
