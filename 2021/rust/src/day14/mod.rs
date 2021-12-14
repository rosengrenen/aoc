use std::collections::HashMap;

use aoc_util::{Solver, SolverOutput};

#[derive(Default)]
pub struct Day14;

impl Solver for Day14 {
	fn part_one(&self, input: &str) -> SolverOutput {
		let (mut tmpl, insertion_rules) = parse_(input);

		println!("{:?} {:?}", tmpl, insertion_rules);

		for _ in 0..10 {
			let mut i = 0;
			while i < tmpl.len() - 1 {
				let first = tmpl[i];
				let second = tmpl[i + 1];
				if let Some(insertion) = insertion_rules.get(&(first, second)) {
					i += 1;
					tmpl.insert(i, *insertion);
				}

				i += 1;
			}

			println!("{}", tmpl.len());
		}

		let mut counts = [0; 26];

		for c in tmpl {
			counts[(c - b'A') as usize] += 1;
		}

		let min = counts.iter().filter(|&&c| c > 0).min().unwrap();
		let max = counts.iter().filter(|&&c| c > 0).max().unwrap();

		SolverOutput::Num(max - min)
	}

	fn part_two(&self, input: &str) -> SolverOutput {
		let (tmpl, insertion_rules) = parse_(input);

		SolverOutput::Num(0)
	}
}

fn parse_(input: &str) -> (Vec<u8>, HashMap<(u8, u8), u8>) {
	let (tmpl, insertion_rules) = input.split_once("\n\n").unwrap();
	let tmpl = tmpl.as_bytes().iter().map(|b| *b).collect();
	let insertion_rules = insertion_rules
		.lines()
		.map(|line| line.split_once(" -> ").unwrap())
		.map(|(from, to)| {
			let first = from.as_bytes()[0];
			let second = from.as_bytes()[1];
			let to = to.as_bytes()[0];

			((first, second), to)
		})
		.collect();

	(tmpl, insertion_rules)
}

#[cfg(test)]
mod benches {
	use super::*;
	use aoc_util::get_input;
	use test::{black_box, Bencher};

	#[bench]
	fn parse(bencher: &mut Bencher) {
		let input = get_input(2021, 14).unwrap();
		bencher.iter(|| parse_(black_box(&input)));
	}
}
