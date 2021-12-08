use aoc_util::{Solver, SolverOutput};

#[derive(Default)]
pub struct Day7;

impl Solver for Day7 {
	fn part_one(&self, input: &str) -> SolverOutput {
		let crabs = parse_crabs(input);
		let median = median(&crabs);
		SolverOutput::Num(crabs.iter().map(|pos| (median - pos).abs()).sum())
	}

	fn part_two(&self, input: &str) -> SolverOutput {
		let crabs = parse_crabs(input);
		let average = average(&crabs);
		SolverOutput::Num(
			crabs
				.iter()
				.map(|pos| (average - pos).abs())
				.map(|d| (d * (d + 1)) / 2)
				.sum::<i64>()
				.min(
					crabs
						.iter()
						.map(|pos| (average - pos).abs() + 1)
						.map(|d| (d * (d + 1)) / 2)
						.sum(),
				),
		)
	}
}

fn average(nums: &[i64]) -> i64 {
	nums.iter().sum::<i64>() / nums.len() as i64
}

fn median(nums: &[i64]) -> i64 {
	let len = nums.len();
	if len % 2 == 0 {
		let middle = len / 2;
		(nums[middle] + nums[middle - 1]) / 2
	} else {
		nums[len]
	}
}

fn parse_crabs(input: &str) -> Vec<i64> {
	let mut crabs: Vec<_> = input.split(",").map(|num| num.parse().unwrap()).collect();
	crabs.sort_unstable();
	crabs
}

#[cfg(test)]
mod benches {
	use super::*;
	use aoc_util::get_input;
	use test::{black_box, Bencher};

	#[bench]
	fn parse(bencher: &mut Bencher) {
		let input = get_input(2021, 7).unwrap();
		bencher.iter(|| parse_crabs(black_box(&input)));
	}
}
