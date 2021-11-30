use std::cmp;

use aoc_util::{Solver, SolverOutput};

#[derive(Default)]
pub struct Day1;

impl Solver for Day1 {
	fn part_one(&self, input: &str) -> SolverOutput {
		let sum = input
			.lines()
			.map(|s| s.parse().unwrap())
			.map(|w| calculate_fuel_cost(w, false))
			.sum();
		SolverOutput::Num(sum)
	}

	fn part_two(&self, input: &str) -> SolverOutput {
		let sum = input
			.lines()
			.map(|s| s.parse().unwrap())
			.map(|w| calculate_fuel_cost(w, true))
			.sum();
		SolverOutput::Num(sum)
	}
}

fn calculate_fuel_cost(weight: i64, part_two: bool) -> i64 {
	let cost = cmp::max(weight / 3 - 2, 0);
	if part_two && cost > 0 {
		cost + calculate_fuel_cost(cost, part_two)
	} else {
		cost
	}
}
