use std::{cmp::Ordering, collections::HashMap};

use aoc_util::{Solver, SolverOutput};

#[derive(Default)]
pub struct Day6;

impl Solver for Day6 {
  fn part_one(&self, input: &str) -> SolverOutput {
    let tree = build_orbit_tree(input);
    let orbit_counts = calculate_orbit_counts(&tree);
    SolverOutput::Num(orbit_counts)
  }

  fn part_two(&self, input: &str) -> SolverOutput {
    let tree = build_orbit_tree(input);
    let orbital_transfers =
      calculate_orbital_transfers(&tree, String::from("YOU"), String::from("SAN"));
    SolverOutput::Num(orbital_transfers)
  }
}

#[derive(Debug)]
struct OrbitNode {
  children: Vec<OrbitNode>,
  depth: i64,
  name: String,
}

fn build_orbit_tree(input: &str) -> OrbitNode {
  let mut parent_to_children: HashMap<&str, Vec<&str>> = HashMap::new();
  for s in input.lines() {
    let planets: Vec<&str> = s.split(')').collect();
    match parent_to_children.get_mut(planets[0]) {
      Some(n) => n.push(planets[1]),
      None => {
        parent_to_children.insert(planets[0], vec![planets[1]]);
      }
    }
  }
  let mut node = OrbitNode {
    children: Vec::new(),
    depth: 1,
    name: String::from("COM"),
  };
  build_orbit_subtree(&parent_to_children, &mut node);
  node
}

fn build_orbit_subtree(parent_to_children: &HashMap<&str, Vec<&str>>, node: &mut OrbitNode) {
  match parent_to_children.get(&node.name[..]) {
    Some(children) => {
      for child in children.iter() {
        node.children.push(OrbitNode {
          children: Vec::new(),
          depth: node.depth + 1,
          name: child.to_string(),
        })
      }
      for child in node.children.iter_mut() {
        build_orbit_subtree(parent_to_children, child);
      }
    }
    None => (),
  };
}

fn calculate_orbit_counts(node: &OrbitNode) -> i64 {
  let direct_orbits = std::cmp::max(std::cmp::min(node.depth - 1, 1), 0);
  let indirect_orbits = std::cmp::max(node.depth - 2, 0);
  if node.children.is_empty() {
    return direct_orbits + indirect_orbits;
  }

  let mut orbits = 0;
  for planet in node.children.iter() {
    orbits += calculate_orbit_counts(planet);
  }

  orbits + indirect_orbits + direct_orbits
}

fn find_path(node: &OrbitNode, target: String) -> Vec<(String, i64)> {
  let mut path: Vec<(String, i64)> = Vec::new();
  find_path_builder(node, target, &mut path);
  path
}

fn find_path_builder(node: &OrbitNode, target: String, acc: &mut Vec<(String, i64)>) -> bool {
  if node.name == target {
    return true;
  }

  if node.children.is_empty() {
    return false;
  }

  for n in node.children.iter() {
    if find_path_builder(n, target.clone(), acc) {
      acc.push((node.name.clone(), node.depth));
      return true;
    }
  }

  false
}

fn calculate_orbital_transfers(node: &OrbitNode, from: String, to: String) -> i64 {
  let from_path = find_path(node, from);
  let to_path = find_path(node, to);

  let mut from_index = 0;
  let mut to_index = 0;

  loop {
    let (from_name, from_depth) = &from_path[from_index];
    let (to_name, to_depth) = &to_path[to_index];
    match from_depth.cmp(&to_depth) {
      Ordering::Equal => {
        if from_name == to_name {
          break;
        }

        from_index += 1;
        to_index += 1;
      }
      Ordering::Greater => {
        from_index += (from_depth - to_depth) as usize;
      }
      Ordering::Less => {
        to_index += (to_depth - from_depth) as usize;
      }
    }
  }

  (from_index + to_index) as i64
}

// #[cfg(test)]
// mod tests {
// 	use super::*;

// 	#[test]
// 	fn part_one_test_cases() {
// 		let orbits = calculate_orbit_counts(&build_orbit_tree(&vec![
// 			String::from("COM)B"),
// 			String::from("B)C"),
// 			String::from("C)D"),
// 			String::from("D)E"),
// 			String::from("E)F"),
// 			String::from("B)G"),
// 			String::from("G)H"),
// 			String::from("D)I"),
// 			String::from("E)J"),
// 			String::from("J)K"),
// 			String::from("K)L"),
// 		]));
// 		assert_eq!(orbits, 42);
// 	}

// 	#[test]
// 	fn part_two_test_cases() {
// 		let orbital_transfers = calculate_orbital_transfers(
// 			&build_orbit_tree(&vec![
// 				String::from("COM)B"),
// 				String::from("B)C"),
// 				String::from("C)D"),
// 				String::from("D)E"),
// 				String::from("E)F"),
// 				String::from("B)G"),
// 				String::from("G)H"),
// 				String::from("D)I"),
// 				String::from("E)J"),
// 				String::from("J)K"),
// 				String::from("K)L"),
// 				String::from("K)YOU"),
// 				String::from("I)SAN"),
// 			]),
// 			String::from("YOU"),
// 			String::from("SAN"),
// 		);
// 		assert_eq!(orbital_transfers, 4);
// 	}
// }
