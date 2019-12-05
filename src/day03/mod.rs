use crate::lib::Solver;
use std::cmp::Eq;
use std::cmp::Ordering;
use std::collections::HashMap;

pub struct Day3Solver;

impl Solver for Day3Solver {
  fn solve(&self, lines: Vec<String>, part_two: bool) -> String {
    let first_path: Vec<&str> = lines[0].split(',').collect();
    let second_path: Vec<&str> = lines[1].split(',').collect();

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
        .fold(
          std::i32::MAX,
          |previous, (_, _, mh_distance)| match mh_distance.cmp(&previous) {
            Ordering::Equal => previous,
            Ordering::Greater => previous,
            Ordering::Less => mh_distance,
          },
        )
        .to_string()
    } else {
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
        .to_string()
    }
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
