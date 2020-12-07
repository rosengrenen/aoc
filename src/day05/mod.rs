use crate::lib::Solver;

pub struct Day5Solver;

impl Solver for Day5Solver {
	fn solve(&self, input: &str, part_two: bool) -> i64 {
		if !part_two {
			input.lines().map(|line| get_seat_id(line)).max().unwrap()
		} else {
			let mut seat_ids: Vec<i64> = input.lines().map(|line| get_seat_id(line)).collect();
			seat_ids.sort_unstable();
			let mut prev_id = -999;
			for &current_id in seat_ids.iter() {
				if current_id - prev_id == 2 {
					break;
				}

				prev_id = current_id;
			}

			prev_id + 1
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
		let input = include_str!("input.txt");
		let solver = Day5Solver {};
		bencher.iter(|| solver.solve(&input, false));
	}

	#[bench]
	fn bench_part_two(bencher: &mut Bencher) {
		let input = include_str!("input.txt");
		let solver = Day5Solver {};
		bencher.iter(|| solver.solve(&input, true));
	}
}
