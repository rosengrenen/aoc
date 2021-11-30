use std::collections::VecDeque;

use aoc_util::{Solver, SolverOutput};
use hashbrown::HashSet;

#[derive(Default)]
pub struct Day22;

impl Solver for Day22 {
	fn part_one(&self, input: &str) -> SolverOutput {
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

		SolverOutput::Num(deck_value(&p1_deck))
	}

	fn part_two(&self, input: &str) -> SolverOutput {
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
			} else if p1_card > p2_card {
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

		SolverOutput::Num(deck_value(&p1_deck))
	}
}

fn deck_value(p: &VecDeque<i64>) -> i64 {
	p.iter()
		.rev()
		.enumerate()
		.fold(0, |prev, (i, cur)| prev + (i as i64 + 1) * cur)
}

fn parse_decks(input: &str) -> (VecDeque<i64>, VecDeque<i64>) {
	let (p1, p2) = input.split_once("\n\n").unwrap();

	let p1 = p1.lines().skip(1).map(|l| l.parse().unwrap()).collect();
	let p2 = p2.lines().skip(1).map(|l| l.parse().unwrap()).collect();
	(p1, p2)
}

fn play_recurse(mut p1_deck: VecDeque<i64>, mut p2_deck: VecDeque<i64>) -> bool {
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
		} else if p1_card > p2_card {
			p1_deck.push_back(p1_card);
			p1_deck.push_back(p2_card);
		} else {
			p2_deck.push_back(p2_card);
			p2_deck.push_back(p1_card);
		}
	}

	!p1_deck.is_empty()
}
