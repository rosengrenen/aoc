use crate::lib::Solver;
use num::Integer;
use std::cmp;

pub struct Day5Solver;

impl Solver for Day5Solver {
	fn solve(&self, lines: Vec<String>, part_two: bool) -> String {
		if !part_two {
			lines
				.iter()
				.fold(0, |highest_seat_id, line| {
					cmp::max(highest_seat_id, get_seat_id(line))
				})
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
	i64::from_str_radix(
		&partition_data
			.replace("F", "0")
			.replace("B", "1")
			.replace("L", "0")
			.replace("R", "1"),
		2,
	)
	.unwrap()
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn part_one_test_cases() {
		assert_eq!(get_seat_id("FBFBBFFRLR"), 357);
		assert_eq!(get_seat_id("BFFFBBFRRR"), 567);
		assert_eq!(get_seat_id("FFFBBBFRRR"), 119);
		assert_eq!(get_seat_id("BBFFBBFRLL"), 820);
	}
}
