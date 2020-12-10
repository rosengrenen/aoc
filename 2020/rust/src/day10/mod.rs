use crate::lib::Solver;
use std::collections::HashSet;

pub struct Day10Solver;

impl Solver for Day10Solver {
	fn solve_part_one(&self, input: &str) -> i64 {
		let mut output_joltages = parse_output_joltages(input);
		output_joltages.sort();
		let mut ones = 1;
		let mut threes = 1;
		let mut prev_output_joltage = output_joltages[0];
		for &output_joltage in &output_joltages[1..] {
			match output_joltage - prev_output_joltage {
				1 => ones += 1,
				3 => threes += 1,
				_ => (),
			}

			prev_output_joltage = output_joltage;
		}

		ones * threes
	}

	fn solve_part_two(&self, input: &str) -> i64 {
		let mut output_joltages = parse_output_joltages(input);
		output_joltages.push(0);
		output_joltages.sort();
		chunk_output_joltages(&output_joltages)
			.iter()
			.map(|part| permutations_in_part(part))
			.product()
	}
}

fn parse_output_joltages(input: &str) -> Vec<i64> {
	input
		.lines()
		.map(|line| line.parse::<i64>().unwrap())
		.collect()
}

fn chunk_output_joltages(output_joltages: &Vec<i64>) -> Vec<Vec<i64>> {
	let mut chunks = Vec::new();
	let mut current_chunk = Vec::new();
	for &output_joltage in output_joltages {
		if !current_chunk.is_empty() && output_joltage - current_chunk.last().unwrap() == 3 {
			chunks.push(current_chunk);
			current_chunk = Vec::new();
		}

		current_chunk.push(output_joltage);
	}
	chunks.push(current_chunk);
	chunks
}

fn permutations_in_part(part: &Vec<i64>) -> i64 {
	if part.len() < 3 {
		return 1;
	}
	let mut permutations = HashSet::new();
	permutations.insert(part.clone());
	for i in 1..part.len() - 1 {
		let mut part = part.clone();
		part.remove(i);
		if is_valid(&part) {
			permutations.insert(part.clone());
			permutations_in_part_recurse_search(part, &mut permutations);
		}
	}
	permutations.len() as i64
}

fn permutations_in_part_recurse_search(part: Vec<i64>, permutations_acc: &mut HashSet<Vec<i64>>) {
	if part.len() < 3 {
		permutations_acc.insert(part.clone());
		return;
	}
	for i in 1..part.len() - 1 {
		let mut part = part.clone();
		part.remove(i);
		if is_valid(&part) {
			permutations_acc.insert(part.clone());
			permutations_in_part_recurse_search(part, permutations_acc);
		}
	}
}

fn is_valid(slice: &[i64]) -> bool {
	let mut prev = slice[0];
	for &part in slice.iter().skip(1) {
		if part - prev > 3 {
			return false;
		}

		prev = part;
	}
	true
}

#[cfg(test)]
mod tests {
	use super::*;
	use test::Bencher;

	#[test]
	fn part_one_test_cases() {
		let input = include_str!("input.test1.txt");
		let input2 = include_str!("input.test2.txt");
		let solver = Day10Solver {};
		assert_eq!(solver.solve_part_one(input), 35);
		assert_eq!(solver.solve_part_one(input2), 220);
	}

	#[test]
	fn part_two_test_cases() {
		let input = include_str!("input.test1.txt");
		let input1 = include_str!("input.test2.txt");
		let solver = Day10Solver {};
		assert_eq!(solver.solve_part_two(input), 8);
		assert_eq!(solver.solve_part_two(input1), 19208);
	}

	#[bench]
	fn bench_part_one(bencher: &mut Bencher) {
		let input = include_str!("input.txt");
		let solver = Day10Solver {};
		bencher.iter(|| solver.solve_part_one(input));
	}

	#[bench]
	fn bench_parse_output_joltages(bencher: &mut Bencher) {
		let input = include_str!("input.txt");
		bencher.iter(|| parse_output_joltages(input));
	}

	#[bench]
	fn bench_chunk_output_joltages(bencher: &mut Bencher) {
		let input = include_str!("input.txt");
		let output_joltages = parse_output_joltages(input);
		bencher.iter(|| chunk_output_joltages(&output_joltages));
	}

	#[bench]
	fn bench_part_two(bencher: &mut Bencher) {
		let input = include_str!("input.txt");
		let solver = Day10Solver {};
		bencher.iter(|| solver.solve_part_two(input));
	}
}
