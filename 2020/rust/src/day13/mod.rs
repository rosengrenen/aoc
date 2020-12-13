use crate::lib::Solver;
use num::integer::lcm;

pub struct Day13Solver;

impl Solver for Day13Solver {
	fn solve_part_one(&self, input: &str) -> i64 {
		let (first_poss, buses) = parse_buses(input);
		// Here index is used as the wait time for the first bus of the specific interval to depart
		let mut first_avail = Bus {
			index: std::i64::MAX,
			..buses[0]
		};
		for Bus { interval, .. } in buses {
			let wait_time = ((first_poss / interval) + 1) * interval - first_poss;
			if wait_time < first_avail.index {
				first_avail = Bus {
					interval,
					index: wait_time,
				};
			}
		}

		first_avail.interval * first_avail.index
	}

	fn solve_part_two(&self, input: &str) -> i64 {
		let (first_poss, mut buses) = parse_buses(input);
		buses.sort_by(|left, right| right.interval.cmp(&left.interval));
		let first_avail = (first_poss / buses[0].interval + 1) * buses[0].interval;
		// Have to subtract with index since the list is sorted and 0th index is
		// not necessarily first anymore
		let mut t = first_avail - buses[0].index;
		loop {
			let mut found = true;
			let mut smallest_step_forward = buses[0].interval;
			for &Bus { interval, index } in buses.iter().skip(1) {
				if (t + index) % interval == 0 {
					smallest_step_forward = lcm(smallest_step_forward, interval);
				} else {
					found = false;
					break;
				}
			}
			if found {
				return t;
			}
			t += smallest_step_forward;
		}
	}
}

#[derive(Clone, Copy, Debug)]
struct Bus {
	interval: i64,
	index: i64,
}

fn parse_buses(input: &str) -> (i64, Vec<Bus>) {
	let lines: Vec<&str> = input.lines().collect();

	let first_poss = lines[0].parse().unwrap();
	let mut bus_intervals = Vec::new();
	for (i, s) in lines[1].split(',').enumerate() {
		if s != "x" {
			bus_intervals.push(Bus {
				interval: s.parse().unwrap(),
				index: i as i64,
			})
		}
	}
	(first_poss, bus_intervals)
}

#[cfg(test)]
mod tests {
	use super::*;
	use test::Bencher;

	#[test]
	fn part_one_test_cases() {
		let input = include_str!("input.test1.txt");
		let solver = Day13Solver {};
		assert_eq!(solver.solve_part_one(input), 295);
	}

	#[test]
	fn part_two_test_cases() {
		let input = include_str!("input.test1.txt");
		let input2 = include_str!("input.test2.txt");
		let input3 = include_str!("input.test3.txt");
		let input4 = include_str!("input.test4.txt");
		let input5 = include_str!("input.test5.txt");
		let input6 = include_str!("input.test6.txt");
		let solver = Day13Solver {};
		assert_eq!(solver.solve_part_two(input), 1068781);
		assert_eq!(solver.solve_part_two(input2), 3417);
		assert_eq!(solver.solve_part_two(input3), 754018);
		assert_eq!(solver.solve_part_two(input4), 779210);
		assert_eq!(solver.solve_part_two(input5), 1261476);
		assert_eq!(solver.solve_part_two(input6), 1202161486);
	}

	#[bench]
	fn bench_parse_buses(bencher: &mut Bencher) {
		let input = include_str!("input.txt");
		bencher.iter(|| parse_buses(input));
	}

	#[bench]
	fn bench_part_one(bencher: &mut Bencher) {
		let input = include_str!("input.txt");
		let solver = Day13Solver {};
		bencher.iter(|| solver.solve_part_one(input));
	}

	#[bench]
	fn bench_part_two(bencher: &mut Bencher) {
		let input = include_str!("input.txt");
		let solver = Day13Solver {};
		bencher.iter(|| solver.solve_part_two(input));
	}
}
