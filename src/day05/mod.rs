use crate::lib::Solver;

pub struct Day5Solver;

impl Solver for Day5Solver {
	fn solve(&self, lines: &[String], part_two: bool) -> String {
		if !part_two {
			lines
				.iter()
				.map(|line| get_seat_id(line))
				.max()
				.unwrap()
				.to_string()
		} else {
			let mut seat_ids: Vec<i64> = lines.iter().map(|line| get_seat_id(line)).collect();
			seat_ids.sort_unstable();
			let mut prev_id = -999;
			for &current_id in seat_ids.iter() {
				if current_id - prev_id == 2 {
					break;
				}

				prev_id = current_id;
			}

			(prev_id + 1).to_string()
		}
	}
}

fn get_seat_id(partition_data: &str) -> i64 {
	let mut seat_id = 0;
	for &c in partition_data.as_bytes() {
		seat_id <<= 1;
		if c == b'B' || c == b'R' {
			seat_id |= 1;
		}
	}
	seat_id
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::lib::read_lines;
	use test::Bencher;

	#[test]
	fn part_one_test_cases() {
		assert_eq!(get_seat_id("FBFBBFFRLR"), 357);
		assert_eq!(get_seat_id("BFFFBBFRRR"), 567);
		assert_eq!(get_seat_id("FFFBBBFRRR"), 119);
		assert_eq!(get_seat_id("BBFFBBFRLL"), 820);
	}

	#[bench]
	fn bench_part_one(bencher: &mut Bencher) {
		let input = read_lines("src/day05/input.txt");
		let solver = Day5Solver {};
		bencher.iter(|| solver.solve(&input, false));
	}

	#[bench]
	fn bench_part_two(bencher: &mut Bencher) {
		let input = read_lines("src/day05/input.txt");
		let solver = Day5Solver {};
		bencher.iter(|| solver.solve(&input, true));
	}
}
