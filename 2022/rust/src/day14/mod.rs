use std::collections::HashMap;

use aoc_util::{Solver, SolverOutput};

#[derive(Default)]
pub struct Day14;

impl Solver for Day14 {
  fn part_one(&self, input: &str) -> SolverOutput {
    let paths = parse_paths(input);
    let mut grid = draw_paths(&paths);
    simulate_grid(&mut grid, false);
    SolverOutput::Num(
      grid
        .into_values()
        .filter(|&tile| tile == Tile::Sand(true))
        .count() as i64,
    )
  }

  fn part_two(&self, input: &str) -> SolverOutput {
    let paths = parse_paths(input);
    let mut grid = draw_paths(&paths);
    simulate_grid(&mut grid, true);
    SolverOutput::Num(
      grid
        .into_values()
        .filter(|&tile| tile == Tile::Sand(true))
        .count() as i64,
    )
  }
}

fn simulate_grid(grid: &mut HashMap<Pos, Tile>, has_floor: bool) {
  let (_, grid_max) = grid_bounds(&grid);
  let start_pos = Pos { x: 500, y: 0 };
  loop {
    if grid.get(&start_pos) == Some(&Tile::Sand(true)) {
      break;
    } else {
      grid.insert(start_pos, Tile::Sand(false));
    }

    let (min, max) = grid_bounds(&grid);
    if max.y > grid_max.y + 1 {
      break;
    }
    for y in (min.y..=max.y).rev() {
      for x in min.x..=max.x {
        let pos = Pos { x, y };
        if let Some(Tile::Sand(false)) = grid.get(&pos) {
          if has_floor && y == grid_max.y + 1 {
            grid.insert(pos, Tile::Sand(true));
          } else if !grid.contains_key(&pos.down()) {
            grid.remove(&pos);
            grid.insert(pos.down(), Tile::Sand(false));
          } else if !grid.contains_key(&pos.down().left()) {
            grid.remove(&pos);
            grid.insert(pos.down().left(), Tile::Sand(false));
          } else if !grid.contains_key(&pos.down().right()) {
            grid.remove(&pos);
            grid.insert(pos.down().right(), Tile::Sand(false));
          } else {
            grid.insert(pos, Tile::Sand(true));
          }
        }
      }
    }
  }
}

fn draw_paths(paths: &Vec<Vec<Pos>>) -> HashMap<Pos, Tile> {
  let mut grid = HashMap::new();
  for path in paths {
    let mut path_iter = path.iter();
    let mut start = path_iter.next().unwrap();
    while let Some(end) = path_iter.next() {
      let min_x = start.x.min(end.x);
      let max_x = start.x.max(end.x);
      let min_y = start.y.min(end.y);
      let max_y = start.y.max(end.y);
      for x in min_x..=max_x {
        for y in min_y..=max_y {
          grid.insert(Pos { x, y }, Tile::Rock);
        }
      }

      start = end;
    }
  }

  grid
}

fn parse_paths(input: &str) -> Vec<Vec<Pos>> {
  input
    .lines()
    .map(|line| {
      line
        .split(" -> ")
        .map(|pos| {
          let (x, y) = pos.split_once(",").unwrap();
          Pos {
            x: x.parse().unwrap(),
            y: y.parse().unwrap(),
          }
        })
        .collect()
    })
    .collect()
}

fn grid_bounds(grid: &HashMap<Pos, Tile>) -> (Pos, Pos) {
  let min_x = grid.keys().map(|pos| pos.x).min().unwrap();
  let max_x = grid.keys().map(|pos| pos.x).max().unwrap();
  let min_y = grid.keys().map(|pos| pos.y).min().unwrap();
  let max_y = grid.keys().map(|pos| pos.y).max().unwrap();
  let min = Pos { x: min_x, y: min_y };
  let max = Pos { x: max_x, y: max_y };
  (min, max)
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
struct Pos {
  x: i64,
  y: i64,
}

impl Pos {
  fn right(&self) -> Self {
    Self {
      x: self.x + 1,
      y: self.y,
    }
  }

  fn down(&self) -> Self {
    Self {
      x: self.x,
      y: self.y + 1,
    }
  }

  fn left(&self) -> Self {
    Self {
      x: self.x - 1,
      y: self.y,
    }
  }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum Tile {
  Rock,
  Sand(bool),
}

fn _print_grid(grid: &HashMap<Pos, Tile>) {
  let min_x = grid.keys().map(|&Pos { x, .. }| x).min().unwrap();
  let max_x = grid.keys().map(|&Pos { x, .. }| x).max().unwrap();
  let min_y = grid.keys().map(|&Pos { y, .. }| y).min().unwrap();
  let max_y = grid.keys().map(|&Pos { y, .. }| y).max().unwrap();
  for y in min_y..=max_y {
    for x in min_x..=max_x {
      if let Some(tile) = grid.get(&Pos { x, y }) {
        match tile {
          Tile::Rock => print!("#"),
          Tile::Sand(true) => print!("O"),
          Tile::Sand(false) => print!("o"),
        }
      } else {
        print!(".");
      }
    }
    println!();
  }
}
