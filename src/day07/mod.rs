use crate::lib::Solver;
use regex::Regex;
use std::collections::{HashMap, HashSet};

pub struct Day7Solver;

impl Solver for Day7Solver {
	fn solve(&self, lines: &[String], part_two: bool) -> String {
		lazy_static! {
			static ref OUTER_REGEX: Regex = Regex::new("^(.*?) bags").unwrap();
			static ref INNER_REGEX: Regex = Regex::new("(\\d+) (.*?) bags?").unwrap();
		}
		if !part_two {
			// Bag -> List of bags that contain Bag
			let mut bag_map = HashMap::new();
			for line in lines.iter() {
				let bag_color = &OUTER_REGEX.captures(line).unwrap()[1];
				for inner_captures in INNER_REGEX.captures_iter(line) {
					let container_bag_color = inner_captures[2].to_owned();
					let entry = bag_map.entry(container_bag_color).or_insert(Vec::new());
					entry.push(bag_color.to_owned());
				}
			}
			let mut set = HashSet::new();
			find_bags_that_contains_bag(&bag_map, "shiny gold", &mut set);
			set.len().to_string()
		} else {
			// Bag -> List of bags in Bag
			let mut bag_map = HashMap::new();
			for line in lines.iter() {
				let bag_color = OUTER_REGEX.captures(line).unwrap()[1].to_owned();
				let bags_in_bag: Vec<_> = INNER_REGEX
					.captures_iter(line)
					.map(|inner_captures| {
						let container_bag_count: i64 = inner_captures[1].parse().unwrap();
						let container_bag_color = inner_captures[2].to_owned();
						(container_bag_count, container_bag_color)
					})
					.collect();
				bag_map.insert(bag_color, bags_in_bag);
			}
			count_bags_in_bag(&bag_map, "shiny gold").to_string()
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
	use crate::lib::read_lines;
	use test::Bencher;

	#[test]
	fn part_one_test_cases() {
		let input: Vec<String> = vec![
			"light red bags contain 1 bright white bag, 2 muted yellow bags.".to_string(),
			"dark orange bags contain 3 bright white bags, 4 muted yellow bags.".to_string(),
			"bright white bags contain 1 shiny gold bag.".to_string(),
			"muted yellow bags contain 2 shiny gold bags, 9 faded blue bags.".to_string(),
			"shiny gold bags contain 1 dark olive bag, 2 vibrant plum bags.".to_string(),
			"dark olive bags contain 3 faded blue bags, 4 dotted black bags.".to_string(),
			"vibrant plum bags contain 5 faded blue bags, 6 dotted black bags.".to_string(),
			"faded blue bags contain no other bags.".to_string(),
			"dotted black bags contain no other bags.".to_string(),
		];

		let solver = Day7Solver {};
		assert_eq!(solver.solve(&input, false), "4");
	}

	#[test]
	fn part_two_test_cases() {
		let input: Vec<String> = vec![
			"light red bags contain 1 bright white bag, 2 muted yellow bags.".to_string(),
			"dark orange bags contain 3 bright white bags, 4 muted yellow bags.".to_string(),
			"bright white bags contain 1 shiny gold bag.".to_string(),
			"muted yellow bags contain 2 shiny gold bags, 9 faded blue bags.".to_string(),
			"shiny gold bags contain 1 dark olive bag, 2 vibrant plum bags.".to_string(),
			"dark olive bags contain 3 faded blue bags, 4 dotted black bags.".to_string(),
			"vibrant plum bags contain 5 faded blue bags, 6 dotted black bags.".to_string(),
			"faded blue bags contain no other bags.".to_string(),
			"dotted black bags contain no other bags.".to_string(),
		];

		let input2: Vec<String> = vec![
			"shiny gold bags contain 2 dark red bags.".to_string(),
			"dark red bags contain 2 dark orange bags.".to_string(),
			"dark orange bags contain 2 dark yellow bags.".to_string(),
			"dark yellow bags contain 2 dark green bags.".to_string(),
			"dark green bags contain 2 dark blue bags.".to_string(),
			"dark blue bags contain 2 dark violet bags.".to_string(),
			"dark violet bags contain no other bags.".to_string(),
		];

		let solver = Day7Solver {};
		assert_eq!(solver.solve(&input, true), "32");
		assert_eq!(solver.solve(&input2, true), "126");
	}

	#[bench]
	fn bench_part_one(bencher: &mut Bencher) {
		let input = read_lines("src/day07/input.txt");
		let solver = Day7Solver {};
		bencher.iter(|| solver.solve(&input, false));
	}

	#[bench]
	fn bench_part_two(bencher: &mut Bencher) {
		let input = read_lines("src/day07/input.txt");
		let solver = Day7Solver {};
		bencher.iter(|| solver.solve(&input, true));
	}
}
