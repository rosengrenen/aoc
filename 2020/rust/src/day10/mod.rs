use crate::lib::Solver;

pub struct Day10Solver;

impl Solver for Day10Solver {
	fn solve_part_one(&self, input: &str) -> i64 {
		let mut output_joltages = parse_output_joltages(input);
		output_joltages.push(0);
		output_joltages.sort_unstable();
		solve_part_one(&output_joltages)
	}

	fn solve_part_two(&self, input: &str) -> i64 {
		let mut output_joltages = parse_output_joltages(input);
		output_joltages.push(0);
		output_joltages.sort_unstable();
		solve_part_two(&chunk_output_joltages(&output_joltages))
	}
}

fn solve_part_one(output_joltages: &Vec<i64>) -> i64 {
	let mut ones = 0;
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

fn solve_part_two(chunked_output_joltages: &Vec<Vec<i64>>) -> i64 {
	chunked_output_joltages
		.iter()
		.map(|chunk| permutations_in_chunk(chunk))
		.product()
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

fn permutations_in_chunk(part: &Vec<i64>) -> i64 {
	if part.len() < 3 {
		return 1;
	}

	let part_length = part.len();
	let mut permutations = 1;
	let mut index_stack = Vec::with_capacity(64);
	index_stack.push(1);
	loop {
		if index_stack[0] == part_length - 1 {
			break;
		}

		// Unwind stack
		while index_stack[index_stack.len() - 1] > part_length - 1 {
			index_stack.pop();
		}

		// Check if permutation is ok
		let mut prev_joltage = part[0];
		let mut valid = true;
		for i in 1..part_length {
			if index_stack.contains(&i) {
				continue;
			}

			let current_joltage = part[i];
			if current_joltage - prev_joltage > 3 {
				valid = false;
				break;
			}

			prev_joltage = current_joltage;
		}

		if valid {
			permutations += 1;
			let index_stack_length = index_stack.len();
			index_stack[index_stack_length - 1] += 1;
			index_stack.push(index_stack[index_stack.len() - 1] + 1);
		} else {
			let index_stack_length = index_stack.len();
			index_stack[index_stack_length - 1] += 1;
		}
	}

	permutations
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
	fn bench_part_one(bencher: &mut Bencher) {
		let input = include_str!("input.txt");
		let solver = Day10Solver {};
		bencher.iter(|| solver.solve_part_one(input));
	}

	#[bench]
	fn bench_part_one_skip_parse(bencher: &mut Bencher) {
		let input = include_str!("input.txt");
		let output_joltages = parse_output_joltages(input);
		bencher.iter(|| solve_part_one(&output_joltages));
	}

	#[bench]
	fn bench_part_two(bencher: &mut Bencher) {
		let input = include_str!("input.txt");
		let solver = Day10Solver {};
		bencher.iter(|| solver.solve_part_two(input));
	}

	#[bench]
	fn bench_part_two_skip_parse(bencher: &mut Bencher) {
		let input = include_str!("input.txt");
		let output_joltages = parse_output_joltages(input);
		let chunked_output_voltages = chunk_output_joltages(&output_joltages);
		bencher.iter(|| solve_part_two(&chunked_output_voltages));
	}
}
