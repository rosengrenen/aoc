use std::collections::LinkedList;

use hashbrown::HashSet;

use crate::lib::Solver;

pub struct Day22Solver;

impl Solver for Day22Solver {
	fn solve_part_one(&self, input: &str) -> i64 {
		let (mut p1_deck, mut p2_deck) = parse_decks(input);
		while !p1_deck.is_empty() && !p2_deck.is_empty() {
			let p1_card = p1_deck.pop_front().unwrap();
			let p2_card = p2_deck.pop_front().unwrap();
			if p1_card > p2_card {
				p1_deck.push_back(p1_card);
				p1_deck.push_back(p2_card);
			} else {
				p2_deck.push_back(p2_card);
				p2_deck.push_back(p1_card);
			}
		}

		if p1_deck.is_empty() {
			p1_deck = p2_deck;
		}

		deck_value(&p1_deck)
	}

	fn solve_part_two(&self, input: &str) -> i64 {
		let (mut p1_deck, mut p2_deck) = parse_decks(input);
		while !p1_deck.is_empty() && !p2_deck.is_empty() {
			let p1_card = p1_deck.pop_front().unwrap();
			let p2_card = p2_deck.pop_front().unwrap();
			if p1_deck.len() as i64 >= p1_card && p2_deck.len() as i64 >= p2_card {
				let new_p1 = p1_deck.iter().cloned().take(p1_card as usize).collect();
				let new_p2 = p2_deck.iter().cloned().take(p2_card as usize).collect();
				if play_recurse(new_p1, new_p2) {
					p1_deck.push_back(p1_card);
					p1_deck.push_back(p2_card);
				} else {
					p2_deck.push_back(p2_card);
					p2_deck.push_back(p1_card);
				}
			} else {
				if p1_card > p2_card {
					p1_deck.push_back(p1_card);
					p1_deck.push_back(p2_card);
				} else {
					p2_deck.push_back(p2_card);
					p2_deck.push_back(p1_card);
				}
			}
		}

		if p1_deck.is_empty() {
			p1_deck = p2_deck;
		}

		deck_value(&p1_deck)
	}
}

fn deck_value(p: &LinkedList<i64>) -> i64 {
	p.iter()
		.rev()
		.enumerate()
		.fold(0, |prev, (i, cur)| prev + (i as i64 + 1) * cur)
}

fn parse_decks(input: &str) -> (LinkedList<i64>, LinkedList<i64>) {
	let (p1, p2) = input.split_once("\n\n").unwrap();

	let p1 = p1.lines().skip(1).map(|l| l.parse().unwrap()).collect();
	let p2 = p2.lines().skip(1).map(|l| l.parse().unwrap()).collect();
	(p1, p2)
}

fn play_recurse(mut p1_deck: LinkedList<i64>, mut p2_deck: LinkedList<i64>) -> bool {
	let mut prev_games = HashSet::new();
	while !p1_deck.is_empty() && !p2_deck.is_empty() {
		let decks_pair = (p1_deck.clone(), p2_deck.clone());
		if prev_games.contains(&decks_pair) {
			return true;
		} else {
			prev_games.insert(decks_pair);
		}

		let p1_card = p1_deck.pop_front().unwrap();
		let p2_card = p2_deck.pop_front().unwrap();
		if p1_deck.len() as i64 >= p1_card && p2_deck.len() as i64 >= p2_card {
			let new_p1_deck = p1_deck.iter().cloned().take(p1_card as usize).collect();
			let new_p2_deck = p2_deck.iter().cloned().take(p2_card as usize).collect();
			if play_recurse(new_p1_deck, new_p2_deck) {
				p1_deck.push_back(p1_card);
				p1_deck.push_back(p2_card);
			} else {
				p2_deck.push_back(p2_card);
				p2_deck.push_back(p1_card);
			}
		} else {
			if p1_card > p2_card {
				p1_deck.push_back(p1_card);
				p1_deck.push_back(p2_card);
			} else {
				p2_deck.push_back(p2_card);
				p2_deck.push_back(p1_card);
			}
		}
	}

	!p1_deck.is_empty()
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

	#[bench]
	fn bench_parse_decks(bencher: &mut Bencher) {
		let input = fetch_input(22);
		bencher.iter(|| parse_decks(&input));
	}

	#[bench]
	fn bench_part_one(bencher: &mut Bencher) {
		let input = fetch_input(22);
		let solver = Day22Solver {};
		bencher.iter(|| solver.solve_part_one(&input));
	}

	#[bench]
	fn bench_part_two(bencher: &mut Bencher) {
		let input = fetch_input(22);
		let solver = Day22Solver {};
		bencher.iter(|| solver.solve_part_two(&input));
	}
}
