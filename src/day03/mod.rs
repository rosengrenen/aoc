use crate::lib::Solver;

pub struct Day3Solver;

impl Solver for Day3Solver {
	fn solve(&self, lines: Vec<String>, part_two: bool) -> String {
		let map = create_map(&lines);

		if !part_two {
			traverse_map(&map, 3, 1).to_string()
		} else {
			(traverse_map(&map, 1, 1)
				* traverse_map(&map, 3, 1)
				* traverse_map(&map, 5, 1)
				* traverse_map(&map, 7, 1)
				* traverse_map(&map, 1, 2))
			.to_string()
		}
	}
}

fn create_map(lines: &Vec<String>) -> Vec<Vec<bool>> {
	lines
		.iter()
		.map(|line| {
			line.as_bytes()
				.iter()
				.map(|&c| c == b'#')
				.collect::<Vec<bool>>()
		})
		.collect()
}

fn traverse_map(map: &Vec<Vec<bool>>, x_step: usize, y_step: usize) -> i64 {
	map.iter().enumerate().fold(0, |hits, (mut index, row)| {
		if index % y_step != 0 {
			return hits;
		}

		// This line is necessary, else the index is y_step times higher than it should be
		index /= y_step;
		if row[index * x_step % row.len()] {
			hits + 1
		} else {
			hits
		}
	})
}

#[cfg(test)]
mod tests {
	use super::*;

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
		assert_eq!(solver.solve(input, false), "7");
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

		let map = create_map(&input);
		assert_eq!(traverse_map(&map, 1, 1), 2);
		assert_eq!(traverse_map(&map, 3, 1), 7);
		assert_eq!(traverse_map(&map, 5, 1), 3);
		assert_eq!(traverse_map(&map, 7, 1), 4);
		assert_eq!(traverse_map(&map, 1, 2), 2);

		let solver = Day3Solver {};
		assert_eq!(solver.solve(input, true), "336");
	}
}
