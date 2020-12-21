use hashbrown::{HashMap, HashSet};

use crate::lib::Solver;

pub struct Day21Solver;

impl Solver for Day21Solver {
	fn solve_part_one(&self, input: &str) -> i64 {
		let input = parse(input);
		let mut allergents_set = HashSet::new();
		for (_, allergents) in input.iter() {
			for allergent in allergents.iter() {
				allergents_set.insert(allergent);
			}
		}

		println!("{:?}", allergents_set);
		let mut ingredient_to_allergent = HashMap::new();

		while !allergents_set.is_empty() {
			let mut found = None;
			for allergent in allergents_set.iter() {
				let mut s = HashSet::new();
				let mut first = true;
				for (ingredients, allergents) in input.iter() {
					if allergents.contains(allergent) {
						if first {
							first = false;
							for ingredient in ingredients.iter() {
								if !ingredient_to_allergent.contains_key(ingredient) {
									s.insert(ingredient);
								}
							}
						} else {
							s.retain(|ing| ingredients.contains(ing));
						}
					}
				}

				println!("Remaining to deduce {}: {:?}", allergent, s);

				if s.len() == 1 {
					found = Some((allergent.to_owned(), s.iter().next().unwrap().to_owned()));
					break;
				}
			}

			if let Some((allergent, ingredient)) = found {
				ingredient_to_allergent.insert(ingredient, allergent);
				allergents_set.remove(allergent);
			} else {
				panic!(format!("{}", allergents_set.len()))
			}
		}

		println!("{:?}", allergents_set);
		println!("{:?}", ingredient_to_allergent);

		let mut count = 0;

		for (ingredients, _) in input.iter() {
			for ingredient in ingredients.iter() {
				if !ingredient_to_allergent.contains_key(ingredient) {
					count += 1;
				}
			}
		}

		count
	}

	fn solve_part_two(&self, input: &str) -> i64 {
		let input = parse(input);
		let mut allergents_set = HashSet::new();
		for (_, allergents) in input.iter() {
			for allergent in allergents.iter() {
				allergents_set.insert(allergent);
			}
		}

		println!("{:?}", allergents_set);
		let mut ingredient_to_allergent = HashMap::new();

		while !allergents_set.is_empty() {
			let mut found = None;
			for allergent in allergents_set.iter() {
				let mut s = HashSet::new();
				let mut first = true;
				for (ingredients, allergents) in input.iter() {
					if allergents.contains(allergent) {
						if first {
							first = false;
							for ingredient in ingredients.iter() {
								if !ingredient_to_allergent.contains_key(ingredient) {
									s.insert(ingredient);
								}
							}
						} else {
							s.retain(|ing| ingredients.contains(ing));
						}
					}
				}

				println!("Remaining to deduce {}: {:?}", allergent, s);

				if s.len() == 1 {
					found = Some((allergent.to_owned(), s.iter().next().unwrap().to_owned()));
					break;
				}
			}

			if let Some((allergent, ingredient)) = found {
				ingredient_to_allergent.insert(ingredient, allergent);
				allergents_set.remove(allergent);
			} else {
				panic!(format!("{}", allergents_set.len()))
			}
		}

		println!("{:?}", allergents_set);
		println!("{:?}", ingredient_to_allergent);

		let mut dangerous_ingredients: Vec<_> = ingredient_to_allergent.iter().collect();
		dangerous_ingredients.sort_unstable_by(|left, right| left.1.cmp(&right.1));
		println!(
			"{:?}",
			dangerous_ingredients
				.iter()
				.map(|(i, _)| ***i)
				.collect::<Vec<_>>()
				.join(",")
		);

		1
	}
}

fn parse(input: &str) -> Vec<(Vec<&str>, Vec<&str>)> {
	input
		.lines()
		.map(|line| {
			let (ingredients_raw, allergents_raw) = line.split_once(" (contains ").unwrap();
			let ingredients = ingredients_raw.trim().split_whitespace().collect();
			let allergents = allergents_raw
				.strip_suffix(')')
				.unwrap()
				.trim()
				.split(", ")
				.collect();
			(ingredients, allergents)
		})
		.collect()
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::lib::fetch_input;
	use test::Bencher;

	#[test]
	fn part_one_test_cases() {
		let input = include_str!("input.test1.txt");
		let solver = Day21Solver {};
		assert_eq!(solver.solve_part_one(input), 5);
	}

	#[test]
	fn part_two_test_cases() {
		let input = include_str!("input.test1.txt");
		let solver = Day21Solver {};
		assert_eq!(solver.solve_part_two(input), 0);
	}

	// #[bench]
	// fn bench_parse(bencher: &mut Bencher) {
	// 	let input = fetch_input(21);
	// 	bencher.iter(|| parse(&input));
	// }

	// #[bench]
	// fn bench_part_one(bencher: &mut Bencher) {
	// 	let input = fetch_input(21);
	// 	let solver = Day21Solver {};
	// 	bencher.iter(|| solver.solve_part_one(&input));
	// }

	// #[bench]
	// fn bench_part_two(bencher: &mut Bencher) {
	// 	let input = fetch_input(21);
	// 	let solver = Day21Solver {};
	// 	bencher.iter(|| solver.solve_part_two(&input));
	// }
}
