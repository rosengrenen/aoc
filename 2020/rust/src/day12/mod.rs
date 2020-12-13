use crate::lib::Solver;

pub struct Day12Solver;

impl Solver for Day12Solver {
	fn solve_part_one(&self, input: &str) -> i64 {
		let instructions = parse_instructions(input);
		let mut facing = Point { x: 1, y: 0 };
		let mut position = Point { x: 0, y: 0 };
		for &Instruction { action, value } in instructions.iter() {
			match action {
				b'N' => position.y += value,
				b'E' => position.x += value,
				b'S' => position.y -= value,
				b'W' => position.x -= value,
				b'R' => rotate_clockwise(&mut facing, value),
				b'L' => rotate_anticlockwise(&mut facing, value),
				b'F' => {
					position.x += facing.x * value;
					position.y += facing.y * value
				}
				_ => panic!("Invalid action"),
			}
		}
		position.x.abs() + position.y.abs()
	}

	fn solve_part_two(&self, input: &str) -> i64 {
		let instructions = parse_instructions(input);
		let mut waypoint = Point { x: 10, y: 1 };
		let mut position = Point { x: 0, y: 0 };
		for &Instruction { action, value } in instructions.iter() {
			match action {
				b'N' => waypoint.y += value,
				b'E' => waypoint.x += value,
				b'S' => waypoint.y -= value,
				b'W' => waypoint.x -= value,
				b'R' => rotate_clockwise(&mut waypoint, value),
				b'L' => rotate_anticlockwise(&mut waypoint, value),
				b'F' => {
					position.x += waypoint.x * value;
					position.y += waypoint.y * value
				}
				_ => panic!("Invalid action"),
			}
		}
		position.x.abs() + position.y.abs()
	}
}

struct Instruction {
	action: u8,
	value: i64,
}

fn parse_instructions(input: &str) -> Vec<Instruction> {
	input
		.lines()
		.map(|line| {
			let (action, value) = line.split_at(1);
			Instruction {
				action: action.as_bytes()[0],
				value: value.parse().unwrap(),
			}
		})
		.collect()
}

struct Point {
	x: i64,
	y: i64,
}

fn rotate_clockwise(point: &mut Point, degrees: i64) {
	for _ in 0..(degrees / 90) {
		let tmp = point.x;
		point.x = point.y;
		point.y = -tmp;
	}
}

fn rotate_anticlockwise(point: &mut Point, degrees: i64) {
	for _ in 0..(degrees / 90) {
		let tmp = point.x;
		point.x = -point.y;
		point.y = tmp;
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::lib::fetch_input;
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
		let input = fetch_input(12);
		bencher.iter(|| parse_instructions(&input));
	}

	#[bench]
	fn bench_part_one(bencher: &mut Bencher) {
		let input = fetch_input(12);
		let solver = Day12Solver {};
		bencher.iter(|| solver.solve_part_one(&input));
	}

	#[bench]
	fn bench_part_two(bencher: &mut Bencher) {
		let input = fetch_input(12);
		let solver = Day12Solver {};
		bencher.iter(|| solver.solve_part_two(&input));
	}
}
