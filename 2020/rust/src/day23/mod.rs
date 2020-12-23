use crate::lib::Solver;

pub struct Day23Solver;

impl Solver for Day23Solver {
	fn solve_part_one(&self, input: &str) -> i64 {
		let input = parse_cups(input);
		let mut forward_links = vec![0; input.len()];
		for i in 0..input.len() - 1 {
			forward_links[input[i] as usize - 1] = input[i + 1];
		}
		forward_links[input[input.len() - 1] as usize - 1] = input[0];

		let mut current = input[0];
		let input_max = *input.iter().max().unwrap();
		for _ in 0..100 {
			let a = forward_links[current as usize - 1];
			let b = forward_links[a as usize - 1];
			let c = forward_links[b as usize - 1];

			let mut destination = current;
			loop {
				if destination == 1 {
					destination = input_max;
				} else {
					destination -= 1;
				}

				if destination != a && destination != b && destination != c {
					break;
				}
			}

			// "remove" picked values
			let next_current = forward_links[c as usize - 1];
			forward_links[current as usize - 1] = next_current;

			// "insert" picked values after destination
			let after_destination = forward_links[destination as usize - 1];
			forward_links[destination as usize - 1] = a;
			forward_links[c as usize - 1] = after_destination;

			current = next_current;
		}

		let mut num = 0;
		let mut next = forward_links[0];
		while next != 1 {
			num *= 10;
			num += next;
			next = forward_links[next as usize - 1];
		}
		num
	}

	fn solve_part_two(&self, input: &str) -> i64 {
		const FILL_SIZE: usize = 1_000_000;
		let input = parse_cups(input);
		let mut forward_links = vec![0; FILL_SIZE];
		let input_max = *input.iter().max().unwrap();
		for i in 0..input.len() - 1 {
			forward_links[input[i] as usize - 1] = input[i + 1];
		}
		forward_links[input[input.len() - 1] as usize - 1] = input_max + 1;

		for i in input_max + 1..FILL_SIZE as i64 {
			forward_links[i as usize - 1] = i + 1;
		}
		forward_links[FILL_SIZE - 1] = input[0];

		let mut current = input[0];
		let input_max = FILL_SIZE as i64;
		for _ in 0..10_000_000 {
			let a = forward_links[current as usize - 1];
			let b = forward_links[a as usize - 1];
			let c = forward_links[b as usize - 1];

			let mut destination = current;
			loop {
				if destination == 1 {
					destination = input_max;
				} else {
					destination -= 1;
				}

				if destination != a && destination != b && destination != c {
					break;
				}
			}

			// "remove" picked values
			let next_current = forward_links[c as usize - 1];
			forward_links[current as usize - 1] = next_current;

			// "insert" picked values after destination
			let after_destination = forward_links[destination as usize - 1];
			forward_links[destination as usize - 1] = a;
			forward_links[c as usize - 1] = after_destination;

			current = next_current;
		}

		let first_number = forward_links[0];
		let second_number = forward_links[first_number as usize - 1];

		first_number * second_number
	}
}

fn parse_cups(input: &str) -> Vec<i64> {
	input
		.as_bytes()
		.iter()
		.map(|c| (*c - b'0') as i64)
		.collect()
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::lib::fetch_input;
	use test::Bencher;

	#[test]
	fn part_one_test_cases() {
		let input = include_str!("input.test1.txt");
		let solver = Day23Solver {};
		assert_eq!(solver.solve_part_one(input), 67384529);
	}

	#[test]
	fn part_two_test_cases() {
		let input = include_str!("input.test1.txt");
		let solver = Day23Solver {};
		assert_eq!(solver.solve_part_two(input), 149245887792);
	}

	#[bench]
	fn bench_parse_cups(bencher: &mut Bencher) {
		let input = fetch_input(23);
		bencher.iter(|| parse_cups(&input));
	}

	#[bench]
	fn bench_part_one(bencher: &mut Bencher) {
		let input = fetch_input(23);
		let solver = Day23Solver {};
		bencher.iter(|| solver.solve_part_one(&input));
	}

	#[bench]
	fn bench_part_two(bencher: &mut Bencher) {
		let input = fetch_input(23);
		let solver = Day23Solver {};
		bencher.iter(|| solver.solve_part_two(&input));
	}
}
