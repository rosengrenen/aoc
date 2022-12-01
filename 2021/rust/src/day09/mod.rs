use std::collections::HashSet;

use aoc_util::{Solver, SolverOutput};

#[derive(Default)]
pub struct Day9;

impl Solver for Day9 {
  fn part_one(&self, input: &str) -> SolverOutput {
    let a = parse_(input);
    let mut k = 0;
    for y in 0..a.len() {
      for x in 0..a[0].len() {
        if x < a[0].len() - 1 && a[y][x] >= a[y][x + 1] {
          continue;
        }

        if x > 0 && a[y][x] >= a[y][x - 1] {
          continue;
        }

        if y < a.len() - 1 && a[y][x] >= a[y + 1][x] {
          continue;
        }

        if y > 0 && a[y][x] >= a[y - 1][x] {
          continue;
        }

        k += a[y][x] as i64 + 1;
      }
    }

    SolverOutput::Num(k)
  }

  fn part_two(&self, input: &str) -> SolverOutput {
    let a = parse_(input);
    let b = find_low_points(&a);
    let mut basin_sizes: Vec<_> = b.into_iter().map(|b| calculate_basin(&a, b)).collect();
    basin_sizes.sort_by_key(|f| -f);

    SolverOutput::Num(basin_sizes.iter().take(3).product())
  }
}

fn calculate_basin(map: &Vec<Vec<u8>>, (x, y): (usize, usize)) -> i64 {
  let mut basin_points = HashSet::new();
  basin_points.insert((x, y));

  loop {
    let mut new_basin_points = HashSet::new();
    for &(x, y) in basin_points.iter() {
      if y < map.len() - 1
        && !basin_points.contains(&(x, y + 1))
        && map[y + 1][x] != 9
        && map[y + 1][x] > map[y][x]
      {
        new_basin_points.insert((x, y + 1));
      }

      if y > 0
        && !basin_points.contains(&(x, y - 1))
        && map[y - 1][x] != 9
        && map[y - 1][x] > map[y][x]
      {
        new_basin_points.insert((x, y - 1));
      }

      if x < map[0].len() - 1
        && !basin_points.contains(&(x + 1, y))
        && map[y][x + 1] != 9
        && map[y][x + 1] > map[y][x]
      {
        new_basin_points.insert((x + 1, y));
      }

      if x > 0
        && !basin_points.contains(&(x - 1, y))
        && map[y][x - 1] != 9
        && map[y][x - 1] > map[y][x]
      {
        new_basin_points.insert((x - 1, y));
      }
    }

    basin_points.extend(new_basin_points.iter());

    if new_basin_points.is_empty() {
      break;
    }
  }

  basin_points.len() as i64
}

fn find_low_points(map: &Vec<Vec<u8>>) -> Vec<(usize, usize)> {
  let mut low_points = vec![];

  for y in 0..map.len() {
    for x in 0..map[0].len() {
      if x < map[0].len() - 1 && map[y][x] >= map[y][x + 1] {
        continue;
      }

      if x > 0 && map[y][x] >= map[y][x - 1] {
        continue;
      }

      if y < map.len() - 1 && map[y][x] >= map[y + 1][x] {
        continue;
      }

      if y > 0 && map[y][x] >= map[y - 1][x] {
        continue;
      }

      low_points.push((x, y));
    }
  }

  low_points
}

fn parse_(input: &str) -> Vec<Vec<u8>> {
  input
    .lines()
    .map(|line| line.as_bytes().iter().map(|b| b - b'0').collect())
    .collect()
}

#[cfg(test)]
mod benches {
  use super::*;
  use aoc_util::get_input;
  use test::{black_box, Bencher};

  #[bench]
  fn parse(bencher: &mut Bencher) {
    let input = get_input(2021, 9).unwrap();
    bencher.iter(|| parse_(black_box(&input)));
  }
}
