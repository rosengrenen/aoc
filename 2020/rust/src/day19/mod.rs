use hashbrown::HashMap;
use regex::Regex;

use crate::lib::Solver;

pub struct Day19Solver;

impl Solver for Day19Solver {
	fn solve_part_one(&self, input: &str) -> i64 {
		let (rules, messages) = parse(input);
		let regex = Regex::new(&create_regex(&rules, 0, 0)).unwrap();
		messages.iter().fold(
			0,
			|prev, cur| if regex.is_match(cur) { prev + 1 } else { prev },
		)
	}

	fn solve_part_two(&self, input: &str) -> i64 {
		let (mut rules, messages) = parse(input);
		rules
			.entry(8)
			.and_modify(|entry| *entry = ProductionRule::NonTerminal(vec![vec![42], vec![42, 8]]));
		rules.entry(11).and_modify(|entry| {
			*entry = ProductionRule::NonTerminal(vec![vec![42, 31], vec![42, 11, 31]])
		});

		let regex = Regex::new(&create_regex(&rules, 0, 1)).unwrap();
		messages.iter().fold(
			0,
			|prev, cur| if regex.is_match(cur) { prev + 1 } else { prev },
		)
	}
}

fn create_regex(rules: &HashMap<i64, ProductionRule>, rule: i64, recurse_levels: i64) -> String {
	let mut regex_string = String::new();

	if rule == 0 {
		regex_string.push('^');
	}

	let mut recursive = false;
	match rules.get(&rule).unwrap() {
		ProductionRule::Terminal(t) => regex_string.push(*t),
		ProductionRule::NonTerminal(ors) => {
			if ors.len() > 1 {
				regex_string.push('(');
			}
			for (i, or) in ors.iter().enumerate() {
				if i > 0 {
					regex_string.push('|');
				}

				for rule_num in or.iter() {
					if *rule_num == rule {
						regex_string.push_str(&rule.to_string());
						recursive = true;
					} else {
						regex_string.push_str(&create_regex(rules, *rule_num, recurse_levels));
					}
				}
			}
			if ors.len() > 1 {
				regex_string.push(')');
			}
		}
	}

	if recursive {
		const ITERATIONS: i64 = 5;
		for i in 1..ITERATIONS {
			regex_string = regex_string.replace(&rule.to_string(), &regex_string);
			if i == ITERATIONS - 1 {
				regex_string = regex_string.replace(&rule.to_string(), "");
			}
		}
	}

	if rule == 0 {
		regex_string.push('$');
	}

	regex_string
}

#[derive(Clone, Debug)]
enum ProductionRule {
	Terminal(char),
	NonTerminal(Vec<Vec<i64>>),
}

fn parse(input: &str) -> (HashMap<i64, ProductionRule>, Vec<&str>) {
	let (rules_raw, messages_raw) = input.split_once("\n\n").unwrap();
	let messages: Vec<_> = messages_raw.lines().collect();
	let mut rules = HashMap::new();

	for rule_raw in rules_raw.lines() {
		let (num, rule) = rule_raw.split_once(": ").unwrap();
		let num: i64 = num.parse().unwrap();
		if rule == "\"a\"" || rule == "\"b\"" {
			rules.insert(num, ProductionRule::Terminal(rule.chars().nth(1).unwrap()));
			continue;
		}

		let mut r = Vec::new();
		for p in rule.split('|') {
			let mut m = Vec::new();
			for x in p.trim().split(' ') {
				m.push(x.parse::<i64>().unwrap());
			}
			r.push(m);
		}

		rules.insert(num, ProductionRule::NonTerminal(r));
	}

	(rules, messages)
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::lib::fetch_input;
	use test::Bencher;

	#[test]
	fn part_one_test_cases() {
		let input = include_str!("input.test1.txt");
		let solver = Day19Solver {};
		assert_eq!(solver.solve_part_one(input), 2);
	}

	#[test]
	fn part_two_test_cases() {
		let input = include_str!("input.test2.txt");
		let solver = Day19Solver {};
		assert_eq!(solver.solve_part_two(input), 12);
	}

	#[bench]
	fn bench_parse(bencher: &mut Bencher) {
		let input = fetch_input(19);
		bencher.iter(|| parse(&input));
	}

	#[bench]
	fn bench_part_one(bencher: &mut Bencher) {
		let input = fetch_input(19);
		let solver = Day19Solver {};
		bencher.iter(|| solver.solve_part_one(&input));
	}

	#[bench]
	fn bench_part_two(bencher: &mut Bencher) {
		let input = fetch_input(19);
		let solver = Day19Solver {};
		bencher.iter(|| solver.solve_part_two(&input));
	}
}
