use crate::lib::Solver;
use num::integer::lcm;

pub struct Day13Solver;

impl Solver for Day13Solver {
	fn solve_part_one(&self, input: &str) -> i64 {
		let (early_off, buses) = parse(input);
		let mut bus_number = buses[0];
		let mut earliest = std::i64::MAX;
		for bus in buses {
			let a = early_off / bus;
			let b = early_off - a * bus;
			let mut c = 0;
			while c < early_off {
				c += bus;
			}
			let d = c - early_off;
			if d < earliest {
				earliest = d;
				bus_number = bus;
			}
			println!("{}", d);
		}
		earliest * bus_number
	}

	fn solve_part_two(&self, input: &str) -> i64 {
		let (early_off, buses) = parse2(input);
		println!("{:?}", buses);
		println!(
			"{} {} {} {}",
			early_off,
			early_off / buses[0].0,
			((early_off / buses[0].0) + 1) * (buses[0].0),
			0
		);
		// 100000000000000
		// 5795080000
		let mut cur_start_time = ((early_off / buses[0].0) + 1) * (buses[0].0);
		// let mut cur_start_time = ((100000000000000 / buses[0].0) + 1) * (buses[0].0);

		let mut buses_sorted = buses.clone();
		// buses_sorted.sort_by(|left, right| right.0.cmp(&left.0));
		println!("{:?}", buses_sorted);
		// buses_sorted.sort_by(|left, right| right.0.cmp(&left.0));
		println!("{:?}", buses_sorted);
		let mut cur_start_time = ((early_off / buses_sorted[0].0) + 1) * (buses_sorted[0].0);
		loop {
			// if cur_start_time % 1000 == 0 {
			// 	println!("{}", cur_start_time);
			// }
			let mut found = true;
			let mut smallest_step_forward = buses_sorted[0].0;
			let mut matched = 0;
			for &(time, index) in buses_sorted.iter().skip(1) {
				if (cur_start_time + index) % time != 0 {
					// println!("break {}", index);
					// if time > smallest_step_forward {
					// 	smallest_step_forward = ((time / buses[0].0) + 1) * (buses[0].0);
					// }
					found = false;

				// break;
				} else {
					matched += 1;
					// println!("{}", smallest_step_forward);
					smallest_step_forward = lcm(smallest_step_forward, time);
				}
			}
			if found {
				return cur_start_time;
			}
			// println!("{}", smallest_step_forward);
			println!("{}/{}", matched, buses.len());
			cur_start_time += smallest_step_forward;
			// cur_start_time += buses_sorted[0].0;
		}
		// let current = buses.iter().map(|(bus,index)|{

		// }).collect()
	}
}

fn parse(input: &str) -> (i64, Vec<i64>) {
	let lines: Vec<&str> = input.lines().collect();
	(
		lines[0].parse().unwrap(),
		lines[1]
			.split(',')
			.filter(|s| s != &"x")
			.map(|s| s.parse().unwrap())
			.collect(),
	)
}

fn parse2(input: &str) -> (i64, Vec<(i64, i64)>) {
	let lines: Vec<&str> = input.lines().collect();

	let mut v = Vec::new();
	for (i, s) in lines[1].split(',').enumerate() {
		if s != "x" {
			v.push((s.parse().unwrap(), i as i64))
		}
	}
	(lines[0].parse().unwrap(), v)
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
	fn bench_parse(bencher: &mut Bencher) {
		let input = include_str!("input.txt");
		bencher.iter(|| parse(input));
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
