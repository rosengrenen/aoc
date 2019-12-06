use crate::lib::Solver;
use std::cmp::Eq;
use std::cmp::Ordering;
use std::collections::HashMap;

pub struct Day3Solver;

impl Solver for Day3Solver {
  fn solve(&self, lines: Vec<String>, part_two: bool) -> String {
    let first_path: Vec<&str> = lines[0].split(',').collect();
    let second_path: Vec<&str> = lines[1].split(',').collect();

    calculate_distance(&first_path, &second_path, part_two).to_string()
  }
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
struct Pos {
  x: i32,
  y: i32,
}

impl Pos {
  fn up(&mut self) {
    self.y += 1;
  }

  fn right(&mut self) {
    self.x += 1;
  }

  fn down(&mut self) {
    self.y -= 1;
  }

  fn left(&mut self) {
    self.x -= 1;
  }
}

impl Default for Pos {
  fn default() -> Self {
    Pos { x: 0, y: 0 }
  }
}

fn manhattan_distance(pos1: &Pos, pos2: &Pos) -> i32 {
  (pos1.x - pos2.x).abs() + (pos1.y - pos2.y).abs()
}

fn calculate_distance(first_path: &Vec<&str>, second_path: &Vec<&str>, part_two: bool) -> i32 {
  let mut paths: HashMap<Pos, (i32, bool)> = HashMap::new();
  let mut current_pos = Pos { x: 0, y: 0 };
  let mut current_distance_travelled = 0;
  paths.insert(current_pos, (1, false));

  for &instruction in first_path.iter() {
    let (direction, distance) = instruction.split_at(1);
    let distance: i32 = distance.parse().unwrap();

    match direction {
      "U" => {
        for _ in 0..distance {
          current_pos.up();
          current_distance_travelled += 1;
          paths
            .entry(current_pos)
            .or_insert((current_distance_travelled, false));
        }
      }
      "R" => {
        for _ in 0..distance {
          current_pos.right();
          current_distance_travelled += 1;
          if !paths.contains_key(&current_pos) {
            paths.insert(current_pos, (current_distance_travelled, false));
          }
        }
      }
      "D" => {
        for _ in 0..distance {
          current_pos.down();
          current_distance_travelled += 1;
          if !paths.contains_key(&current_pos) {
            paths.insert(current_pos, (current_distance_travelled, false));
          }
        }
      }
      "L" => {
        for _ in 0..distance {
          current_pos.left();
          current_distance_travelled += 1;
          if !paths.contains_key(&current_pos) {
            paths.insert(current_pos, (current_distance_travelled, false));
          }
        }
      }
      _ => panic!("Invalid direction"),
    }
  }

  let mut intersections: HashMap<Pos, (i32, i32)> = HashMap::new();
  current_pos = Pos { x: 0, y: 0 };
  current_distance_travelled = 0;
  for &instruction in second_path.iter() {
    let (direction, distance) = instruction.split_at(1);
    let distance: i32 = distance.parse().unwrap();
    match direction {
      "U" => {
        for _ in 0..distance {
          current_pos.up();
          current_distance_travelled += 1;

          match paths.get(&current_pos) {
            Some(&(distance, _)) => match intersections.get_mut(&current_pos) {
              Some(_) => (),
              None => {
                intersections.insert(
                  current_pos,
                  (
                    distance + current_distance_travelled,
                    manhattan_distance(&Pos::default(), &current_pos),
                  ),
                );
              }
            },
            None => (),
          }
        }
      }
      "R" => {
        for _ in 0..distance {
          current_pos.right();
          current_distance_travelled += 1;

          match paths.get(&current_pos) {
            Some(&(distance, _)) => match intersections.get_mut(&current_pos) {
              Some(_) => (),
              None => {
                intersections.insert(
                  current_pos,
                  (
                    distance + current_distance_travelled,
                    manhattan_distance(&Pos::default(), &current_pos),
                  ),
                );
              }
            },
            None => (),
          }
        }
      }
      "D" => {
        for _ in 0..distance {
          current_pos.down();
          current_distance_travelled += 1;

          match paths.get(&current_pos) {
            Some(&(distance, _)) => match intersections.get_mut(&current_pos) {
              Some(_) => (),
              None => {
                intersections.insert(
                  current_pos,
                  (
                    distance + current_distance_travelled,
                    manhattan_distance(&Pos::default(), &current_pos),
                  ),
                );
              }
            },
            None => (),
          }
        }
      }
      "L" => {
        for _ in 0..distance {
          current_pos.left();
          current_distance_travelled += 1;

          match paths.get(&current_pos) {
            Some(&(distance, _)) => match intersections.get_mut(&current_pos) {
              Some(_) => (),
              None => {
                intersections.insert(
                  current_pos,
                  (
                    distance + current_distance_travelled,
                    manhattan_distance(&Pos::default(), &current_pos),
                  ),
                );
              }
            },
            None => (),
          }
        }
      }
      _ => panic!("Invalid direction"),
    }
  }

  let intersections: Vec<(Pos, i32, i32)> = intersections
    .iter()
    .map(|(pos, &(distance, mh_distance))| (pos.clone(), distance, mh_distance))
    .collect();

  if part_two {
    intersections
      .iter()
      .cloned()
      .fold(std::i32::MAX, |previous, (_, distance, _)| {
        match distance.cmp(&previous) {
          Ordering::Equal => previous,
          Ordering::Greater => previous,
          Ordering::Less => distance,
        }
      })
  } else {
    intersections
      .iter()
      .cloned()
      .fold(
        std::i32::MAX,
        |previous, (_, _, mh_distance)| match mh_distance.cmp(&previous) {
          Ordering::Equal => previous,
          Ordering::Greater => previous,
          Ordering::Less => mh_distance,
        },
      )
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn part_one_test_cases() {
    assert_eq!(
      calculate_distance(
        &vec!["R75", "D30", "R83", "U83", "L12", "D49", "R71", "U7", "L72"],
        &vec!["U62", "R66", "U55", "R34", "D71", "R55", "D58", "R83"],
        false
      ),
      159
    );
    assert_eq!(
      calculate_distance(
        &vec!["R98", "U47", "R26", "D63", "R33", "U87", "L62", "D20", "R33", "U53", "R51"],
        &vec!["U98", "R91", "D20", "R16", "D67", "R40", "U7", "R15", "U6", "R7"],
        false
      ),
      135
    );
  }

  #[test]
  fn part_two_test_cases() {
    assert_eq!(
      calculate_distance(
        &vec!["R75", "D30", "R83", "U83", "L12", "D49", "R71", "U7", "L72"],
        &vec!["U62", "R66", "U55", "R34", "D71", "R55", "D58", "R83"],
        true
      ),
      610
    );
    assert_eq!(
      calculate_distance(
        &vec!["R98", "U47", "R26", "D63", "R33", "U87", "L62", "D20", "R33", "U53", "R51"],
        &vec!["U98", "R91", "D20", "R16", "D67", "R40", "U7", "R15", "U6", "R7"],
        true
      ),
      410
    );
  }

  #[test]
  fn mh_distance_test() {
    assert_eq!(
      manhattan_distance(&Pos { x: 0, y: 0 }, &Pos { x: -7, y: 10 }),
      17
    );
    assert_eq!(
      manhattan_distance(&Pos { x: -75, y: 18 }, &Pos { x: -38, y: -17 }),
      72
    );
  }
}
