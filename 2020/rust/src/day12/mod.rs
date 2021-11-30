use aoc_util::{Solver, SolverOutput};

#[derive(Default)]
pub struct Day12;

impl Solver for Day12 {
	fn part_one(&self, input: &str) -> SolverOutput {
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

		SolverOutput::Num(position.x.abs() + position.y.abs())
	}

	fn part_two(&self, input: &str) -> SolverOutput {
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

		SolverOutput::Num(position.x.abs() + position.y.abs())
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
