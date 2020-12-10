use crate::lib::{Solver, SolverResult};
use std::collections::HashMap;

pub struct Day11Solver;

impl Solver for Day11Solver {
	fn solve_part_one(&self, input: &str) -> SolverResult {
		let program_orig: Vec<i64> = input.split(',').map(|s| s.parse().unwrap()).collect();

		let mut canvas: HashMap<Pos, (Colour, i64)> = HashMap::new();
		let mut robot_direction = Direction::Up;
		let mut robot_position = Pos { x: 2, y: 2 };
		let mut computer = IntcodeComputer::new(&program_orig);
		loop {
			computer.run(true, |_| match canvas.get(&robot_position) {
				Some((colour, _)) => match colour {
					Colour::Black => 0,
					Colour::White => 1,
				},
				None => 0,
			});
			if !computer.is_pending() {
				break;
			}
			match computer.get_output() {
				0 => {
					canvas
						.entry(robot_position)
						.and_modify(|(colour, times_painted)| {
							*colour = Colour::Black;
							*times_painted += 1;
						})
						.or_insert((Colour::Black, 1));
				}
				1 => {
					canvas
						.entry(robot_position)
						.and_modify(|(colour, times_painted)| {
							*colour = Colour::White;
							*times_painted += 1;
						})
						.or_insert((Colour::White, 1));
				}
				n => panic!("Invalid colour: {}", n),
			}
			computer.run(true, |_| unimplemented!());
			match computer.get_output() {
				0 => robot_direction = robot_direction.turn_left(),
				1 => robot_direction = robot_direction.turn_right(),
				n => panic!("Invalid direction: {}", n),
			}
			match robot_direction {
				Direction::Up => robot_position.y -= 1,
				Direction::Right => robot_position.x += 1,
				Direction::Down => robot_position.y += 1,
				Direction::Left => robot_position.x -= 1,
			}
		}
		SolverResult::Num(canvas.len() as i64)
	}

	fn solve_part_two(&self, input: &str) -> crate::lib::SolverResult {
		let program_orig: Vec<i64> = input
			.split(',')
			.map(|s| s.parse::<i64>().unwrap())
			.collect();
		let mut canvas: HashMap<Pos, (Colour, i64)> = HashMap::new();
		let mut robot_direction = Direction::Up;
		let mut robot_position = Pos { x: 0, y: 0 };
		let mut computer = IntcodeComputer::new(&program_orig);
		loop {
			computer.run(true, |input_index| match canvas.get(&robot_position) {
				Some((colour, _)) => match colour {
					Colour::Black => 0,
					Colour::White => 1,
				},
				None => {
					if input_index == 0 {
						1
					} else {
						0
					}
				}
			});
			if !computer.is_pending() {
				break;
			}
			match computer.get_output() {
				0 => {
					canvas
						.entry(robot_position)
						.and_modify(|(colour, times_painted)| {
							*colour = Colour::Black;
							*times_painted += 1;
						})
						.or_insert((Colour::Black, 1));
				}
				1 => {
					canvas
						.entry(robot_position)
						.and_modify(|(colour, times_painted)| {
							*colour = Colour::White;
							*times_painted += 1;
						})
						.or_insert((Colour::White, 1));
				}
				n => panic!("Invalid colour: {}", n),
			}
			computer.run(true, |_| unimplemented!());
			match computer.get_output() {
				0 => robot_direction = robot_direction.turn_left(),
				1 => robot_direction = robot_direction.turn_right(),
				n => panic!("Invalid direction: {}", n),
			}
			match robot_direction {
				Direction::Up => robot_position.y -= 1,
				Direction::Right => robot_position.x += 1,
				Direction::Down => robot_position.y += 1,
				Direction::Left => robot_position.x -= 1,
			}
		}
		let mut top_left = Pos {
			x: std::i64::MAX,
			y: std::i64::MAX,
		};
		let mut bottom_right = Pos {
			x: std::i64::MIN,
			y: std::i64::MIN,
		};
		for (pos, _) in canvas.iter() {
			if pos.x < top_left.x {
				top_left.x = pos.x;
			}
			if pos.y < top_left.y {
				top_left.y = pos.y;
			}

			if pos.x > bottom_right.x {
				bottom_right.x = pos.x;
			}
			if pos.y > bottom_right.y {
				bottom_right.y = pos.y;
			}
		}
		let mut reg_id = String::from("\n");
		for y in 0..=bottom_right.y - top_left.y {
			for x in 0..=bottom_right.x - top_left.x {
				match canvas.get(&Pos {
					x: x + top_left.x,
					y: y + top_left.y,
				}) {
					Some((colour, _)) => match colour {
						Colour::Black => reg_id.push(' '),
						Colour::White => reg_id.push('O'),
					},
					None => reg_id.push(' '),
				}
			}
			reg_id.push('\n');
		}
		SolverResult::Text(reg_id)
	}
}

