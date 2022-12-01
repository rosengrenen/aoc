use std::collections::{HashMap, HashSet};

use aoc_util::{Solver, SolverOutput};

#[derive(Default)]
pub struct Day7;

impl Solver for Day7 {
  fn part_one(&self, input: &str) -> SolverOutput {
    let child_to_parents_map = create_child_to_parents_map(input);
    let set = find_bags_that_contains_bag(&child_to_parents_map, "shiny gold");
    SolverOutput::Num(set.len() as i64)
  }

  fn part_two(&self, input: &str) -> SolverOutput {
    let parent_to_children_map = create_parent_to_children_map(input);
    SolverOutput::Num(count_bags_in_bag(&parent_to_children_map, "shiny gold"))
  }
}

fn create_child_to_parents_map(input: &str) -> HashMap<String, Vec<&str>> {
  let mut child_to_parents_map = HashMap::new();
  for line in input.lines() {
    let (parent_color, children_raw) = line.split_once(" bags contain ").unwrap();
    // No children
    if children_raw.as_bytes()[0] == b'n' {
      continue;
    }

    for child_raw in children_raw.split(',') {
      let child_parts_raw_iter = child_raw.trim().split_whitespace();
      let mut child_name_parts_iter = child_parts_raw_iter.skip(1).take(2);
      let child_color = child_name_parts_iter.next().unwrap().to_owned()
        + " "
        + child_name_parts_iter.next().unwrap();

      let entry = child_to_parents_map
        .entry(child_color)
        .or_insert_with(Vec::new);
      entry.push(parent_color);
    }
  }
  child_to_parents_map
}

fn create_parent_to_children_map(input: &str) -> HashMap<&str, Vec<(i64, String)>> {
  let mut parent_to_children_map = HashMap::new();
  for line in input.lines() {
    let (parent, children_raw) = line.split_once(" bags contain ").unwrap();
    // No children
    if children_raw.as_bytes()[0] == b'n' {
      continue;
    }

    let children: Vec<(i64, String)> = children_raw
      .split(',')
      .map(|child_raw| {
        let mut child_parts_raw_iter = child_raw.trim().split_whitespace();
        let child_count: i64 = child_parts_raw_iter.next().unwrap().parse().unwrap();
        let child_color = child_parts_raw_iter.next().unwrap().to_owned()
          + " "
          + child_parts_raw_iter.next().unwrap();

        (child_count, child_color)
      })
      .collect();
    parent_to_children_map.insert(parent, children);
  }
  parent_to_children_map
}

fn find_bags_that_contains_bag<'a>(
  child_to_parents_map: &HashMap<String, Vec<&'a str>>,
  bag: &str,
) -> HashSet<&'a str> {
  let mut result_acc = HashSet::new();
  find_bags_that_contains_bag_recursive(child_to_parents_map, bag, &mut result_acc);
  result_acc
}

fn find_bags_that_contains_bag_recursive<'a>(
  child_to_parents_map: &HashMap<String, Vec<&'a str>>,
  bag: &str,
  result_acc: &mut HashSet<&'a str>,
) {
  if let Some(bags_that_contain_bag) = child_to_parents_map.get(bag) {
    for bag_that_contains_bag in bags_that_contain_bag.iter() {
      result_acc.insert(bag_that_contains_bag);
      find_bags_that_contains_bag_recursive(child_to_parents_map, bag_that_contains_bag, result_acc)
    }
  }
}

fn count_bags_in_bag(parent_to_children_map: &HashMap<&str, Vec<(i64, String)>>, bag: &str) -> i64 {
  if let Some(bags_in_bag) = parent_to_children_map.get(bag) {
    bags_in_bag
      .iter()
      .map(|(child_count, child_color)| {
        child_count * (1 + count_bags_in_bag(parent_to_children_map, child_color))
      })
      .sum()
  } else {
    0
  }
}
