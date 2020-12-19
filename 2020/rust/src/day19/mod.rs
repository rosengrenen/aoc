use hashbrown::{HashMap, HashSet};

use crate::lib::Solver;

pub struct Day19Solver;

impl Solver for Day19Solver {
	fn solve_part_one(&self, input: &str) -> i64 {
		let (rules, messages) = parse(input);
		let mut s = 0;
		let valid_messages = all_valid(&rules);
		let valid_messages = valid_messages.get(&0).unwrap();
		// println!("{:?}", valid_messages);
		for &m in messages.iter() {
			// if m is valid add one to s
			if valid_messages.contains(&m.to_owned()) {
				s += 1;
			}
		}
		s
	}

	fn solve_part_two(&self, input: &str) -> i64 {
		let (mut rules, messages) = parse(input);
		//8: 42 | 42 8
		rules
			.entry(8)
			.and_modify(|entry| *entry = K::NonTerminal(vec![vec![42], vec![42, 8]]));
		//11: 42 31 | 42 11 31
		rules
			.entry(11)
			.and_modify(|entry| *entry = K::NonTerminal(vec![vec![42, 31], vec![42, 11, 31]]));

		let mut s = 0;
		let valid_messages = all_valid_loop(&rules);
		// println!("{:#?}", valid_messages.get(&8).unwrap());
		// let valid_messages = valid_messages.get(&0).unwrap();
		// println!("{:?}", valid_messages.len());
		// println!(
		// 	"FINALLY {}",
		// 	is_valid(
		// 		&valid_messages,
		// 		&"aabbbbbaabbbaaaaaabbbbbababaaaaabbaaabba",
		// 		0
		// 	)
		// );
		for &m in messages.iter() {
			// if m is valid add one to s
			if is_valid2(&valid_messages, m, 0) {
				s += 1;
			} else {
				println!("{} is invalid but should be valid", m);
			}
		}
		s
	}
}

#[derive(Clone, Debug)]
enum K {
	Terminal(char),
	NonTerminal(Vec<Vec<i64>>),
}

