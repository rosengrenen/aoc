use crate::lib::Solver;

pub struct Day3Solver;

impl Solver for Day3Solver {
	fn solve(&self, lines: &Vec<String>, part_two: bool) -> String {
		if !part_two {
			traverse_map(lines, 3, 1).to_string()
		} else {
			(traverse_map(lines, 1, 1)
				* traverse_map(lines, 3, 1)
				* traverse_map(lines, 5, 1)
				* traverse_map(lines, 7, 1)
				* traverse_map(lines, 1, 2))
			.to_string()
		}
	}
}

fn traverse_map(map: &[String], x_step: usize, y_step: usize) -> i64 {
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
	use crate::lib::read_lines;
	use test::Bencher;

	#[test]
	fn part_one_test_cases() {
		let input: Vec<String> = vec![
			"..##.......".to_string(),
			"#...#...#..".to_string(),
			".#....#..#.".to_string(),
			"..#.#...#.#".to_string(),
			".#...##..#.".to_string(),
			"..#.##.....".to_string(),
			".#.#.#....#".to_string(),
			".#........#".to_string(),
			"#.##...#...".to_string(),
			"#...##....#".to_string(),
			".#..#...#.#".to_string(),
		];

		let solver = Day3Solver {};
		assert_eq!(solver.solve(&input, false), "7");
	}

	#[test]
	fn part_two_test_cases() {
		let input: Vec<String> = vec![
			"..##.......".to_string(),
			"#...#...#..".to_string(),
			".#....#..#.".to_string(),
			"..#.#...#.#".to_string(),
			".#...##..#.".to_string(),
			"..#.##.....".to_string(),
			".#.#.#....#".to_string(),
			".#........#".to_string(),
			"#.##...#...".to_string(),
			"#...##....#".to_string(),
			".#..#...#.#".to_string(),
		];

		assert_eq!(traverse_map(&input, 1, 1), 2);
		assert_eq!(traverse_map(&input, 3, 1), 7);
		assert_eq!(traverse_map(&input, 5, 1), 3);
		assert_eq!(traverse_map(&input, 7, 1), 4);
		assert_eq!(traverse_map(&input, 1, 2), 2);

		let solver = Day3Solver {};
		assert_eq!(solver.solve(&input, true), "336");
	}

	#[bench]
	fn bench_part_one(bencher: &mut Bencher) {
		let input = read_lines("src/day03/input.txt");
		let solver = Day3Solver {};
		bencher.iter(|| solver.solve(&input, false));
	}

	#[bench]
	fn bench_part_two(bencher: &mut Bencher) {
		let input = read_lines("src/day03/input.txt");
		let solver = Day3Solver {};
		bencher.iter(|| solver.solve(&input, true));
	}
}
