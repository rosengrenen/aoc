use aoc_util::{Solver, SolverOutput};
use hashbrown::{HashMap, HashSet};

#[derive(Default)]
pub struct Day21;

impl Solver for Day21 {
  fn part_one(&self, input: &str) -> SolverOutput {
    let foods = parse_foods(input);
    let ingredient_allergent_pairs = find_ingredient_allergent_pairs(&foods);

    let mut count = 0;

    for (ingredients, _) in foods.into_iter() {
      for ingredient in ingredients.into_iter() {
        if !ingredient_allergent_pairs.contains_key(ingredient) {
          count += 1;
        }
      }
    }

    SolverOutput::Num(count)
  }

  fn part_two(&self, input: &str) -> SolverOutput {
    let foods = parse_foods(input);
    let ingredient_to_allergent = find_ingredient_allergent_pairs(&foods);

    let mut dangerous_ingredients: Vec<_> = ingredient_to_allergent.into_iter().collect();
    dangerous_ingredients.sort_unstable_by(|left, right| left.1.cmp(&right.1));

    SolverOutput::String(
      dangerous_ingredients
        .iter()
        .map(|&(i, _)| i)
        .collect::<Vec<_>>()
        .join(","),
    )
  }
}

fn parse_foods(input: &str) -> Vec<(HashSet<&str>, HashSet<&str>)> {
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

fn find_ingredient_allergent_pairs<'a>(
  foods: &[(HashSet<&'a str>, HashSet<&'a str>)],
) -> HashMap<&'a str, &'a str> {
  let mut all_allergents: HashSet<&str> = HashSet::new();
  for (_, allergents) in foods.iter() {
    for &allergent in allergents.iter() {
      all_allergents.insert(allergent);
    }
  }

  let mut ingredient_to_allergent: HashMap<&str, &str> = HashMap::new();
  while !all_allergents.is_empty() {
    let mut found = None;
    for allergent in all_allergents.iter() {
      // Ingredients that are all foods that contain the allergent, after checking
      // all foods this hopefully leaves only one ingredient
      let mut matching_ingredients: HashSet<&str> = HashSet::new();
      // Since the matching_ingredients set is empty from the beginning, we need
      // to fill it up with the first matching food's ingredients, and then reduce
      let mut first_matching_food = true;
      for (ingredients, allergents) in foods.iter() {
        if allergents.contains(allergent) {
          if first_matching_food {
            first_matching_food = false;
            for ingredient in ingredients.iter() {
              if !ingredient_to_allergent.contains_key(ingredient) {
                matching_ingredients.insert(ingredient);
              }
            }
          } else {
            matching_ingredients.retain(|ingredient| ingredients.contains(ingredient));
          }
        }
      }

      // If there is only one we have found a ingredient -> allergent pair
      if matching_ingredients.len() == 1 {
        found = Some((
          allergent.to_owned(),
          matching_ingredients.iter().next().unwrap().to_owned(),
        ));
        break;
      }
    }

    if let Some((allergent, ingredient)) = found {
      ingredient_to_allergent.insert(ingredient, allergent);
      all_allergents.remove(allergent);
    } else {
      panic!("No allergent could be matched to one specific ingredient");
    }
  }

  ingredient_to_allergent
}
