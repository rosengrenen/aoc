use aoc_util::{Solver, SolverOutput};

#[derive(Default)]
pub struct Day10;

impl Solver for Day10 {
	fn part_one(&self, input: &str) -> SolverOutput {
		let mut adapters = parse_adapters(input);
		adapters.push(0);
		adapters.sort_unstable();
		SolverOutput::Num(part_one(&adapters))
	}

	fn part_two(&self, input: &str) -> SolverOutput {
		let mut adapters = parse_adapters(input);
		adapters.sort_unstable();
		SolverOutput::Num(part_two(&adapters))
	}
}

fn part_one(adapters: &[i64]) -> i64 {
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

fn part_two(adapters: &[i64]) -> i64 {
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