fn parse(input: &str) -> (HashMap<i64, K>, Vec<&str>) {
	println!("{:?}", input.split("\n\n").collect::<Vec<_>>());
	let (rules_raw, messages_raw) = input.split_once("\n\n").unwrap();
	let messages: Vec<_> = messages_raw.lines().collect();
	let mut rules = HashMap::new();

	for rule_raw in rules_raw.lines() {
		let (num, rule) = rule_raw.split_once(": ").unwrap();
		let num: i64 = num.parse().unwrap();
		if rule == "\"a\"" || rule == "\"b\"" {
			rules.insert(num, K::Terminal(rule.chars().nth(1).unwrap()));
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

		rules.insert(num, K::NonTerminal(r));
	}

	(rules, messages)

	// unreachable!();
	// let mut productions = HashMap::new();
	// for rule in rules.iter() {
	// 	match *rule.1 {
	// 		K::Terminal(t) => productions.insert(rule.0, t),
	// 		K::NonTerminal(b) => {
	// 			//
	// 		}
	// 	};
	// }

	// (productions, messages)
}

fn all_valid(rules: &HashMap<i64, K>) -> HashMap<i64, HashSet<String>> {
	let mut result = HashMap::new();
	all_valid_internal(rules, 0, &mut result);
	result
}

fn all_valid_internal(rules: &HashMap<i64, K>, i: i64, result: &mut HashMap<i64, HashSet<String>>) {
	if result.contains_key(&i) {
		return;
	}
	// println!("{}", i);
	let a = rules.get(&i).unwrap();
	if let K::Terminal(t) = a {
		let mut res: HashSet<String> = HashSet::new();
		res.insert(t.to_string());
		result.insert(i, res);
	} else if let K::NonTerminal(p) = a {
		let mut res: HashSet<String> = HashSet::new();
		for b in p.iter() {
			let mut inner_res = HashSet::new();
			for c in b.iter() {
				all_valid_internal(rules, *c, result);
				if inner_res.is_empty() {
					for d in result.get(c).unwrap() {
						inner_res.insert(d.clone());
					}
				} else {
					let mut new_res: HashSet<String> = HashSet::new();
					for e in inner_res.iter() {
						for d in result.get(c).unwrap() {
							new_res.insert(e.clone() + d);
						}
						new_res.remove(e);
					}
					inner_res = new_res;
				}
			}
			for s in inner_res {
				res.insert(s);
			}
		}
		result.insert(i, res.clone());
	}
}

fn all_valid_loop(rules: &HashMap<i64, K>) -> HashMap<i64, HashSet<String>> {
	let mut result = HashMap::new();
	all_valid_internal_loop(rules, 0, &mut result);
	result
}

fn all_valid_internal_loop(
	rules: &HashMap<i64, K>,
	i: i64,
	result: &mut HashMap<i64, HashSet<String>>,
) {
	println!("getting {}", i);
	if result.contains_key(&i) {
		return;
	}
	let a = rules.get(&i).unwrap();
	if let K::Terminal(t) = a {
		let mut res: HashSet<String> = HashSet::new();
		res.insert(t.to_string());
		result.insert(i, res);
	} else if let K::NonTerminal(p) = a {
		let mut res: HashSet<String> = HashSet::new();
		for b in p.iter() {
			let mut inner_res: HashSet<String> = HashSet::new();
			for c in b.iter() {
				if *c == i {
					let mut new_res = HashSet::new();
					for inr in inner_res.iter() {
						new_res.insert(inr.clone() + &i.to_string());
					}
					inner_res = new_res;
					continue;
				}
				all_valid_internal_loop(rules, *c, result);
				if inner_res.is_empty() {
					for d in result.get(c).unwrap() {
						inner_res.insert(d.clone());
					}
				} else {
					let mut new_res: HashSet<String> = HashSet::new();
					for e in inner_res.iter() {
						for d in result.get(c).unwrap() {
							new_res.insert(e.clone() + d);
						}
					}
					inner_res = new_res;
				}
			}
			for s in inner_res {
				res.insert(s);
			}
		}
		result.insert(i, res.clone());
	}
}

fn is_valid2(rules: &HashMap<i64, HashSet<String>>, message: &str, rule: i64) -> bool {
	for valid_message in rules.get(&rule).unwrap() {
		if valid_message.len() > message.len() {
			continue;
		}

		if valid_message == message {
			// println!("matched {} to rule {}", message, rule);
			return true;
		}

		let mut parts = Vec::new();
		let mut rule_nums: Vec<i64> = Vec::new();

		let mut m = Some(&valid_message[..]);
		while let Some(loc_m) = m {
			let mut index = None;
			if let Some(i) = loc_m.find("11") {
				index = Some((i, String::from("11")));
			}
			if let Some(i) = loc_m.find('8') {
				if let Some((prev_index, _)) = index {
					if i < prev_index {
						index = Some((i, String::from("8")));
					}
				} else {
					index = Some((i, String::from("8")));
				}
			}

			if let Some((i, s)) = index {
				parts.push(loc_m[..i].to_owned());
				rule_nums.push(s.parse().unwrap());
				m = Some(&loc_m[i + s.len()..]);
			} else {
				m = None;
				parts.push(loc_m.to_owned());
			}
		}

		// There was no recursive number in the valid_message
		if rule_nums.is_empty() {
			continue;
		}

		if let Some(message) = message.strip_prefix(parts.first().unwrap()) {
			if let Some(message) = message.strip_suffix(parts.last().unwrap()) {
				// If rule nums is 1, just check the only recursive part
				if rule_nums.len() == 1 {
					if is_valid2(rules, message, rule_nums[0]) {
						return true;
					}
				} else if rule_nums.len() == 2 {
					let middle_part = &parts[1];
					for middle_part_index in message.match_indices(middle_part).map(|(i, _)| i) {
						if is_valid2(rules, &message[..middle_part_index], rule_nums[0])
							&& is_valid2(
								rules,
								&message[middle_part_index + middle_part.len()..],
								rule_nums[1],
							) {
							return true;
						}
					}
				} else {
					panic!("According to the puzzle input there should never be 3 parts");
				}
			}
		}
	}

	false
}

fn is_valid(rules: &HashMap<i64, HashSet<String>>, message: &str, rule: i64) -> bool {
	println!("matching {} to rule {}", message, rule);
	'root: for valid_message in rules.get(&rule).unwrap() {
		if valid_message.len() > message.len() {
			continue;
		}

		if valid_message == message {
			println!("matched {} to rule {}", message, rule);
			return true;
		}

		let mut parts = Vec::new();
		let mut rule_nums: Vec<i64> = Vec::new();

		let mut m = Some(&valid_message[..]);
		while let Some(loc_m) = m {
			let mut index = None;
			if let Some(i) = loc_m.find("11") {
				index = Some((i, String::from("11")));
			}
			if let Some(i) = loc_m.find('8') {
				if let Some((prev_index, _)) = index {
					if i < prev_index {
						index = Some((i, String::from("8")));
					}
				} else {
					index = Some((i, String::from("8")));
				}
			}

			if let Some((i, s)) = index {
				parts.push(loc_m[..i].to_owned());
				rule_nums.push(s.parse().unwrap());
				m = Some(&loc_m[i + s.len()..]);
			} else {
				m = None;
				parts.push(loc_m.to_owned());
			}
		}

		if !rule_nums.is_empty() {
			// println!("{:?} {:?}", parts, rule_nums);
			let orig_message = message;
			if let Some(message) = message.strip_prefix(parts.first().unwrap()) {
				if let Some(message) = message.strip_suffix(parts.last().unwrap()) {
					// println!(
					// 	"{} {:?} {:?} {} {}",
					// 	rule, parts, rule_nums, orig_message, message
					// );
					if rule_nums.len() == 1 {
						if is_valid(rules, message, rule_nums[0]) {
							// println!("matched {} to rule {}", message, rule);
							return true;
						}
					} else {
						let mut indices = HashSet::new();
						indices.insert(0);
						for (i, rule_num) in rule_nums.iter().enumerate() {
							let mut new_indices = HashSet::new();
							println!("{}+ ---------------------", rule_num);
							for &index in indices.iter() {
								let message = &message[index..];
								let next_part_indices = message
									.match_indices(&parts[i + 1])
									.map(|(i, _)| i)
									.collect::<Vec<_>>();
								println!("{}+ next p i {:?}", rule_num, next_part_indices);
								for &next_part_index in next_part_indices.iter() {
									if is_valid(rules, &message[..next_part_index], *rule_num) {
										new_indices.insert(index + next_part_index);
									}
								}
								println!("{}+ valid p i {:?}", rule_num, new_indices);
							}
							// println!("valid p i 2 {} {} {:?}", i, rule_num, new_indices);
							println!("!---------------------");
							if new_indices.is_empty() {
								break 'root;
							}
							indices = new_indices;
							// let next_part_indices = message
							// 	.match_indices(&parts[i + 1])
							// 	.map(|(i, _)| i)
							// 	.collect::<Vec<_>>();
							// if next_part_indices.is_empty() {
							// 	return false;
							// }

							// for &next_part_index in next_part_indices.iter() {
							// 	if is_valid(rules, &message[..next_part_index], *rule_num) {
							// 		indices.push(next_part_index);
							// 	}
							// }

							// if indices.is_empty() {
							// 	return false;
							// }
							// println!("!!! {:?} {:?}", next_part_indices, indices);
						}
						println!("matched 3 {} to rule {}", message, rule);
						return true;
					}
				}
			}
		// return false;
		} else {
			// println!("{} empty", rule);
			// let mut indices = Vec::new();
			// for i in 0..rule_nums.len() {
			// 	break;
			// }
		}

		// if let Some(index) = valid_message.find("11") {
		// 	let before = &valid_message[0..index];
		// 	let after = &valid_message[index + 2..];
		// 	if message.starts_with(before) && valid_message.ends_with(after) {
		// 		println!(
		// 			"this could match {} {} {} {} {}",
		// 			message,
		// 			before,
		// 			&message[before.len()..message.len() - after.len()],
		// 			after,
		// 			valid_message
		// 		);
		// 		// aaabbbbbbaaaabaababaabababbabaaabbababababaaa aaaba8baaaa aabaababaabababbabaaabbababab baaab aaaba8baaaa11baaab
		// 		// if is_valid(rules, message, 11);
		// 	}
		// };

		// if let Some(index) = valid_message.find('8') {
		// 	println!("found 8 at {}", index);
		// }
	}

	println!("SADFACE");
	false
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

	// #[bench]
	// fn bench_part_one(bencher: &mut Bencher) {
	// 	let input = fetch_input(19);
	// 	let solver = Day19Solver {};
	// 	bencher.iter(|| solver.solve_part_one(&input));
	// }

	// #[bench]
	// fn bench_part_two(bencher: &mut Bencher) {
	// 	let input = fetch_input(19);
	// 	let solver = Day19Solver {};
	// 	bencher.iter(|| solver.solve_part_two(&input));
	// }
}
