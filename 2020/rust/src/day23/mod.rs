use hashbrown::HashSet;
use std::collections::{LinkedList, VecDeque};

use crate::lib::Solver;

pub struct Day23Solver;

impl Solver for Day23Solver {
	fn solve_part_one(&self, input: &str) -> i64 {
		let mut input = parse(input);
		let mut numbers = input.clone();
		numbers.sort_unstable();

		// println!("{:?}", input);
		let mut current = input[0];

		for m in 0..100 {
			println!("\n-- move {} --", m + 1);
			print!("cups: ");
			for cup in input.iter() {
				if *cup == current {
					print!("({}) ", cup);
				} else {
					print!("{} ", cup);
				}
			}
			print!("\n");

			let mut a = 0;
			let mut b = 0;
			let mut c = 0;
			let mut i = 0;
			loop {
				if input[i] == current {
					a = input[(i + 1) % input.len()];
					b = input[(i + 2) % input.len()];
					c = input[(i + 3) % input.len()];
					input = input
						.into_iter()
						.filter(|&n| n != a && n != b && n != c)
						.collect();
					break;
				}

				i = i + 1 % input.len();
			}

			println!("pick up: {}, {}, {}", a, b, c);
			let mut destination = 0;
			let mut i = 0;
			loop {
				if numbers[i] == current {
					loop {
						if i == 0 {
							i = numbers.len() - 1;
						} else {
							i -= 1;
						}

						// println!("{}, {} {} {}", numbers[i], a, b, c);

						if numbers[i] != a && numbers[i] != b && numbers[i] != c {
							destination = numbers[i];
							break;
						}
					}

					break;
				}

				i += 1;
			}
			println!("destination: {}", destination);
			let mut i = 0;
			loop {
				if input[i] == destination {
					input.insert(i + 1, a);
					input.insert(i + 2, b);
					input.insert(i + 3, c);
					break;
				}
				i += 1;
			}
			println!("{:?}", input);
			let mut i = 0;
			loop {
				if input[i] == current {
					current = input[(i + 1) % input.len()];
					break;
				}

				i += 1;
			}
		}

		let mut i = 0;
		loop {
			if input[i] == 1 {
				break;
			}
			i += 1;
		}
		i += 1;
		let mut num = 0;
		loop {
			if input[i] == 1 {
				break;
			}

			num *= 10;
			num += input[i];

			i = (i + 1) % input.len();
		}

		num
	}

	fn solve_part_two(&self, input: &str) -> i64 {
		let mut input = parse(input);
		// let mut input: VecDeque<_> = input.into_iter().collect();
		for i in 10..=1_000_000 {
			input.push(i);
		}
		// let mut numbers = input.clone();
		// numbers.sort_unstable();

		// println!("{:?}", input);
		let mut current = input[0];
		let mut current_index = 0;

		for m in 0..10_000_000 {
			if m % 1000 == 0 {
				println!("\n-- move {} --", m + 1);
			}
			// print!("cups: ");
			// for cup in input.iter() {
			// 	if *cup == current {
			// 		print!("({}) ", cup);
			// 	} else {
			// 		print!("{} ", cup);
			// 	}
			// }
			// print!("\n");

			let mut a = input[(current_index + 1) % input.len()];
			input.remove((current_index + 1) % input.len());
			let mut b = input[(current_index + 1) % input.len()];
			let mut c = input[(current_index + 1) % input.len()];
			let mut new_current = input[(current_index + 4) % input.len()];
			// TODO remove els and adjust index

			// println!("pick up: {}, {}, {}", a, b, c);
			let mut destination = current;
			loop {
				if destination == 0 {
					destination = 1_000_000;
				} else {
					destination -= 1;
				}

				// println!("{}, {} {} {}", numbers[i], a, b, c);

				if c != a && c != b && c != c {
					break;
				}
			}

			// println!("destination: {}", destination);
			let mut i = 0;
			loop {
				if input[i] == destination {
					input.insert(i + 1, a);
					input.insert(i + 2, b);
					input.insert(i + 3, c);
					break;
				}
				i += 1;
			}
			// println!("{:?}", input);
			current = new_current;
			current_index = new_current_index;
			// let mut i = 0;
			// loop {
			// 	if input[i] == current {
			// 		current = input[(i + 1) % input.len()];
			// 		break;
			// 	}

			// 	i += 1;
			// }
		}

		let mut i = 0;
		loop {
			if input[i] == 1 {
				break;
			}
			i += 1;
		}
		i += 1;
		let mut num = 0;
		loop {
			if input[i] == 1 {
				break;
			}

			num *= 10;
			num += input[i];

			i = (i + 1) % input.len();
		}

		num
	}
}

fn parse(input: &str) -> Vec<i64> {
	input
		.as_bytes()
		.iter()
		.map(|c| {
			println!("{} {}", c, b'0');
			(*c - b'0') as i64
		})
		.collect()
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::lib::fetch_input;
	use test::Bencher;

	#[test]
	fn part_one_test_cases() {
		let input = include_str!("input.test1.txt");
		let solver = Day23Solver {};
		assert_eq!(solver.solve_part_one(input), 67384529);
	}

	#[test]
	fn part_two_test_cases() {
		let input = include_str!("input.test1.txt");
		let solver = Day23Solver {};
		assert_eq!(solver.solve_part_two(input), 149245887792);
	}

	// #[bench]
	// fn bench_parse(bencher: &mut Bencher) {
	// 	let input = fetch_input(23);
	// 	bencher.iter(|| parse(&input));
	// }

	// #[bench]
	// fn bench_part_one(bencher: &mut Bencher) {
	// 	let input = fetch_input(23);
	// 	let solver = Day23Solver {};
	// 	bencher.iter(|| solver.solve_part_one(&input));
	// }

	// #[bench]
	// fn bench_part_two(bencher: &mut Bencher) {
	// 	let input = fetch_input(23);
	// 	let solver = Day23Solver {};
	// 	bencher.iter(|| solver.solve_part_two(&input));
	// }
}
