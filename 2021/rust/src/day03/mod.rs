use aoc_util::{Solver, SolverOutput};

#[derive(Default)]
pub struct Day3;

impl Solver for Day3 {
	fn part_one(&self, input: &str) -> SolverOutput {
		let nums = parse_nums(input);
		let nums_len = nums.len() as i64;
		let num_len = nums[0].len();
		let gamma_rate = (0..num_len)
			.map(|i| {
				nums.iter().fold(0, |occurences, num| {
					occurences + (num.as_bytes()[i] - b'0') as i64
				})
			})
			.map(|occs| if 2 * occs > nums_len { 1 } else { 0 })
			.fold(0, |gamma_rate, binary| (gamma_rate << 1) + binary);
		let epsilon_rate = !gamma_rate & (1 << num_len) - 1;
		SolverOutput::Num(gamma_rate * epsilon_rate)
	}

	fn part_two(&self, input: &str) -> SolverOutput {
		let nums = parse_nums(input);
		let oxygen_rate = find_num(nums.clone(), |ones, len| ones >= len - ones);
		let co2_rate = find_num(nums, |ones, len| 2 * ones < len);
		SolverOutput::Num(oxygen_rate * co2_rate)
	}
}

fn parse_nums(input: &str) -> Vec<&str> {
	input.lines().collect()
}

fn find_num<F>(mut nums: Vec<&str>, pred: F) -> i64
where
	F: Fn(i64, i64) -> bool,
{
	let num_len = nums[0].len();
	for i in 0..num_len {
		if nums.len() <= 1 {
			break;
		}

		let ones = nums
			.iter()
			.fold(0, |ones, num| ones + (num.as_bytes()[i] - b'0') as i64);

		let filter_on = if pred(ones, nums.len() as i64) {
			b'1'
		} else {
			b'0'
		};

		nums = nums
			.into_iter()
			.filter(|num| num.as_bytes()[i] == filter_on)
			.collect();
	}

	assert_eq!(nums.len(), 1);

	i64::from_str_radix(nums[0], 2).unwrap()
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
