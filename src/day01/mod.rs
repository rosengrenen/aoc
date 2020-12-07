use crate::lib::Solver;

pub struct Day1Solver;

impl Solver for Day1Solver {
	fn solve(&self, input: &str, part_two: bool) -> i64 {
		let mut numbers = parse_numbers(input);

		numbers.sort_unstable();
		if !part_two {
			for first_number in numbers.iter() {
				for second_number in numbers.iter() {
					if first_number + second_number == 2020 {
						return first_number * second_number;
					}
				}
			}
		} else {
			for &first_number in numbers.iter() {
				if first_number > 2020 {
					continue;
				}
				for &second_number in numbers.iter() {
					if first_number + second_number > 2020 {
						continue;
					}
					for &third_number in numbers.iter() {
						if first_number + second_number + third_number == 2020 {
							return first_number * second_number * third_number;
						}
					}
				}
			}
		}
		panic!("")
	}
}

fn parse_numbers(input: &str) -> Vec<i64> {
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
		let solver: Day1Solver = Day1Solver {};
		assert_eq!(solver.solve(input, false), 514579);
	}

	#[test]
	fn part_two_test_cases() {
		let input = include_str!("input.test1.txt");
		let solver: Day1Solver = Day1Solver {};
		assert_eq!(solver.solve(input, true), 241861950);
	}

	#[bench]
	fn bench_parse_numbers(bencher: &mut Bencher) {
		let input = include_str!("input.txt");
		bencher.iter(|| parse_numbers(input));
	}

	#[bench]
	fn bench_part_one(bencher: &mut Bencher) {
		let input = include_str!("input.txt");
		let solver = Day1Solver {};
		bencher.iter(|| solver.solve(input, false));
	}

	#[bench]
	fn bench_part_two(bencher: &mut Bencher) {
		let input = include_str!("input.txt");
		let solver = Day1Solver {};
		bencher.iter(|| solver.solve(input, true));
	}
}
