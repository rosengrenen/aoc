use std::collections::HashMap;

use crate::lib::Solver;

pub struct Day15Solver;

impl Solver for Day15Solver {
	fn solve_part_one(&self, input: &str) -> i64 {
		let input = parse_numbers(input);
		let mut map: HashMap<i64, (i64, Option<i64>)> = HashMap::new();
		let mut last_num = 0;
		for (i, &num) in input.iter().enumerate() {
			map.insert(num, (i as i64 + 1, None));
			last_num = num;
		}

		let mut i = input.len() as i64 + 1;
		loop {
			let next_num = if let Some(&(prev, prev_prev)) = map.get(&last_num) {
				if let Some(prev_prev) = prev_prev {
					prev - prev_prev
				} else {
					0
				}
			} else {
				unreachable!();
			};

			if i == 2020 {
				return next_num;
			}

			let entry = map.entry(next_num).or_insert((i, None));
			*entry = (i, Some(entry.0));
			last_num = next_num;
			i += 1;
		}
	}

	fn solve_part_two(&self, input: &str) -> i64 {
		let input = parse_numbers(input);
		let mut map: HashMap<i64, (i64, Option<i64>)> = HashMap::new();
		let mut last_num = 0;
		for (i, &num) in input.iter().enumerate() {
			map.insert(num, (i as i64 + 1, None));
			last_num = num;
		}

		let mut i = input.len() as i64 + 1;
		loop {
			let next_num = if let Some(&(prev, prev_prev)) = map.get(&last_num) {
				if let Some(prev_prev) = prev_prev {
					prev - prev_prev
				} else {
					0
				}
			} else {
				unreachable!();
			};

			if i == 30_000_000 {
				return next_num;
			}

			let entry = map.entry(next_num).or_insert((i, None));
			*entry = (i, Some(entry.0));
			last_num = next_num;
			i += 1;
		}
	}
}

fn parse_numbers(input: &str) -> Vec<i64> {
	input
		.trim()
		.split(',')
		.map(|c| c.parse().unwrap())
		.collect()
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::lib::fetch_input;
	use test::Bencher;

	#[test]
	fn part_one_test_cases() {
		let solver = Day15Solver {};
		assert_eq!(solver.solve_part_one(&"1,3,2"), 1);
		assert_eq!(solver.solve_part_one(&"2,1,3"), 10);
		assert_eq!(solver.solve_part_one(&"1,2,3"), 27);
		assert_eq!(solver.solve_part_one(&"2,3,1"), 78);
		assert_eq!(solver.solve_part_one(&"3,2,1"), 438);
		assert_eq!(solver.solve_part_one(&"3,1,2"), 1836);
	}

	#[test]
	fn part_two_test_cases() {
		let solver = Day15Solver {};
		assert_eq!(solver.solve_part_two(&"0,3,6"), 175594);
		assert_eq!(solver.solve_part_two(&"1,3,2"), 2578);
		assert_eq!(solver.solve_part_two(&"2,1,3"), 3544142);
		assert_eq!(solver.solve_part_two(&"1,2,3"), 261214);
		assert_eq!(solver.solve_part_two(&"2,3,1"), 6895259);
		assert_eq!(solver.solve_part_two(&"3,2,1"), 18);
		assert_eq!(solver.solve_part_two(&"3,1,2"), 362);
	}

	#[bench]
	fn bench_parse_numbers(bencher: &mut Bencher) {
		let input = fetch_input(15);
		bencher.iter(|| parse_numbers(&input));
	}

	#[bench]
	fn bench_part_one(bencher: &mut Bencher) {
		let input = fetch_input(15);
		let solver = Day15Solver {};
		bencher.iter(|| solver.solve_part_one(&input));
	}

	#[bench]
	fn bench_part_two(bencher: &mut Bencher) {
		let input = fetch_input(15);
		let solver = Day15Solver {};
		bencher.iter(|| solver.solve_part_two(&input));
	}
}
