use crate::lib::{Solver, SolverResult};
use std::cmp;

pub struct Day1Solver;

impl Solver for Day1Solver {
	fn solve_part_one(&self, input: &str) -> SolverResult {
		let total_fuel_requirement =
			parse_numbers(input).fold(0, |total, weight| total + calc_req_fuel(weight));
		SolverResult::Num(total_fuel_requirement)
	}

	fn solve_part_two(&self, input: &str) -> SolverResult {
		let total_fuel_requirement =
			parse_numbers(input).fold(0, |total, weight| total + calc_req_fuel_inc_self(weight));
		SolverResult::Num(total_fuel_requirement)
	}
}

fn parse_numbers(input: &'_ str) -> impl Iterator<Item = i64> + '_ {
	input.lines().map(|line| line.parse::<i64>().unwrap())
}

fn calc_req_fuel(weight: i64) -> i64 {
	weight / 3 - 2
}

fn calc_req_fuel_inc_self(weight: i64) -> i64 {
	let cost = cmp::max(weight / 3 - 2, 0);
	if cost == 0 {
		return 0;
	}

	cost + calc_req_fuel_inc_self(cost)
}

#[cfg(test)]
mod tests {
	use super::*;
	use test::Bencher;

	#[test]
	fn part_one_test_cases() {
		let solver = Day1Solver {};
		assert_eq!(solver.solve_part_one("12"), SolverResult::Num(2));
		assert_eq!(solver.solve_part_one("14"), SolverResult::Num(2));
		assert_eq!(solver.solve_part_one("1969"), SolverResult::Num(654));
		assert_eq!(solver.solve_part_one("100756"), SolverResult::Num(33583));
	}

	#[test]
	fn part_two_test_cases() {
		let solver = Day1Solver {};
		assert_eq!(solver.solve_part_two("14"), SolverResult::Num(2));
		assert_eq!(solver.solve_part_two("1969"), SolverResult::Num(966));
		assert_eq!(solver.solve_part_two("100756"), SolverResult::Num(50346));
	}

	#[bench]
	fn bench_part_one(bencher: &mut Bencher) {
		let input = include_str!("input.txt");
		let solver = Day1Solver {};
		bencher.iter(|| solver.solve_part_one(input));
	}

	#[bench]
	fn bench_part_two(bencher: &mut Bencher) {
		let input = include_str!("input.txt");
		let solver = Day1Solver {};
		bencher.iter(|| solver.solve_part_two(input));
	}
}
