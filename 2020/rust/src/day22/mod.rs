use std::collections::LinkedList;

use hashbrown::HashSet;

use crate::lib::Solver;

pub struct Day22Solver;

impl Solver for Day22Solver {
	fn solve_part_one(&self, input: &str) -> i64 {
		let (mut p1, mut p2) = parse(input);
		while !p1.is_empty() && !p2.is_empty() {
			let p1_card = p1.pop_front().unwrap();
			let p2_card = p2.pop_front().unwrap();
			if p1_card > p2_card {
				p1.push_back(p1_card);
				p1.push_back(p2_card);
			} else {
				p2.push_back(p2_card);
				p2.push_back(p1_card);
			}
		}

		if p1.is_empty() {
			p1 = p2;
		}

		c(&p1)
	}

	fn solve_part_two(&self, input: &str) -> i64 {
		let (mut p1, mut p2) = parse(input);
		while !p1.is_empty() && !p2.is_empty() {
			let p1_card = p1.pop_front().unwrap();
			let p2_card = p2.pop_front().unwrap();
			if p1.len() as i64 >= p1_card && p2.len() as i64 >= p2_card {
				let new_p1 = p1.iter().cloned().take(p1_card as usize).collect();
				let new_p2 = p2.iter().cloned().take(p2_card as usize).collect();
				if play_recurse(new_p1, new_p2) {
					p1.push_back(p1_card);
					p1.push_back(p2_card);
				} else {
					p2.push_back(p2_card);
					p2.push_back(p1_card);
				}
			} else {
				if p1_card > p2_card {
					p1.push_back(p1_card);
					p1.push_back(p2_card);
				} else {
					p2.push_back(p2_card);
					p2.push_back(p1_card);
				}
			}
		}

		if p1.is_empty() {
			p1 = p2;
		}

		c(&p1)
	}
}

fn c(p: &LinkedList<i64>) -> i64 {
	p.iter()
		.rev()
		.enumerate()
		.fold(0, |prev, (i, cur)| prev + (i as i64 + 1) * cur)
}

fn parse(input: &str) -> (LinkedList<i64>, LinkedList<i64>) {
	let (p1, p2) = input.split_once("\n\n").unwrap();

	let p1 = p1.lines().skip(1).map(|l| l.parse().unwrap()).collect();
	let p2 = p2.lines().skip(1).map(|l| l.parse().unwrap()).collect();
	(p1, p2)
}

fn play_recurse(mut p1: LinkedList<i64>, mut p2: LinkedList<i64>) -> bool {
	let mut prev_games = HashSet::new();
	while !p1.is_empty() && !p2.is_empty() {
		let c1 = p1.clone();
		let c2 = p2.clone();
		let c = (c1, c2);
		if prev_games.contains(&c) {
			return true;
		} else {
			prev_games.insert(c);
		}
		let p1_card = p1.pop_front().unwrap();
		let p2_card = p2.pop_front().unwrap();
		if p1.len() as i64 >= p1_card && p2.len() as i64 >= p2_card {
			let new_p1 = p1.iter().cloned().take(p1_card as usize).collect();
			let new_p2 = p2.iter().cloned().take(p2_card as usize).collect();
			if play_recurse(new_p1, new_p2) {
				p1.push_back(p1_card);
				p1.push_back(p2_card);
			} else {
				p2.push_back(p2_card);
				p2.push_back(p1_card);
			}
		} else {
			if p1_card > p2_card {
				p1.push_back(p1_card);
				p1.push_back(p2_card);
			} else {
				p2.push_back(p2_card);
				p2.push_back(p1_card);
			}
		}
	}

	if p1.is_empty() {
		false
	} else {
		true
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::lib::fetch_input;
	use test::Bencher;

	#[test]
	fn part_one_test_cases() {
		let input = include_str!("input.test1.txt");
		let solver = Day22Solver {};
		assert_eq!(solver.solve_part_one(input), 306);
	}

	#[test]
	fn part_two_test_cases() {
		let input = include_str!("input.test1.txt");
		let solver = Day22Solver {};
		assert_eq!(solver.solve_part_two(input), 291);
	}

	// #[bench]
	// fn bench_parse_foods(bencher: &mut Bencher) {
	// 	let input = fetch_input(22);
	// 	bencher.iter(|| parse_foods(&input));
	// }

	// #[bench]
	// fn bench_part_one(bencher: &mut Bencher) {
	// 	let input = fetch_input(22);
	// 	let solver = Day22Solver {};
	// 	bencher.iter(|| solver.solve_part_one(&input));
	// }

	// #[bench]
	// fn bench_part_two(bencher: &mut Bencher) {
	// 	let input = fetch_input(22);
	// 	let solver = Day22Solver {};
	// 	bencher.iter(|| solver.solve_part_two(&input));
	// }
}
