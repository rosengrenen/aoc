use aoc_util::{Solver, SolverOutput};

#[derive(Default)]
pub struct Day3;

impl Solver for Day3 {
	fn part_one(&self, input: &str) -> SolverOutput {
		let nums = parse_nums(input);
		let nums_len = nums.len();
		let num_len = nums[0].len();
		let gamma_rate = (0..num_len).fold(0, |prev, i| {
			let occurences =
				(0..nums_len).fold(0, |prev, j| prev + (nums[j].as_bytes()[i] - b'0') as i64);
			if occurences > nums_len as i64 - occurences {
				(prev << 1) + 1
			} else {
				prev << 1
			}
		});
		let epsilon_rate = (!gamma_rate) & ((1 << num_len) - 1);
		SolverOutput::Num(gamma_rate * epsilon_rate)
	}

	fn part_two(&self, input: &str) -> SolverOutput {
		let nums = parse_nums(input);
		let num_len = nums[0].len();
		let mut oxygen_nums = nums.clone();
		for i in 0..num_len {
			if oxygen_nums.len() == 1 {
				break;
			}

			let ones = oxygen_nums
				.iter()
				.fold(0, |prev, cur| prev + (cur.as_bytes()[i] - b'0') as i64);

			if ones >= oxygen_nums.len() as i64 - ones {
				oxygen_nums = oxygen_nums
					.into_iter()
					.filter(|num| num.as_bytes()[i] == b'1')
					.collect();
			} else {
				oxygen_nums = oxygen_nums
					.into_iter()
					.filter(|num| num.as_bytes()[i] == b'0')
					.collect();
			}
		}

		let mut co2_nums = nums.clone();
		for i in 0..num_len {
			if co2_nums.len() == 1 {
				break;
			}

			let ones = co2_nums
				.iter()
				.fold(0, |prev, cur| prev + (cur.as_bytes()[i] - b'0') as i64);

			if ones < co2_nums.len() as i64 - ones {
				co2_nums = co2_nums
					.into_iter()
					.filter(|num| num.as_bytes()[i] == b'1')
					.collect();
			} else {
				co2_nums = co2_nums
					.into_iter()
					.filter(|num| num.as_bytes()[i] == b'0')
					.collect();
			}
		}

		let oxygen_rate = (0..num_len).fold(0, |prev, i| {
			if oxygen_nums[0].as_bytes()[i] == b'1' {
				(prev << 1) + 1
			} else {
				prev << 1
			}
		});

		let co2_rate = (0..num_len).fold(0, |prev, i| {
			if co2_nums[0].as_bytes()[i] == b'1' {
				(prev << 1) + 1
			} else {
				prev << 1
			}
		});

		SolverOutput::Num(oxygen_rate * co2_rate)
	}
}

fn parse_nums(input: &str) -> Vec<&str> {
	input.lines().collect()
}

#[cfg(test)]
mod benches {
	use super::*;
	use aoc_util::get_input;
	use test::{black_box, Bencher};

	#[bench]
	fn parse(bencher: &mut Bencher) {
		let input = get_input(2021, 3).unwrap();
		bencher.iter(|| parse_nums(black_box(&input)));
	}
}
