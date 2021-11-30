use std::{cmp::Ordering, collections::HashMap};

use aoc_util::{Solver, SolverOutput};

#[derive(Default)]
pub struct Day13;

impl Solver for Day13 {
	fn part_one(&self, input: &str) -> SolverOutput {
		let program_orig: Vec<i64> = input
			.split(',')
			.map(|s| s.parse::<i64>().unwrap())
			.collect();
		let mut computer = IntcodeComputer::new(&program_orig);

		let mut screen: HashMap<Pos, Tile> = HashMap::new();
		loop {
			computer.run(true, |_| unimplemented!());
			if !computer.is_pending() {
				break;
			}
			let x = computer.get_output();
			computer.run(true, |_| unimplemented!());
			let y = computer.get_output();
			computer.run(true, |_| unimplemented!());
			let tile_id = computer.get_output();
			screen.insert(
				Pos { x, y },
				match tile_id {
					0 => Tile::Empty,
					1 => Tile::Wall,
					2 => Tile::Block,
					3 => Tile::Paddle,
					4 => Tile::Ball,
					_ => panic!("Invalid tile id"),
				},
			);
		}

		let block_tile_count = screen.iter().fold(0, |previous, (_, tile)| match tile {
			Tile::Block => previous + 1,
			_ => previous,
		});
		SolverOutput::Num(block_tile_count)
	}

	fn part_two(&self, input: &str) -> SolverOutput {
		let program_orig: Vec<i64> = input
			.split(',')
			.map(|s| s.parse::<i64>().unwrap())
			.collect();
		let mut computer = IntcodeComputer::new(&program_orig);

		computer.program[0] = 2;
		let mut screen: HashMap<Pos, Tile> = HashMap::new();
		let mut score = 0;
		let mut paddle_position = Pos { x: 0, y: 0 };
		let mut ball_position = Pos { x: 0, y: 0 };
		loop {
			let joystick = match paddle_position.x.cmp(&ball_position.x) {
				Ordering::Equal => Direction::Neutral,
				Ordering::Greater => Direction::Left,
				Ordering::Less => Direction::Right,
			};
			computer.run(true, |_| match joystick {
				Direction::Left => -1,
				Direction::Neutral => 0,
				Direction::Right => 1,
			});
			if !computer.is_pending() {
				break;
			}
			let x = computer.get_output();
			computer.run(true, |_| unimplemented!());
			let y = computer.get_output();
			computer.run(true, |_| unimplemented!());
			let tile_id = computer.get_output();

			match tile_id {
				0 => {
					screen.insert(Pos { x, y }, Tile::Empty);
				}
				1 => {
					screen.insert(Pos { x, y }, Tile::Wall);
				}
				2 => {
					screen.insert(Pos { x, y }, Tile::Block);
				}
				3 => {
					paddle_position = Pos { x, y };
					screen.insert(paddle_position, Tile::Paddle);
				}
				4 => {
					ball_position = Pos { x, y };
					screen.insert(ball_position, Tile::Ball);
				}
				n => {
					score = n;
				}
			};
		}

		SolverOutput::Num(score)
	}
}

enum Direction {
	Left,
	Neutral,
	Right,
}

enum Tile {
	Empty,
	Wall,
	Block,
	Paddle,
	Ball,
}

#[derive(Clone, Copy, Eq, Hash, PartialEq)]
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
