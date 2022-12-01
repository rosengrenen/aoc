use std::collections::HashMap;

use aoc_util::{Solver, SolverOutput};
use num;

#[derive(Default)]
pub struct Day10;

impl Solver for Day10 {
  fn part_one(&self, input: &str) -> SolverOutput {
    let asteroid_map = create_asteroid_map(&input.lines().map(|line| line.to_owned()).collect());
    let (_, asteroids_in_sight) = find_best_asteroid(&asteroid_map);
    SolverOutput::Num(asteroids_in_sight)
  }

  fn part_two(&self, input: &str) -> SolverOutput {
    let asteroid_map = create_asteroid_map(&input.lines().map(|line| line.to_owned()).collect());
    let (best_asteroid_position, _) = find_best_asteroid(&asteroid_map);
    let vaporized_asteroid_200 =
      vaporize_asteroids(asteroid_map.clone(), &best_asteroid_position)[199];
    SolverOutput::Num(vaporized_asteroid_200.x * 100 + vaporized_asteroid_200.y)
  }
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
struct Pos {
  x: i64,
  y: i64,
}

fn create_asteroid_map(lines: &Vec<String>) -> Vec<Vec<bool>> {
  let mut asteroid_map: Vec<Vec<bool>> = Vec::new();

  for line in lines.iter() {
    let mut row: Vec<bool> = Vec::new();
    for c in line.chars() {
      row.push(c == '#');
    }
    asteroid_map.push(row);
  }

  asteroid_map
}

fn find_best_asteroid(asteroid_map: &Vec<Vec<bool>>) -> (Pos, i64) {
  let mut asteroid_map_sight_count: HashMap<Pos, i64> = HashMap::new();
  for y in 0..asteroid_map.len() {
    for x in 0..asteroid_map[0].len() {
      if !asteroid_map[y][x] {
        continue;
      }
      for other_y in 0..asteroid_map.len() {
        for other_x in 0..asteroid_map[0].len() {
          if !asteroid_map[other_y][other_x] {
            continue;
          }
          let pos = Pos {
            x: x as i64,
            y: y as i64,
          };
          let other_pos = Pos {
            x: other_x as i64,
            y: other_y as i64,
          };
          if pos == other_pos {
            continue;
          }
          if is_in_line_of_sight(asteroid_map, pos, other_pos) {
            *asteroid_map_sight_count.entry(pos).or_insert(0) += 1;
          }
        }
      }
    }
  }
  asteroid_map_sight_count.iter().fold(
    (Pos { x: 0, y: 0 }, 0),
    |(prev_pos, prev_asteroids_in_sight), (pos, asteroids_in_sight)| {
      if *asteroids_in_sight > prev_asteroids_in_sight {
        (*pos, *asteroids_in_sight)
      } else {
        (prev_pos, prev_asteroids_in_sight)
      }
    },
  )
}

fn is_in_line_of_sight(asteroid_map: &Vec<Vec<bool>>, start_pos: Pos, target_pos: Pos) -> bool {
  let mut x_diff = target_pos.x - start_pos.x;
  let mut y_diff = target_pos.y - start_pos.y;
  let gcd = num::integer::gcd(x_diff.abs(), y_diff.abs());
  x_diff /= gcd;
  y_diff /= gcd;
  let mut current_pos = start_pos;
  loop {
    current_pos.x += x_diff;
    current_pos.y += y_diff;
    if current_pos == target_pos {
      return true;
    }
    if asteroid_map[current_pos.y as usize][current_pos.x as usize] {
      return false;
    }
  }
}

fn vaporize_asteroids(mut asteroid_map: Vec<Vec<bool>>, from: &Pos) -> Vec<Pos> {
  let mut laser_direction_index = 0;
  let laser_directions = generate_laser_directions(&asteroid_map, from);
  let map_boundary = Pos {
    x: asteroid_map[0].len() as i64,
    y: asteroid_map.len() as i64,
  };
  let mut asteroid_count = count_asteroids(&asteroid_map) - 1;
  let mut vaporized_asteroids: Vec<Pos> = Vec::new();

  while asteroid_count != 0 {
    let mut current_laser_position = *from;
    loop {
      current_laser_position.x += laser_directions[laser_direction_index].x;
      current_laser_position.y += laser_directions[laser_direction_index].y;
      if current_laser_position.x < 0
        || current_laser_position.y < 0
        || current_laser_position.x >= map_boundary.x
        || current_laser_position.y >= map_boundary.y
      {
        break;
      }
      if asteroid_map[current_laser_position.y as usize][current_laser_position.x as usize] {
        asteroid_map[current_laser_position.y as usize][current_laser_position.x as usize] = false;
        vaporized_asteroids.push(current_laser_position);
        asteroid_count -= 1;
        break;
      }
    }
    laser_direction_index += 1;
    laser_direction_index %= laser_directions.len();
  }

  vaporized_asteroids
}

fn generate_laser_directions(asteroid_map: &Vec<Vec<bool>>, from: &Pos) -> Vec<Pos> {
  let mut laser_directions: Vec<(f64, Pos)> = Vec::new();
  for y in 0..asteroid_map.len() {
    for x in 0..asteroid_map[0].len() {
      let current_pos = Pos {
        x: x as i64 - from.x,
        y: y as i64 - from.y,
      };
      if current_pos.x == 0 && current_pos.y == 0 {
        continue;
      }
      laser_directions.push((
        ((current_pos.y as f64)
          .atan2(current_pos.x as f64)
          .to_degrees()
          + 450.0)
          .rem_euclid(360.0),
        current_pos,
      ));
    }
  }

  laser_directions = laser_directions
    .iter()
    .map(|(angle, pos)| {
      let gcd = num::integer::gcd(pos.x.abs(), pos.y.abs());
      (
        *angle,
        Pos {
          x: pos.x / gcd,
          y: pos.y / gcd,
        },
      )
    })
    .collect();

  laser_directions
    .sort_by(|(left_angle, _), (right_angle, _)| left_angle.partial_cmp(right_angle).unwrap());

  laser_directions.dedup_by(|(_, left_pos), (_, right_pos)| left_pos == right_pos);

  laser_directions
    .iter()
    .map(|&(_, pos)| pos)
    .collect::<Vec<Pos>>()
}

fn count_asteroids(asteroid_map: &Vec<Vec<bool>>) -> i64 {
  let mut count = 0;
  for y in 0..asteroid_map.len() {
    for x in 0..asteroid_map[0].len() {
      if asteroid_map[y][x] {
        count += 1;
      }
    }
  }
  count
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn day_one_test_cases() {
    assert_eq!(
      find_best_asteroid(&create_asteroid_map(&vec![
        String::from("......#.#."),
        String::from("#..#.#...."),
        String::from("..#######."),
        String::from(".#.#.###.."),
        String::from(".#..#....."),
        String::from("..#....#.#"),
        String::from("#..#....#."),
        String::from(".##.#..###"),
        String::from("##...#..#."),
        String::from(".#....####"),
      ])),
      (Pos { x: 5, y: 8 }, 33)
    );

    assert_eq!(
      find_best_asteroid(&create_asteroid_map(&vec![
        String::from("#.#...#.#."),
        String::from(".###....#."),
        String::from(".#....#..."),
        String::from("##.#.#.#.#"),
        String::from("....#.#.#."),
        String::from(".##..###.#"),
        String::from("..#...##.."),
        String::from("..##....##"),
        String::from("......#..."),
        String::from(".####.###."),
      ])),
      (Pos { x: 1, y: 2 }, 35)
    );

    assert_eq!(
      find_best_asteroid(&create_asteroid_map(&vec![
        String::from(".#..#..###"),
        String::from("####.###.#"),
        String::from("....###.#."),
        String::from("..###.##.#"),
        String::from("##.##.#.#."),
        String::from("....###..#"),
        String::from("..#.#..#.#"),
        String::from("#..#.#.###"),
        String::from(".##...##.#"),
        String::from(".....#.#.."),
      ])),
      (Pos { x: 6, y: 3 }, 41)
    );

    assert_eq!(
      find_best_asteroid(&create_asteroid_map(&vec![
        String::from(".#..##.###...#######"),
        String::from("##.############..##."),
        String::from(".#.######.########.#"),
        String::from(".###.#######.####.#."),
        String::from("#####.##.#.##.###.##"),
        String::from("..#####..#.#########"),
        String::from("####################"),
        String::from("#.####....###.#.#.##"),
        String::from("##.#################"),
        String::from("#####.##.###..####.."),
        String::from("..######..##.#######"),
        String::from("####.##.####...##..#"),
        String::from(".#####..#.######.###"),
        String::from("##...#.##########..."),
        String::from("#.##########.#######"),
        String::from(".####.#.###.###.#.##"),
        String::from("....##.##.###..#####"),
        String::from(".#.#.###########.###"),
        String::from("#.#.#.#####.####.###"),
        String::from("###.##.####.##.#..##"),
      ])),
      (Pos { x: 11, y: 13 }, 210)
    );
  }

  #[test]
  fn past_two_test_cases() {
    let asteroid_map = create_asteroid_map(&vec![
      String::from(".#..##.###...#######"),
      String::from("##.############..##."),
      String::from(".#.######.########.#"),
      String::from(".###.#######.####.#."),
      String::from("#####.##.#.##.###.##"),
      String::from("..#####..#.#########"),
      String::from("####################"),
      String::from("#.####....###.#.#.##"),
      String::from("##.#################"),
      String::from("#####.##.###..####.."),
      String::from("..######..##.#######"),
      String::from("####.##.####...##..#"),
      String::from(".#####..#.######.###"),
      String::from("##...#.##########..."),
      String::from("#.##########.#######"),
      String::from(".####.#.###.###.#.##"),
      String::from("....##.##.###..#####"),
      String::from(".#.#.###########.###"),
      String::from("#.#.#.#####.####.###"),
      String::from("###.##.####.##.#..##"),
    ]);
    let (position, _) = find_best_asteroid(&asteroid_map);
    println!("{:?}", position);
    let vaporized_asteroids = vaporize_asteroids(asteroid_map.clone(), &position);
    assert_eq!(vaporized_asteroids[0], Pos { x: 11, y: 12 });
    assert_eq!(vaporized_asteroids[1], Pos { x: 12, y: 1 });
    assert_eq!(vaporized_asteroids[2], Pos { x: 12, y: 2 });
    assert_eq!(vaporized_asteroids[9], Pos { x: 12, y: 8 });
    assert_eq!(vaporized_asteroids[19], Pos { x: 16, y: 0 });
    assert_eq!(vaporized_asteroids[49], Pos { x: 16, y: 9 });
    assert_eq!(vaporized_asteroids[99], Pos { x: 10, y: 16 });
    assert_eq!(vaporized_asteroids[198], Pos { x: 9, y: 6 });
    assert_eq!(vaporized_asteroids[199], Pos { x: 8, y: 2 });
    assert_eq!(vaporized_asteroids[200], Pos { x: 10, y: 9 });
    assert_eq!(vaporized_asteroids[298], Pos { x: 11, y: 1 });
  }
}
