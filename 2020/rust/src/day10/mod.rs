use crate::lib::Solver;

pub struct Day10Solver;

impl Solver for Day10Solver {
	fn solve_part_one(&self, input: &str) -> i64 {
		let mut adapters = parse_adapters(input);
		adapters.push(0);
		adapters.sort_unstable();
		solve_part_one(&adapters)
	}

	fn solve_part_two(&self, input: &str) -> i64 {
		let mut adapters = parse_adapters(input);
		adapters.sort_unstable();
		solve_part_two(&adapters)
	}
}

fn solve_part_one(adapters: &[i64]) -> i64 {
	let mut ones = 0;
	let mut threes = 1;
	let mut prev_adapter = adapters[0];
	for &adapter in &adapters[1..] {
		match adapter - prev_adapter {
			1 => ones += 1,
			3 => threes += 1,
			_ => (),
		}

		prev_adapter = adapter;
	}

	ones * threes
}

fn solve_part_two(adapters: &[i64]) -> i64 {
	let mut sliding_window = [1, 0, 0, 0];
	let mut sliding_window_index = adapters[adapters.len() - 1];
	for &adapter in adapters.iter().rev() {
		// negative number denoting how many steps have been taken since the last adapter
		let current_local_sliding_window_index = adapter - sliding_window_index;
		if current_local_sliding_window_index < 0 {
			shift_sliding_window(&mut sliding_window, -current_local_sliding_window_index);
			sliding_window_index = adapter;
		}

		if sliding_window[0] != 0 {
			continue;
		}

		sliding_window[0] = sliding_window[1] + sliding_window[2] + sliding_window[3];
	}
	shift_sliding_window(&mut sliding_window, sliding_window_index);
	sliding_window[1] + sliding_window[2] + sliding_window[3]
}

// shift by shift_by steps to the right
fn shift_sliding_window(sliding_window: &mut [i64], shift_by: i64) {
	match shift_by {
		3 => {
			sliding_window[3] = sliding_window[0];
			sliding_window[2] = 0;
			sliding_window[1] = 0;
		}
		2 => {
			sliding_window[3] = sliding_window[1];
			sliding_window[2] = sliding_window[0];
			sliding_window[1] = 0;
		}
		1 => {
			sliding_window[3] = sliding_window[2];
			sliding_window[2] = sliding_window[1];
			sliding_window[1] = sliding_window[0];
		}
		_ => {
			sliding_window[3] = 0;
			sliding_window[2] = 0;
			sliding_window[1] = 0;
		}
	}
	sliding_window[0] = 0;
}

fn parse_adapters(input: &str) -> Vec<i64> {
	input
		.lines()
		.map(|line| line.parse::<i64>().unwrap())
		.collect()
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
	fn bench_parse_adapters(bencher: &mut Bencher) {
		let input = include_str!("input.txt");
		bencher.iter(|| parse_adapters(input));
	}

	#[bench]
	fn bench_part_one(bencher: &mut Bencher) {
		let input = include_str!("input.txt");
		let solver = Day10Solver {};
		bencher.iter(|| solver.solve_part_one(input));
	}

	#[bench]
	fn bench_part_two(bencher: &mut Bencher) {
		let input = include_str!("input.txt");
		let solver = Day10Solver {};
		bencher.iter(|| solver.solve_part_two(input));
	}
}
