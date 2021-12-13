use std::collections::{HashMap, HashSet};

use aoc_util::{Solver, SolverOutput};

#[derive(Default)]
pub struct Day12;

impl Solver for Day12 {
	fn part_one(&self, input: &str) -> SolverOutput {
		let a = parse_(input);

		SolverOutput::Num(count_possible_paths(&a, "start", "end", HashSet::new()))
	}

	fn part_two(&self, input: &str) -> SolverOutput {
		let a = parse_(input);

		SolverOutput::Num(count_possible_paths2(&a, "start", "end", HashMap::new()))
	}
}

fn count_possible_paths<'a>(
	paths: &Paths,
	from: &'a str,
	goal: &str,
	mut already_visited: HashSet<&'a str>,
) -> i64 {
	if from == goal {
		return 1;
	}

	let is_small_cave = from.as_bytes()[0].is_ascii_lowercase();
	if is_small_cave {
		already_visited.insert(&from);
	}

	let mut possible_paths = 0;
	for &to in paths.get(from).unwrap() {
		if already_visited.contains(to) {
			continue;
		}

		possible_paths += count_possible_paths(paths, to, goal, already_visited.clone());
	}

	possible_paths
}

fn count_possible_paths2<'a>(
	paths: &Paths,
	from: &'a str,
	goal: &str,
	mut already_visited: HashMap<&'a str, i64>,
) -> i64 {
	if from == goal {
		return 1;
	}

	let is_small_cave = from.as_bytes()[0].is_ascii_lowercase();
	if is_small_cave {
		let entry = already_visited.entry(&from).or_default();
		*entry += 1;
	}

	let has_visited_twice = already_visited.iter().any(|(_, &visits)| visits == 2);
	let mut possible_paths = 0;
	for &to in paths.get(from).unwrap() {
		if to == "start" || has_visited_twice && already_visited.contains_key(to) {
			continue;
		}

		possible_paths += count_possible_paths2(paths, to, goal, already_visited.clone());
	}

	possible_paths
}

type Paths<'a> = HashMap<&'a str, Vec<&'a str>>;

fn parse_<'a>(input: &'a str) -> Paths<'a> {
	let mut paths = HashMap::new();
	for line in input.lines() {
		let (from, to) = line.split_once("-").unwrap();
		let mut insert_edge = |from, to| {
			let entry = paths.entry(from).or_insert_with(Vec::new);
			entry.push(to);
		};

		insert_edge(from, to);
		insert_edge(to, from);
	}

	paths
}

#[cfg(test)]
mod benches {
	use super::*;
	use aoc_util::get_input;
	use test::{black_box, Bencher};

	#[bench]
	fn parse(bencher: &mut Bencher) {
		let input = get_input(2021, 12).unwrap();
		bencher.iter(|| parse_(black_box(&input)));
	}
}
