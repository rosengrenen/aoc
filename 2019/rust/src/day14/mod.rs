use std::collections::HashMap;

use aoc_util::{Solver, SolverOutput};

#[derive(Default)]
pub struct Day14;

impl Solver for Day14 {
  fn part_one(&self, input: &str) -> SolverOutput {
    let recipes = parse_recipes(input);
    let ore_cost = calculate_ore_cost(&recipes, "FUEL", 1);
    SolverOutput::Num(ore_cost)
  }

  fn part_two(&self, input: &str) -> SolverOutput {
    let recipes = parse_recipes(input);
    let mut max_fuel = 1;
    let max_fuel = loop {
      let ore_cost = calculate_ore_cost(&recipes, "FUEL", max_fuel as i64) as i128;
      let new_max_fuel = max_fuel * 1_000_000_000_000 / ore_cost;
      if new_max_fuel == max_fuel {
        break max_fuel;
      }

      max_fuel = new_max_fuel;
    };

    SolverOutput::Num(max_fuel as i64)
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

fn calculate_ore_cost<'a>(recipes: &Recipes<'a>, name: &'a str, amount: i64) -> i64 {
  let (_, fuel_inputs) = recipes.get(name).unwrap();
  let mut ingredient_amounts: Vec<_> = fuel_inputs
    .iter()
    .map(|ingredient| (ingredient.amount * amount, ingredient.name))
    .collect();

  loop {
    let (current_amount, current_name) = ingredient_amounts.remove(0);
    if current_name == "ORE" {
      if ingredient_amounts.is_empty() {
        return current_amount;
      }

      ingredient_amounts.push((current_amount, current_name));
      continue;
    }

    let only_input = ingredient_amounts
      .iter()
      .all(|(_, n)| !in_input(recipes, n, current_name));
    if only_input {
      let (input_amount, ingredients) = recipes.get(current_name).unwrap();
      let recipe_amount = if current_amount % input_amount != 0 {
        current_amount / input_amount + 1
      } else {
        current_amount / input_amount
      };

      for ingredient in ingredients {
        if let Some(index) = ingredient_amounts
          .iter()
          .position(|(_, name)| *name == ingredient.name)
        {
          ingredient_amounts[index].0 += ingredient.amount * recipe_amount;
        } else {
          ingredient_amounts.push((ingredient.amount * recipe_amount, ingredient.name));
        }
      }
    } else {
      ingredient_amounts.push((current_amount, current_name));
    }
  }
}

fn in_input(recipes: &Recipes, name: &str, search_name: &str) -> bool {
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
