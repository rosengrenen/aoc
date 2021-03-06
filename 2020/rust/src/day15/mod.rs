use crate::lib::Solver;

pub struct Day15Solver;

impl Solver for Day15Solver {
	fn solve_part_one(&self, input: &str) -> i64 {
		let input = parse_numbers(input);
		find_nth_van_eck(&input, 2020)
	}

	fn solve_part_two(&self, input: &str) -> i64 {
		let input = parse_numbers(input);
		find_nth_van_eck(&input, 30_000_000)
	}
}

fn parse_numbers(input: &str) -> Vec<i64> {
	input.split(',').map(|c| c.parse().unwrap()).collect()
}

fn find_nth_van_eck(starting_numbers: &[i64], n: i64) -> i64 {
	let mut last_pos = vec![0; n as usize];
	let mut last_num = 0;
	for (index, &num) in starting_numbers.iter().enumerate() {
		last_pos[num as usize] = index as i64 + 1;
		last_num = num;
	}

	let mut index = starting_numbers.len() as i64;
	while index < n {
		let mut next_num = 0;
		let last_num_pos = last_pos[last_num as usize];
		if last_num_pos != 0 {
			next_num = index - last_num_pos;
			last_pos[last_num as usize] = index;
		} else {
			last_pos[last_num as usize] = index;
		}

		last_num = next_num;
		index += 1;
	}

	last_num
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::lib::fetch_input;
	use test::Bencher;

	#[test]
	fn part_one_test_cases() {
		let solver = Day15Solver {};
		assert_eq!(solver.solve_part_one(&"0,3,6"), 436);
		assert_eq!(solver.solve_part_one(&"1,3,2"), 1);
		assert_eq!(solver.solve_part_one(&"2,1,3"), 10);
		assert_eq!(solver.solve_part_one(&"1,2,3"), 27);
		assert_eq!(solver.solve_part_one(&"2,3,1"), 78);
		assert_eq!(solver.solve_part_one(&"3,2,1"), 438);
		assert_eq!(solver.solve_part_one(&"3,1,2"), 1836);
	}

	// #[test]
	// fn part_two_test_cases() {
	// 	let solver = Day15Solver {};
	// 	assert_eq!(solver.solve_part_two(&"0,3,6"), 175594);
	// 	assert_eq!(solver.solve_part_two(&"1,3,2"), 2578);
	// 	assert_eq!(solver.solve_part_two(&"2,1,3"), 3544142);
	// 	assert_eq!(solver.solve_part_two(&"1,2,3"), 261214);
	// 	assert_eq!(solver.solve_part_two(&"2,3,1"), 6895259);
	// 	assert_eq!(solver.solve_part_two(&"3,2,1"), 18);
	// 	assert_eq!(solver.solve_part_two(&"3,1,2"), 362);
	// }

	#[bench]
	fn bench_parse_numbers(bencher: &mut Bencher) {
		let input = fetch_input(15);
		bencher.iter(|| parse_numbers(&input));
	}

	#[bench]
	fn bench_part_one(bencher: &mut Bencher) {
		let input = fetch_input(15);
		let solver = Day15Solver {};
		bencher.iter(|| solver.solve_part_one(&input));
	}

	#[bench]
	fn bench_part_two(bencher: &mut Bencher) {
		let input = fetch_input(15);
		let solver = Day15Solver {};
		bencher.iter(|| solver.solve_part_two(&input));
	}
}