enum Direction {
	Up,
	Right,
	Down,
	Left,
}

impl Direction {
	fn turn_left(&self) -> Direction {
		match self {
			Direction::Up => Direction::Left,
			Direction::Right => Direction::Up,
			Direction::Down => Direction::Right,
			Direction::Left => Direction::Down,
		}
	}

	fn turn_right(&self) -> Direction {
		match self {
			Direction::Up => Direction::Right,
			Direction::Right => Direction::Down,
			Direction::Down => Direction::Left,
			Direction::Left => Direction::Up,
		}
	}
}

#[derive(Clone, Copy, Debug)]
enum Colour {
	White,
	Black,
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
struct Pos {
	x: i64,
	y: i64,
}

enum State {
	Pending(bool),
	Halted,
}

struct IntcodeComputer {
	instruction_pointer: usize,
	output_register: i64,
	program: Vec<i64>,
	input_index: usize,
	state: State,
	relative_base: i64,
}

impl IntcodeComputer {
	fn new(program: &Vec<i64>) -> IntcodeComputer {
		let mut program = program.clone();
		program.append(&mut vec![0; 1000]);
		IntcodeComputer {
			instruction_pointer: 0,
			output_register: 0,
			program,
			input_index: 0,
			state: State::Pending(false),
			relative_base: 0,
		}
	}

	fn get_n_args(&self, n: usize) -> Vec<(i64, i64)> {
		let mut shift = 100;
		let mut args = Vec::new();
		for offset in 1..=n {
			let mode = (self.program[self.instruction_pointer] / shift) % 10;
			let value = self.program[self.instruction_pointer + offset];
			match mode {
				0 => args.push((value, self.program[value as usize])),
				1 => args.push((value, value)),
				2 => args.push((
					self.relative_base + value,
					self.program[(self.relative_base + value) as usize],
				)),
				_ => panic!("Invalid mode: {}", mode),
			};
			shift *= 10;
		}
		args
	}

	fn get_output(&self) -> i64 {
		self.output_register
	}

	fn is_pending(&self) -> bool {
		match self.state {
			State::Halted => false,
			_ => true,
		}
	}

	fn run<F>(&mut self, pause_on_output: bool, input: F)
	where
		F: Fn(usize) -> i64,
	{
		loop {
			self.step(&input);
			match self.state {
				State::Halted => return,
				State::Pending(true) => {
					if pause_on_output {
						return;
					}
				}
				State::Pending(false) => (),
			}
		}
	}

	fn step<F>(&mut self, input: &F)
	where
		F: Fn(usize) -> i64,
	{
		let instruction_options = self.program[self.instruction_pointer];
		let op_code = instruction_options % 100;

		if op_code == 99 {
			self.state = State::Halted;
			return;
		}

		match op_code {
			1 => {
				let args = self.get_n_args(3);
				self.program[args[2].0 as usize] = args[0].1 + args[1].1;
				self.instruction_pointer += 4;
				self.state = State::Pending(false);
			}
			2 => {
				let args = self.get_n_args(3);
				self.program[args[2].0 as usize] = args[0].1 * args[1].1;
				self.instruction_pointer += 4;
				self.state = State::Pending(false);
			}
			3 => {
				let args = self.get_n_args(1);
				self.program[args[0].0 as usize] = input(self.input_index);
				self.input_index += 1;
				self.instruction_pointer += 2;
				self.state = State::Pending(false);
			}
			4 => {
				let args = self.get_n_args(1);
				self.output_register = args[0].1;
				self.instruction_pointer += 2;
				self.state = State::Pending(true);
			}
			5 => {
				let args = self.get_n_args(2);
				if args[0].1 != 0 {
					self.instruction_pointer = args[1].1 as usize;
				} else {
					self.instruction_pointer += 3;
				}
				self.state = State::Pending(false);
			}
			6 => {
				let args = self.get_n_args(2);
				if args[0].1 == 0 {
					self.instruction_pointer = args[1].1 as usize;
				} else {
					self.instruction_pointer += 3;
				}
				self.state = State::Pending(false);
			}
			7 => {
				let args = self.get_n_args(3);
				self.program[args[2].0 as usize] = if args[0].1 < args[1].1 { 1 } else { 0 };
				self.instruction_pointer += 4;
				self.state = State::Pending(false);
			}
			8 => {
				let args = self.get_n_args(3);
				self.program[args[2].0 as usize] = if args[0].1 == args[1].1 { 1 } else { 0 };
				self.instruction_pointer += 4;
				self.state = State::Pending(false);
			}
			9 => {
				let args = self.get_n_args(1);
				self.relative_base += args[0].1;
				self.instruction_pointer += 2;
				self.state = State::Pending(false);
			}
			_ => panic!(
				"Invalid op code: {} at {}",
				op_code, self.instruction_pointer
			),
		};
	}
}
