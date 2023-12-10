use std::collections::HashSet;

use aoc_util::{Solver, SolverOutput};

#[derive(Default)]
pub struct Day10;

impl Solver for Day10 {
  fn part_one(&self, input: &str) -> SolverOutput {
    let (start_tile, tiles) = parse(input);
    let mut current_tile = find_next_pipe(&tiles, start_tile, start_tile);
    let mut prev_tile = start_tile;
    let mut steps = 1;
    loop {
      let next_current_tile = find_next_pipe(&tiles, current_tile, prev_tile);
      prev_tile = current_tile;
      current_tile = next_current_tile;
      if current_tile == start_tile {
        break;
      }

      steps += 1;
    }

    SolverOutput::Num((steps + 1) / 2)
  }

  fn part_two(&self, input: &str) -> SolverOutput {
    let (start_tile, tiles) = parse(input);
    let mut current_tile = find_next_pipe(&tiles, start_tile, start_tile);
    let mut prev_tile = start_tile;
    let mut pipes = vec![start_tile];
    loop {
      let next_current_tile = find_next_pipe(&tiles, current_tile, prev_tile);
      prev_tile = current_tile;
      current_tile = next_current_tile;
      pipes.push(current_tile);
      if current_tile == start_tile {
        break;
      }
    }

    let left_turns = pipes
      .iter()
      .map(|(x, y)| tiles[*y][*x])
      .filter(|tile| match tile {
        Tile::NorthEastPipe => true,
        Tile::SouthWestPipe => true,
        _ => false,
      })
      .count();
    let right_turns = pipes
      .iter()
      .map(|(x, y)| tiles[*y][*x])
      .filter(|tile| match tile {
        Tile::NorthWestPipe => true,
        Tile::SouthEastPipe => true,
        _ => false,
      })
      .count();
    println!("{} {}", left_turns, right_turns);

    SolverOutput::Num(0)
  }
}

fn find_next_pipe(
  tiles: &[Vec<Tile>],
  current: (usize, usize),
  prev: (usize, usize),
) -> (usize, usize) {
  let (current_x, current_y) = current;
  let current_tile = tiles[current_y][current_x];
  if current_y > 0
    && (current_x, current_y - 1) != prev
    && current_tile.connects_north(tiles[current_y - 1][current_x])
  {
    return (current_x, current_y - 1);
  }

  if current_x < tiles[0].len() - 1
    && (current_x + 1, current_y) != prev
    && current_tile.connects_east(tiles[current_y][current_x + 1])
  {
    return (current_x + 1, current_y);
  }

  if current_y < tiles.len() - 1
    && (current_x, current_y + 1) != prev
    && current_tile.connects_south(tiles[current_y + 1][current_x])
  {
    return (current_x, current_y + 1);
  }

  if current_x > 0
    && (current_x - 1, current_y) != prev
    && current_tile.connects_west(tiles[current_y][current_x - 1])
  {
    return (current_x - 1, current_y);
  }

  unreachable!()
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Tile {
  VerticalPipe,
  HorizontalPipe,
  NorthEastPipe,
  NorthWestPipe,
  SouthWestPipe,
  SouthEastPipe,
  Ground,
  Start,
}

impl Tile {
  fn is_north(&self) -> bool {
    match *self {
      Self::VerticalPipe => true,
      Self::NorthEastPipe => true,
      Self::NorthWestPipe => true,
      _ => false,
    }
  }

  fn is_east(&self) -> bool {
    match *self {
      Self::HorizontalPipe => true,
      Self::NorthEastPipe => true,
      Self::SouthEastPipe => true,
      _ => false,
    }
  }

  fn is_south(&self) -> bool {
    match *self {
      Self::VerticalPipe => true,
      Self::SouthEastPipe => true,
      Self::SouthWestPipe => true,
      _ => false,
    }
  }

  fn is_west(&self) -> bool {
    match *self {
      Self::HorizontalPipe => true,
      Self::NorthWestPipe => true,
      Self::SouthWestPipe => true,
      _ => false,
    }
  }

  fn connects_north(&self, other: Tile) -> bool {
    (*self == Self::Start || self.is_north()) && (other == Self::Start || other.is_south())
  }

  fn connects_east(&self, other: Tile) -> bool {
    (*self == Self::Start || self.is_east()) && (other == Self::Start || other.is_west())
  }

  fn connects_south(&self, other: Tile) -> bool {
    (*self == Self::Start || self.is_south()) && (other == Self::Start || other.is_north())
  }

  fn connects_west(&self, other: Tile) -> bool {
    (*self == Self::Start || self.is_west()) && (other == Self::Start || other.is_east())
  }
}

fn parse<'a>(input: &'a str) -> ((usize, usize), Vec<Vec<Tile>>) {
  let mut start_x = 0;
  let mut start_y = 0;
  let tiles = input
    .lines()
    .enumerate()
    .map(|(y, line)| {
      line
        .chars()
        .enumerate()
        .map(|(x, c)| match c {
          '|' => Tile::VerticalPipe,
          '-' => Tile::HorizontalPipe,
          'L' => Tile::NorthEastPipe,
          'J' => Tile::NorthWestPipe,
          '7' => Tile::SouthWestPipe,
          'F' => Tile::SouthEastPipe,
          '.' => Tile::Ground,
          'S' => {
            start_x = x;
            start_y = y;
            Tile::Start
          }
          _ => unreachable!(),
        })
        .collect()
    })
    .collect();
  ((start_x, start_y), tiles)
}
