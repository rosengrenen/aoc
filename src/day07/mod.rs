use crate::lib::Solver;
use std::collections::{HashMap, HashSet};

pub struct Day7Solver;

impl Solver for Day7Solver {
	fn solve(&self, lines: &[String], part_two: bool) -> String {
		if !part_two {
			let mut bag_map: HashMap<String, Vec<String>> = HashMap::new();
			for line in lines.iter() {
				let split: Vec<&str> = line.split(&"bags contain").collect();
				let bag = split[0].trim();
				let contains: Vec<String> = split[1]
					.trim()
					.split(",")
					.filter(|child_pag| !child_pag.contains("no other bags"))
					.map(|child_bag| {
						let mut split = child_bag.trim().split_whitespace();
						split.next().unwrap();
						split.next().unwrap().to_owned() + " " + split.next().unwrap()
					})
					.collect();
				for c in contains.iter() {
					let e = bag_map.entry(c.clone()).or_insert(Vec::new());
					e.push(bag.to_string());
				}
			}
			let mut set = HashSet::new();
			find_recursive(&bag_map, "shiny gold", &mut set);
			set.len().to_string()
		} else {
			let mut bag_map: HashMap<String, Vec<(i64, String)>> = HashMap::new();
			for line in lines.iter() {
				let split: Vec<&str> = line.split(&"bags contain").collect();
				let bag = split[0].trim();
				let contains: Vec<(i64, String)> = split[1]
					.trim()
					.split(",")
					.filter(|child_pag| !child_pag.contains("no other bags"))
					.map(|child_bag| {
						let mut split = child_bag.trim().split_whitespace();
						(
							split.next().unwrap().parse().unwrap(),
							split.next().unwrap().to_owned() + " " + split.next().unwrap(),
						)
					})
					.collect();
				bag_map.insert(bag.to_owned(), contains);
			}
			(count_recursive(&bag_map, "shiny gold") - 1).to_string()
		}
	}
}

fn find_recursive(bags: &HashMap<String, Vec<String>>, bag: &str, found: &mut HashSet<String>) {
	if let Some(bs) = bags.get(bag) {
		for b in bs.iter() {
			found.insert(b.clone());
			find_recursive(bags, b, found)
		}
	}
}

fn count_recursive(bags: &HashMap<String, Vec<(i64, String)>>, bag: &str) -> i64 {
	if let Some(bs) = bags.get(bag) {
		if bs.is_empty() {
			return 1;
		}

		let mut sum = 1;
		for (c, b) in bs.iter() {
			sum += c * count_recursive(bags, b);
		}

		return sum;
	}
	panic!("duh")
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
