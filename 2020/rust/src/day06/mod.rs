use crate::lib::Solver;

pub struct Day6Solver;

impl Solver for Day6Solver {
	fn solve_part_one(&self, input: &str) -> i64 {
		let mut questions_answered: i64 = 0;
		for batch in input.split("\n\n") {
			// "bitset" that represents if an answer was used, first bit represents 'a', the second 'b', ...
			let mut answers: i64 = 0;
			// Join all answers to get set union
			for c in batch.as_bytes().iter() {
				if *c == b'\n' {
					continue;
				}
				answers |= 1 << (c - b'a');
			}
			questions_answered += answers.count_ones() as i64;
		}
		questions_answered
	}

	fn solve_part_two(&self, input: &str) -> i64 {
		let mut questions_answered: i64 = 0;
		// "bitset" that represents if an answer was used, first bit represents 'a', the second 'b', ...
		let mut answers: i64 = 0;
		// Flag that signifies first loop of new batch/group of people
		let mut first = true;
		for line in input.lines() {
			if line.is_empty() {
				questions_answered += answers.count_ones() as i64;
				first = true;
				continue;
			}
			if first {
				first = false;
				// Reset bits for new batch
				answers = 0;
				// Set the bits for the first person ov the group
				for c in line.as_bytes() {
					set_bit(&mut answers, (c - b'a') as usize);
				}
				continue;
			}
			let prev_answers = answers;
			if answers == 0 {
				continue;
			}
			answers = 0;
			// Set bits for the current persons answers
			for c in line.as_bytes() {
				set_bit(&mut answers, (c - b'a') as usize);
			}
			// AND with current and previous answers to get set intersection
			answers &= prev_answers;
		}
		questions_answered += answers.count_ones() as i64;
		questions_answered
	}
}

fn set_bit(bit_set: &mut i64, bit_index: usize) {
	*bit_set |= 1 << (bit_index % 64);
}

#[cfg(test)]
mod tests {
	use super::*;
	use test::Bencher;

	#[test]
	fn part_one_test_cases() {
		let input = include_str!("input.test1.txt");
		let solver = Day6Solver {};
		assert_eq!(solver.solve_part_one(input), 11);
	}

	#[test]
	fn part_two_test_cases() {
		let input = include_str!("input.test1.txt");
		let solver = Day6Solver {};
		assert_eq!(solver.solve_part_two(input), 6);
	}

	#[bench]
	fn bench_part_one(bencher: &mut Bencher) {
		let input = include_str!("input.txt");
		let solver = Day6Solver {};
		bencher.iter(|| solver.solve_part_one(input));
	}

	#[bench]
	fn bench_part_two(bencher: &mut Bencher) {
		let input = include_str!("input.txt");
		let solver = Day6Solver {};
		bencher.iter(|| solver.solve_part_two(input));
	}
}
