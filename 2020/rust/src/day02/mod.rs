use crate::lib::Solver;

pub struct Day2Solver;

impl Solver for Day2Solver {
	fn solve_part_one(&self, input: &str) -> i64 {
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
		count
	}

	fn solve_part_two(&self, input: &str) -> i64 {
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
		count
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

#[cfg(test)]
mod tests {
	use super::*;
	use crate::lib::fetch_input;
	use test::Bencher;

	#[test]
	fn part_one_test_cases() {
		let input = include_str!("input.test1.txt");

		let solver: Day2Solver = Day2Solver {};
		assert_eq!(solver.solve_part_one(input), 2);
	}

	#[test]
	fn part_two_test_cases() {
		let input = include_str!("input.test1.txt");

		let solver: Day2Solver = Day2Solver {};
		assert_eq!(solver.solve_part_two(input), 1);
	}

	#[bench]
	fn bench_part_one(bencher: &mut Bencher) {
		let input = fetch_input(2);
		let solver = Day2Solver {};
		bencher.iter(|| solver.solve_part_one(&input));
	}

	#[bench]
	fn bench_part_two(bencher: &mut Bencher) {
		let input = fetch_input(2);
		let solver = Day2Solver {};
		bencher.iter(|| solver.solve_part_two(&input));
	}
}
