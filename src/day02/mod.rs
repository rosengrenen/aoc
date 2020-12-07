use crate::lib::Solver;

pub struct Day2Solver;

impl Solver for Day2Solver {
	fn solve(&self, input: &str, part_two: bool) -> i64 {
		let mut count = 0;
		for line in input.lines() {
			let mut first_split = line.split(':');
			let mut second_split = first_split.next().unwrap().split(' ');
			let mut third_split = second_split.next().unwrap().split('-');
			let min_occurences: i64 = third_split.next().unwrap().parse().unwrap();
			let max_occurences: i64 = third_split.next().unwrap().parse().unwrap();
			let character: u8 = second_split.next().unwrap().as_bytes()[0];
			let password: &str = first_split.next().unwrap().trim();

			if !part_two {
				let mut occurences = 0;
				for &c in password.as_bytes().iter() {
					if c == character {
						occurences += 1;
					}

					if occurences > max_occurences {
						break;
					}
				}

				if occurences >= min_occurences && occurences <= max_occurences {
					count += 1;
				}
			} else {
				let mut occurences = 0;
				if password.as_bytes()[min_occurences as usize - 1] == character {
					occurences += 1;
				}
				if password.as_bytes()[max_occurences as usize - 1] == character {
					occurences += 1;
				}
				if occurences == 1 {
					count += 1;
				}
			}
		}
		count
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use test::Bencher;

	#[test]
	fn part_one_test_cases() {
		let input = include_str!("input.test1.txt");

		let solver: Day2Solver = Day2Solver {};
		assert_eq!(solver.solve(input, false), 2);
	}

	#[test]
	fn part_two_test_cases() {
		let input = include_str!("input.test1.txt");

		let solver: Day2Solver = Day2Solver {};
		assert_eq!(solver.solve(input, true), 1);
	}

	#[bench]
	fn bench_part_one(bencher: &mut Bencher) {
		let input = include_str!("input.txt");
		let solver = Day2Solver {};
		bencher.iter(|| solver.solve(input, false));
	}

	#[bench]
	fn bench_part_two(bencher: &mut Bencher) {
		let input = include_str!("input.txt");
		let solver = Day2Solver {};
		bencher.iter(|| solver.solve(input, true));
	}
}
