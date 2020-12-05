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
	let row_partition_data = &partition_data[0..7];
	let column_partition_data = &partition_data[7..10];

	let mut row_min = 0;
	let mut row_max = 127;
	for &c in row_partition_data.as_bytes().iter() {
		if c == b'F' {
			row_max -= (row_max - row_min).div_ceil(&2);
		} else {
			row_min += (row_max - row_min).div_ceil(&2);
		}
	}
	if row_min != row_max {
		panic!("Row could not be found");
	}

	let mut column_min = 0;
	let mut column_max = 7;
	for &c in column_partition_data.as_bytes().iter() {
		if c == b'L' {
			column_max -= (column_max - column_min).div_ceil(&2);
		} else {
			column_min += (column_max - column_min).div_ceil(&2);
		}
	}
	if column_min != column_max {
		panic!("Column could not be found");
	}
	8 * row_min + column_min
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
