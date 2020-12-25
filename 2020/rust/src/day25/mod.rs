use crate::lib::Solver;

pub struct Day25Solver;

impl Solver for Day25Solver {
	fn solve_part_one(&self, input: &str) -> i64 {
		let (card_pub_key, door_pub_key) = parse_pub_keys(input);

		let mut door_loop_size = 1;
		let mut value = 1;
		loop {
			value *= 7;
			value %= 20_201_227;
			if value == door_pub_key {
				break;
			}

			door_loop_size += 1;
		}

		let mut value = 1;
		for _ in 0..door_loop_size {
			value *= card_pub_key;
			value %= 20_201_227;
		}

		value
	}

	fn solve_part_two(&self, _input: &str) -> i64 {
		0
	}
}

fn parse_pub_keys(input: &str) -> (i64, i64) {
	let (card_pub_key_raw, door_pub_key_raw) = input.split_once("\n").unwrap();

	(
		card_pub_key_raw.parse().unwrap(),
		door_pub_key_raw.parse().unwrap(),
	)
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::lib::fetch_input;
	use test::Bencher;

	#[test]
	fn part_one_test_cases() {
		let input = include_str!("input.test1.txt");
		let solver = Day25Solver {};
		assert_eq!(solver.solve_part_one(input), 14897079);
	}

	#[test]
	fn part_two_test_cases() {
		let input = include_str!("input.test1.txt");
		let solver = Day25Solver {};
		assert_eq!(solver.solve_part_two(input), 0);
	}

	#[bench]
	fn bench_parse_pub_keys(bencher: &mut Bencher) {
		let input = fetch_input(25);
		bencher.iter(|| parse_pub_keys(&input));
	}

	#[bench]
	fn bench_part_one(bencher: &mut Bencher) {
		let input = fetch_input(25);
		let solver = Day25Solver {};
		bencher.iter(|| solver.solve_part_one(&input));
	}
}
