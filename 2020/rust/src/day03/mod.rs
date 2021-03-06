use crate::lib::Solver;

pub struct Day3Solver;

impl Solver for Day3Solver {
	fn solve_part_one(&self, input: &str) -> i64 {
		let lines: Vec<&str> = input.lines().collect();
		traverse_map(&lines, 3, 1)
	}

	fn solve_part_two(&self, input: &str) -> i64 {
		let lines: Vec<&str> = input.lines().collect();
		traverse_map(&lines, 1, 1)
			* traverse_map(&lines, 3, 1)
			* traverse_map(&lines, 5, 1)
			* traverse_map(&lines, 7, 1)
			* traverse_map(&lines, 1, 2)
	}
}

fn traverse_map(map: &[&str], x_step: usize, y_step: usize) -> i64 {
	map.iter().enumerate().fold(0, |hits, (mut index, row)| {
		if index % y_step != 0 {
			return hits;
		}

		// This line is necessary, else the index is y_step times higher than it should be
		index /= y_step;
		if row.as_bytes()[index * x_step % row.len()] == b'#' {
			hits + 1
		} else {
			hits
		}
	})
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::lib::fetch_input;
	use test::Bencher;

	#[test]
	fn part_one_test_cases() {
		let input = include_str!("input.test1.txt");

		let solver = Day3Solver {};
		assert_eq!(solver.solve_part_one(input), 7);
	}

	#[test]
	fn part_two_test_cases() {
		let input = include_str!("input.test1.txt");
		let lines: Vec<&str> = input.lines().collect();

		assert_eq!(traverse_map(&lines, 1, 1), 2);
		assert_eq!(traverse_map(&lines, 3, 1), 7);
		assert_eq!(traverse_map(&lines, 5, 1), 3);
		assert_eq!(traverse_map(&lines, 7, 1), 4);
		assert_eq!(traverse_map(&lines, 1, 2), 2);

		let solver = Day3Solver {};
		assert_eq!(solver.solve_part_two(input), 336);
	}

	#[bench]
	fn bench_part_one(bencher: &mut Bencher) {
		let input = fetch_input(3);
		let solver = Day3Solver {};
		bencher.iter(|| solver.solve_part_one(&input));
	}

	#[bench]
	fn bench_part_two(bencher: &mut Bencher) {
		let input = fetch_input(3);
		let solver = Day3Solver {};
		bencher.iter(|| solver.solve_part_two(&input));
	}
}
