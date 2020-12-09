use crate::lib::Solver;

pub struct Day9Solver;

impl Solver for Day9Solver {
	fn solve(&self, input: &str, part_two: bool) -> i64 {
		let numbers = parse_numbers(input);
		if !part_two {
			find_first_invalid(&numbers, 25)
		} else {
			find_invalid_other(&numbers, 25)
		}
	}
}

fn parse_numbers(input: &str) -> Vec<i64> {
	input.lines().map(|line| line.parse().unwrap()).collect()
}

fn find_first_invalid(numbers: &Vec<i64>, preamble_length: i64) -> i64 {
	for i in preamble_length..numbers.len() as i64 {
		let current_number = numbers[i as usize];
		let mut found = false;
		'outer: for &x in &numbers[(i - preamble_length) as usize..i as usize] {
			for &y in &numbers[(i - preamble_length) as usize..i as usize] {
				if x != y && x + y == current_number {
					found = true;
					break 'outer;
				}
			}
		}
		if !found {
			return current_number;
		}
	}
	panic!("No invalid number in sequence");
}

fn find_invalid_other(numbers: &[i64], preamble_length: i64) -> i64 {
	for i in preamble_length..numbers.len() as i64 {
		let current_number = numbers[i as usize];
		let mut found = false;
		'outer: for &x in &numbers[(i - preamble_length) as usize..i as usize] {
			for &y in &numbers[(i - preamble_length) as usize..i as usize] {
				if x != y && x + y == current_number {
					found = true;
					break 'outer;
				}
			}
		}
		if !found {
			let (low, high) =
				sum_contigious_sum_in_range(&numbers[0..i as usize - 1], current_number);
			return low + high;
		}
	}
	panic!("No invalid number in sequence");
}

fn sum_contigious_sum_in_range(range: &[i64], target: i64) -> (i64, i64) {
	let mut smallest = 0;
	let mut biggest = 1;
	let mut found = false;
	while biggest < range.len() {
		let sum: i64 = range[smallest..biggest].iter().sum();
		if sum == target {
			println!("{:?}", &range[smallest..biggest]);
			found = true;
			break;
		} else if sum < target {
			biggest += 1;
		} else if sum > target {
			if smallest + 1 < biggest {
				smallest += 1;
			} else {
				panic!();
			}
		}
	}
	if found {
		let min: i64 = *range[smallest..biggest].iter().min().unwrap();
		let max: i64 = *range[smallest..biggest].iter().max().unwrap();
		(min, max)
	} else {
		panic!()
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use test::Bencher;

	#[test]
	fn part_one_test_cases() {
		let input = include_str!("input.test1.txt");
		assert_eq!(find_first_invalid(&parse_numbers(input), 5), 127);
	}

	#[test]
	fn part_two_test_cases() {
		let input = include_str!("input.test1.txt");
		assert_eq!(find_invalid_other(&parse_numbers(input), 5), 62);
	}

	// #[bench]
	// fn bench_parse_instructions(bencher: &mut Bencher) {
	// 	let input = include_str!("input.txt");
	// 	bencher.iter(|| parse_program(input));
	// }

	// #[bench]
	// fn bench_part_one(bencher: &mut Bencher) {
	// 	let input = include_str!("input.txt");
	// 	let solver = Day9Solver {};
	// 	bencher.iter(|| solver.solve(&input, false));
	// }

	// #[bench]
	// fn bench_part_two(bencher: &mut Bencher) {
	// 	let input = include_str!("input.txt");
	// 	let solver = Day9Solver {};
	// 	bencher.iter(|| solver.solve(&input, true));
	// }
}
