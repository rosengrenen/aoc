use crate::lib::Solver;

pub struct Day1Solver;

impl Solver for Day1Solver {
	fn solve(&self, lines: &Vec<String>, part_two: bool) -> String {
		let mut numbers = parse_numbers(lines);

		numbers.sort();
		if !part_two {
			for first_number in numbers.iter() {
				for second_number in numbers.iter() {
					if first_number + second_number == 2020 {
						return (first_number * second_number).to_string();
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
							return (first_number * second_number * third_number).to_string();
						}
					}
				}
			}
		}
		panic!("")
	}
}

fn parse_numbers(lines: &[String]) -> Vec<i64> {
	lines
		.iter()
		.map(|line| line.parse::<i64>().unwrap())
		.collect()
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::lib::read_lines;
	use test::Bencher;

	#[test]
	fn part_one_test_cases() {
		let input: Vec<String> = vec![
			"1721".to_string(),
			"979".to_string(),
			"366".to_string(),
			"299".to_string(),
			"675".to_string(),
			"1456".to_string(),
		];
		let solver: Day1Solver = Day1Solver {};
		assert_eq!(solver.solve(&input, false), "514579");
	}

	#[test]
	fn part_two_test_cases() {
		let input: Vec<String> = vec![
			"1721".to_string(),
			"979".to_string(),
			"366".to_string(),
			"299".to_string(),
			"675".to_string(),
			"1456".to_string(),
		];
		let solver: Day1Solver = Day1Solver {};
		assert_eq!(solver.solve(&input, true), "241861950");
	}

	#[bench]
	fn bench_parse_numbers(bencher: &mut Bencher) {
		let input = read_lines("src/day01/input.txt");
		bencher.iter(|| parse_numbers(&input));
	}

	#[bench]
	fn bench_part_one(bencher: &mut Bencher) {
		let input = read_lines("src/day01/input.txt");
		let solver = Day1Solver {};
		bencher.iter(|| solver.solve(&input, false));
	}

	#[bench]
	fn bench_part_two(bencher: &mut Bencher) {
		let input = read_lines("src/day01/input.txt");
		let solver = Day1Solver {};
		bencher.iter(|| solver.solve(&input, true));
	}
}
