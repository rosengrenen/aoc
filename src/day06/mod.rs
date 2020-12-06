use crate::lib::Solver;

pub struct Day6Solver;

impl Solver for Day6Solver {
	fn solve(&self, lines: &[String], part_two: bool) -> String {
		if !part_two {
			let mut questions_answered = 0;
			let mut current_group_answers = Vec::new();
			for line in lines.iter() {
				if line.is_empty() {
					questions_answered += current_group_answers.len();
					current_group_answers.clear();
					continue;
				}
				for c in line.as_bytes().iter() {
					if current_group_answers.iter().all(|&c1| c1 != c) {
						current_group_answers.push(c);
					}
				}
			}
			questions_answered += current_group_answers.len();
			questions_answered.to_string()
		} else {
			let mut questions_answered = 0;
			let mut current_group_answers = Vec::new();
			let mut first = true;
			for line in lines.iter() {
				if line.is_empty() {
					first = true;
					questions_answered += current_group_answers.len();
					current_group_answers.clear();
					continue;
				}
				if first {
					first = false;
					for c in line.as_bytes().iter() {
						current_group_answers.push(c);
					}
				} else {
					current_group_answers = current_group_answers
						.into_iter()
						.filter(|&&answer| line.as_bytes().iter().any(|&c| c == answer))
						.collect();
				}
			}
			questions_answered += current_group_answers.len();
			questions_answered.to_string()
		}
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::lib::read_lines;
	use test::Bencher;

	#[test]
	fn part_one_test_cases() {
		let input: Vec<String> = vec![
			"abc".to_string(),
			"".to_string(),
			"a".to_string(),
			"b".to_string(),
			"c".to_string(),
			"".to_string(),
			"ab".to_string(),
			"ac".to_string(),
			"".to_string(),
			"a".to_string(),
			"a".to_string(),
			"a".to_string(),
			"a".to_string(),
			"".to_string(),
			"b".to_string(),
		];

		let solver = Day6Solver {};
		assert_eq!(solver.solve(&input, false), "11");
	}

	#[test]
	fn part_two_test_cases() {
		let input: Vec<String> = vec![
			"abc".to_string(),
			"".to_string(),
			"a".to_string(),
			"b".to_string(),
			"c".to_string(),
			"".to_string(),
			"ab".to_string(),
			"ac".to_string(),
			"".to_string(),
			"a".to_string(),
			"a".to_string(),
			"a".to_string(),
			"a".to_string(),
			"".to_string(),
			"b".to_string(),
		];

		let solver = Day6Solver {};
		assert_eq!(solver.solve(&input, true), "6");
	}

	#[bench]
	fn bench_part_one(bencher: &mut Bencher) {
		let input = read_lines("src/day06/input.txt");
		let solver = Day6Solver {};
		bencher.iter(|| solver.solve(&input, false));
	}

	#[bench]
	fn bench_part_two(bencher: &mut Bencher) {
		let input = read_lines("src/day06/input.txt");
		let solver = Day6Solver {};
		bencher.iter(|| solver.solve(&input, true));
	}
}
