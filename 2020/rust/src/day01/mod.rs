use crate::lib::Solver;
use std::cmp::Ordering;

pub struct Day1Solver;

impl Solver for Day1Solver {
	fn solve_part_one(&self, input: &str) -> i64 {
		let mut numbers = parse_numbers(input);
		numbers.sort_unstable();
		if let Some((first, second)) = sum_in_list(&numbers, 2020) {
			first * second
		} else {
			panic!("Could not find an answer");
		}
	}

	fn solve_part_two(&self, input: &str) -> i64 {
		let mut numbers = parse_numbers(input);
		numbers.sort_unstable();
		for &first in numbers.iter() {
			if let Some((second, third)) = sum_in_list(&numbers, 2020 - first) {
				return first * second * third;
			}
		}

		panic!("Could not find an answer");
	}
}

fn parse_numbers(input: &str) -> Vec<i64> {
	input
		.lines()
		.map(|line| line.parse::<i64>().unwrap())
		.collect()
}

fn sum_in_list(list: &[i64], target: i64) -> Option<(i64, i64)> {
	let mut lower_index = 0;
	let mut upper_index = list.len() - 1;
	while lower_index < upper_index {
		let sum = list[lower_index] + list[upper_index];
		match sum.cmp(&target) {
			Ordering::Equal => return Some((list[lower_index], list[upper_index])),
			Ordering::Greater => upper_index -= 1,
			Ordering::Less => lower_index += 1,
		}
	}

	None
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::lib::fetch_input;
	use test::Bencher;

	#[test]
	fn part_one_test_cases() {
		let input = include_str!("input.test1.txt");
		let solver: Day1Solver = Day1Solver {};
		assert_eq!(solver.solve_part_one(input), 514579);
	}

	#[test]
	fn part_two_test_cases() {
		let input = include_str!("input.test1.txt");
		let solver: Day1Solver = Day1Solver {};
		assert_eq!(solver.solve_part_two(input), 241861950);
	}

	#[bench]
	fn bench_parse_numbers(bencher: &mut Bencher) {
		let input = fetch_input(1);
		bencher.iter(|| parse_numbers(&input));
	}

	#[bench]
	fn bench_part_one(bencher: &mut Bencher) {
		let input = fetch_input(1);
		let solver = Day1Solver {};
		bencher.iter(|| solver.solve_part_one(&input));
	}

	#[bench]
	fn bench_part_two(bencher: &mut Bencher) {
		let input = fetch_input(1);
		let solver = Day1Solver {};
		bencher.iter(|| solver.solve_part_two(&input));
	}
}
