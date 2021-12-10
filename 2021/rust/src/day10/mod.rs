use aoc_util::{Solver, SolverOutput};

#[derive(Default)]
pub struct Day10;

impl Solver for Day10 {
	fn part_one(&self, input: &str) -> SolverOutput {
		let a = parse_(input);

		SolverOutput::Num(a.iter().fold(0, |prev, c| match check_line(c) {
			LineState::Complete => prev,
			LineState::Incomplete(_) => prev,
			LineState::Corrupted(b) => match b {
				b')' => prev + 3,
				b']' => prev + 57,
				b'}' => prev + 1197,
				b'>' => prev + 25137,
				_ => prev,
			},
		}))
	}

	fn part_two(&self, input: &str) -> SolverOutput {
		let a = parse_(input);

		let mut r = a
			.iter()
			.filter_map(|c| match check_line(c) {
				LineState::Complete => None,
				LineState::Incomplete(closing) => Some({
					closing.iter().fold(0, |p, &c| match c {
						b')' => 5 * p + 1,
						b']' => 5 * p + 2,
						b'}' => 5 * p + 3,
						b'>' => 5 * p + 4,
						_ => p,
					})
				}),
				LineState::Corrupted(_) => None,
			})
			.collect::<Vec<_>>();
		r.sort();
		SolverOutput::Num(r[r.len() / 2])
	}
}

#[derive(Debug)]
enum LineState {
	Complete,
	Incomplete(Vec<u8>),
	Corrupted(u8),
}

fn check_line(line: &Vec<u8>) -> LineState {
	let mut stack = vec![];

	for &b in line {
		match b {
			b'(' => stack.push(b),
			b'[' => stack.push(b),
			b'{' => stack.push(b),
			b'<' => stack.push(b),

			b')' => {
				if *stack.last().unwrap() == b'(' {
					stack.pop();
				} else {
					return LineState::Corrupted(b);
				}
			}
			b']' => {
				if *stack.last().unwrap() == b'[' {
					stack.pop();
				} else {
					return LineState::Corrupted(b);
				}
			}
			b'}' => {
				if *stack.last().unwrap() == b'{' {
					stack.pop();
				} else {
					return LineState::Corrupted(b);
				}
			}
			b'>' => {
				if *stack.last().unwrap() == b'<' {
					stack.pop();
				} else {
					return LineState::Corrupted(b);
				}
			}
			_ => panic!(),
		}
	}

	if stack.is_empty() {
		LineState::Complete
	} else {
		let mut closing = vec![];
		for &s in stack.iter().rev() {
			match s {
				b'(' => closing.push(b')'),
				b'[' => closing.push(b']'),
				b'{' => closing.push(b'}'),
				b'<' => closing.push(b'>'),
				_ => panic!(),
			}
		}
		LineState::Incomplete(closing)
	}
}

fn parse_(input: &str) -> Vec<Vec<u8>> {
	input
		.lines()
		.map(|line| line.as_bytes().into_iter().cloned().collect())
		.collect()
}

#[cfg(test)]
mod benches {
	use super::*;
	use aoc_util::get_input;
	use test::{black_box, Bencher};

	#[bench]
	fn parse(bencher: &mut Bencher) {
		let input = get_input(2021, 9).unwrap();
		bencher.iter(|| parse_(black_box(&input)));
	}
}
