use crate::lib::Solver;
use std::cmp::Ordering;

pub struct Day9Solver;

impl Solver for Day9Solver {
	fn solve_part_one(&self, input: &str) -> i64 {
		let numbers = parse_numbers(input);
		find_first_invalid(&numbers, 25)
	}

	fn solve_part_two(&self, input: &str) -> i64 {
		let numbers = parse_numbers(input);
		find_first_invalid_contiguous_min_max(&numbers, 25)
	}
}

fn parse_numbers(input: &str) -> Vec<i64> {
	input.lines().map(|line| line.parse().unwrap()).collect()
}

fn find_first_invalid(numbers: &[i64], preamble_length: usize) -> i64 {
	for i in preamble_length..numbers.len() {
		let current_number = numbers[i];
		if !has_sum(&numbers[(i - preamble_length)..i], current_number) {
			return current_number;
		}
	}

	panic!("No invalid number in sequence");
}

fn find_first_invalid_contiguous_min_max(numbers: &[i64], preamble_length: usize) -> i64 {
	for i in preamble_length..numbers.len() {
		let current_number = numbers[i];
		if !has_sum(&numbers[(i - preamble_length)..i], current_number) {
			let slice = find_contiguous_sum(&numbers[0..i as usize - 1], current_number);

			return slice.iter().min().unwrap() + slice.iter().max().unwrap();
		}
	}

	panic!("No invalid number in sequence");
}

fn has_sum(list: &[i64], target: i64) -> bool {
	for i in list {
		for j in list {
			if i != j && i + j == target {
				return true;
			}
		}
	}

	false
}

fn find_contiguous_sum(list: &[i64], target: i64) -> &[i64] {
	let mut lower_index = 0;
	let mut upper_index = 1;
	let mut partial_sums = Vec::with_capacity(list.len());
	partial_sums.push(list[0]);
	while upper_index < list.len() {
		if upper_index == partial_sums.len() {
			partial_sums.push(partial_sums[partial_sums.len() - 1] + list[upper_index]);
		}
		let current_range = &list[lower_index..=upper_index];
		let sum: i64 = partial_sums[upper_index] - partial_sums[lower_index];
		match sum.cmp(&target) {
			Ordering::Equal => return current_range,
			Ordering::Greater => {
				if upper_index - lower_index == 1 {
					upper_index += 1;
				} else {
					lower_index += 1;
				}
			}
			Ordering::Less => {
				upper_index += 1;
			}
		};
	}

	panic!("Could not find contiguous sum in list");
}

#[cfg(test)]
mod tests {
	use super::*;
	use test::Bencher;

	#[test]
	fn part_one_test_cases() {
		let input = include_str!("input.test1.txt");
		assert_eq!(find_first_invalid(&parse_numbers(input), 5), 127);
	}

	#[test]
	fn part_two_test_cases() {
		let input = include_str!("input.test1.txt");
		assert_eq!(
			find_first_invalid_contiguous_min_max(&parse_numbers(input), 5),
			62
		);
	}

	#[bench]
	fn bench_parse_numbers(bencher: &mut Bencher) {
		let input = include_str!("input.txt");
		bencher.iter(|| parse_numbers(input));
	}

	#[bench]
	fn bench_part_one(bencher: &mut Bencher) {
		let input = include_str!("input.txt");
		let solver = Day9Solver {};
		bencher.iter(|| solver.solve_part_one(input));
	}

	#[bench]
	fn bench_part_two(bencher: &mut Bencher) {
		let input = include_str!("input.txt");
		let solver = Day9Solver {};
		bencher.iter(|| solver.solve_part_two(input));
	}
}
