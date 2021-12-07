use std::collections::HashMap;

use aoc_util::{Solver, SolverOutput};

#[derive(Default)]
pub struct Day14;

impl Solver for Day14 {
	fn part_one(&self, input: &str) -> SolverOutput {
		let recipes = parse_recipes(input);
		let ore_cost = calculate_ore_cost(&recipes);
		SolverOutput::Num(ore_cost)
	}

	fn part_two(&self, input: &str) -> SolverOutput {
		SolverOutput::Num(0)
	}
}

type Recipes<'a> = HashMap<&'a str, (i64, Vec<Ingredient<'a>>)>;

#[derive(Debug)]
struct Ingredient<'a> {
	name: &'a str,
	amount: i64,
}

fn parse_recipes<'a>(input: &'a str) -> Recipes {
	let parse_ingredient = |input: &'a str| -> Ingredient<'a> {
		let (amount, name) = input.trim().split_once(' ').unwrap();
		Ingredient {
			name,
			amount: amount.parse().unwrap(),
		}
	};

	input
		.lines()
		.map(|line| {
			let (inputs, output) = line.split_once("=>").unwrap();
			let inputs = inputs.split(',').map(parse_ingredient).collect();
			let output = parse_ingredient(output);
			(output.name, (output.amount, inputs))
		})
		.collect()
}

fn calculate_ore_cost(recipes: &Recipes) -> i64 {
	let (_, fuel_inputs) = recipes.get("FUEL").unwrap();
	let mut amounts: Vec<_> = fuel_inputs
		.iter()
		.map(|ingredient| (ingredient.amount, ingredient.name))
		.collect();

	println!("pre:");
	println!("check = {:?}", amounts);
	println!("");

	let mut i = 0;
	loop {
		if i > 100 {
			break 0;
		}
		println!("iter {}:", i);

		let (amount, name) = amounts.remove(0);
		if name == "ORE" {
			if amounts.is_empty() {
				return amount;
			}

			amounts.push((amount, name));
			continue;
		}

		println!("cur = ({}, {})", amount, name);

		let only_input = amounts.iter().all(|(_, n)| !in_input(recipes, n, name));
		println!("{} in *? {}", name, !only_input);
		if only_input {
			println!("GOOD add to outputs");

			let (a, is) = recipes.get(name).unwrap();
			let k = if amount % a != 0 {
				amount / a + 1
			} else {
				amount / a
			};

			println!("{} {} {}", amount, a, k);
			for i in is {
				if let Some(index) = amounts.iter().position(|(_, name)| *name == i.name) {
					amounts[index].0 += i.amount * k;
				} else {
					amounts.push((i.amount * k, i.name));
				}
			}
		} else {
			println!("BAD!!!");
			amounts.push((amount, name));
		}

		println!("check = {:?}", amounts);
		println!("");

		i += 1;
	}
}

fn in_input(recipes: &Recipes, name: &str, search_name: &str) -> bool {
	// println!("{} in {}?", search_name, name);
	if name == search_name {
		return true;
	}

	let inputs = match recipes.get(name) {
		Some((_, inputs)) => inputs,
		None => return false,
	};
	inputs
		.iter()
		.any(|input| in_input(recipes, input.name, search_name))
}
