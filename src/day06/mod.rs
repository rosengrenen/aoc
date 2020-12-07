use crate::lib::Solver;

pub struct Day6Solver;

impl Solver for Day6Solver {
	fn solve(&self, input: &str, part_two: bool) -> i64 {
		if !part_two {
			let mut questions_answered: i64 = 0;
			// "bitset" that represents if an answer was used, first bit represents 'a', the second 'b', ...
			let mut answers: i64 = 0;
			for line in input.lines() {
				if line.is_empty() {
					questions_answered += answers.count_ones() as i64;
					answers = 0;
					continue;
				}
				// Join current person answers with the rest
				for c in line.as_bytes().iter() {
					answers |= 1 << (c - b'a');
				}
			}
			questions_answered += answers.count_ones() as i64;
			questions_answered
		} else {
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
						answers |= 1 << (c - b'a');
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
					answers |= 1 << (c - b'a');
				}
				// AND with current and previous answers to get set intersection
				answers &= prev_answers;
			}
			questions_answered += answers.count_ones() as i64;
			questions_answered
		}
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use test::Bencher;

	#[test]
	fn part_one_test_cases() {
		let input = include_str!("input.test1.txt");
		let solver = Day6Solver {};
		assert_eq!(solver.solve(&input, false), 11);
	}

	#[test]
	fn part_two_test_cases() {
		let input = include_str!("input.test1.txt");
		let solver = Day6Solver {};
		assert_eq!(solver.solve(&input, true), 6);
	}

	#[bench]
	fn bench_part_one(bencher: &mut Bencher) {
		let input = include_str!("input.txt");
		let solver = Day6Solver {};
		bencher.iter(|| solver.solve(&input, false));
	}

	#[bench]
	fn bench_part_two(bencher: &mut Bencher) {
		let input = include_str!("input.txt");
		let solver = Day6Solver {};
		bencher.iter(|| solver.solve(&input, true));
	}
}
