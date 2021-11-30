use aoc_util::{Solver, SolverOutput};

#[derive(Default)]
pub struct Day2;

impl Solver for Day2 {
	fn part_one(&self, input: &str) -> SolverOutput {
		let mut count = 0;
		for policy in parse_input(input) {
			let mut occurences = 0;
			for &c in policy.password.as_bytes().iter() {
				if c == policy.pattern {
					occurences += 1;
				}

				if occurences > policy.max {
					break;
				}
			}

			if (policy.min..=policy.max).contains(&occurences) {
				count += 1;
			}
		}

		SolverOutput::Num(count)
	}

	fn part_two(&self, input: &str) -> SolverOutput {
		let mut count = 0;
		for policy in parse_input(input) {
			let mut occurences = 0;
			if policy.password.as_bytes()[policy.min - 1] == policy.pattern {
				occurences += 1;
			}
			if policy.password.as_bytes()[policy.max - 1] == policy.pattern {
				occurences += 1;
			}
			if occurences == 1 {
				count += 1;
			}
		}

		SolverOutput::Num(count)
	}
}

struct Policy<'a> {
	min: usize,
	max: usize,
	pattern: u8,
	password: &'a str,
}

fn parse_input(input: &str) -> impl Iterator<Item = Policy> {
	input.lines().map(|line| {
		let mut first_split = line.split_whitespace();
		let (min_raw, max_raw) = first_split.next().unwrap().split_once('-').unwrap();
		let pattern = first_split.next().unwrap().as_bytes()[0];
		let password = first_split.next().unwrap();
		Policy {
			min: min_raw.parse().unwrap(),
			max: max_raw.parse().unwrap(),
			pattern,
			password,
		}
	})
}
