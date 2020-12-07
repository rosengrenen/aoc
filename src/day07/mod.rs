use crate::lib::Solver;
use regex::Regex;
use std::collections::{HashMap, HashSet};

pub struct Day7Solver;

impl Solver for Day7Solver {
	fn solve(&self, input: &str, part_two: bool) -> i64 {
		let outer_regex: Regex = Regex::new("^(.*?) bags").unwrap();
		let inner_regex: Regex = Regex::new("(\\d+) (.*?) bags?").unwrap();
		if !part_two {
			// Bag -> List of bags that contain Bag
			let mut bag_map = HashMap::new();
			for line in input.lines() {
				let bag_color = &outer_regex.captures(line).unwrap()[1];
				for inner_captures in inner_regex.captures_iter(line) {
					let container_bag_color = inner_captures[2].to_owned();
					let entry = bag_map.entry(container_bag_color).or_insert_with(Vec::new);
					entry.push(bag_color.to_owned());
				}
			}
			let mut set = HashSet::new();
			find_bags_that_contains_bag(&bag_map, "shiny gold", &mut set);
			set.len() as i64
		} else {
			// Bag -> List of bags in Bag
			let mut bag_map = HashMap::new();
			for line in input.lines() {
				let bag_color = outer_regex.captures(line).unwrap()[1].to_owned();
				let bags_in_bag: Vec<_> = inner_regex
					.captures_iter(line)
					.map(|inner_captures| {
						let container_bag_count: i64 = inner_captures[1].parse().unwrap();
						let container_bag_color = inner_captures[2].to_owned();
						(container_bag_count, container_bag_color)
					})
					.collect();
				bag_map.insert(bag_color, bags_in_bag);
			}
			count_bags_in_bag(&bag_map, "shiny gold")
		}
	}
}

fn find_bags_that_contains_bag(
	bags_map: &HashMap<String, Vec<String>>,
	bag: &str,
	result_acc: &mut HashSet<String>,
) {
	if let Some(bags_that_contain_bag) = bags_map.get(bag) {
		for bag_that_contains_bag in bags_that_contain_bag.iter() {
			result_acc.insert(bag_that_contains_bag.clone());
			find_bags_that_contains_bag(bags_map, bag_that_contains_bag, result_acc)
		}
	}
}

fn count_bags_in_bag(bags_map: &HashMap<String, Vec<(i64, String)>>, bag: &str) -> i64 {
	let bags_in_bag = bags_map.get(bag).unwrap();
	bags_in_bag
		.iter()
		.map(|(child_bag_count, child_bag_color)| {
			child_bag_count * (1 + count_bags_in_bag(bags_map, child_bag_color))
		})
		.sum()
}

#[cfg(test)]
mod tests {
	use super::*;
	use test::Bencher;

	#[test]
	fn part_one_test_cases() {
		let input = include_str!("input.test1.txt");
		let solver = Day7Solver {};
		assert_eq!(solver.solve(&input, false), 4);
	}

	#[test]
	fn part_two_test_cases() {
		let input = include_str!("input.test1.txt");
		let input2 = include_str!("input.test2.txt");
		let solver = Day7Solver {};
		assert_eq!(solver.solve(&input, true), 32);
		assert_eq!(solver.solve(&input2, true), 126);
	}

	#[bench]
	fn bench_part_one(bencher: &mut Bencher) {
		let input = include_str!("input.txt");
		let solver = Day7Solver {};
		bencher.iter(|| solver.solve(&input, false));
	}

	#[bench]
	fn bench_part_two(bencher: &mut Bencher) {
		let input = include_str!("input.txt");
		let solver = Day7Solver {};
		bencher.iter(|| solver.solve(&input, true));
	}
}
