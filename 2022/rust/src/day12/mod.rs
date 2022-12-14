use std::collections::{BinaryHeap, HashSet};

use aoc_util::{Solver, SolverOutput};

#[derive(Default)]
pub struct Day12;

impl Solver for Day12 {
  fn part_one(&self, input: &str) -> SolverOutput {
    let (map, start, end) = parse_map(input);
    SolverOutput::Num(find_path(&map, start, end).unwrap())
  }

  fn part_two(&self, input: &str) -> SolverOutput {
    let (map, _, end) = parse_map(input);
    SolverOutput::Num(
      map
        .tiles
        .iter()
        .enumerate()
        .map(|(y, row)| {
          row
            .iter()
            .enumerate()
            .filter(|(_, tile)| **tile == 0)
            .filter_map(|(x, tile)| {
              find_path(
                &map,
                Pos {
                  x: x as i64,
                  y: y as i64,
                },
                end,
              )
            })
            .min()
            .unwrap()
        })
        .min()
        .unwrap(),
    )
  }
}

fn find_path(map: &Map<i64>, start: Pos, end: Pos) -> Option<i64> {
  let mut visited = HashSet::new();
  visited.insert(start);
  let mut queue = vec![start];
  let mut parents: Map<Option<Pos>> = map.map(|_| None);

  while let Some(pos) = queue.pop() {
    if pos == end {
      let mut cur = end;
      let mut length = 0;
      while cur != start {
        length += 1;
        cur = parents.get_tile(cur).unwrap().unwrap();
      }

      return Some(length);
    }

    let height = map.get_tile(pos).unwrap();

    let mut check_pos = |adj_pos: Pos| {
      if !visited.contains(&adj_pos) {
        if let Some(adj_height) = map.get_tile(adj_pos) {
          if adj_height <= height + 1 {
            visited.insert(adj_pos);
            parents.set_tile(adj_pos, Some(pos));
            queue.insert(0, adj_pos);
          }
        }
      }
    };

    check_pos(pos.up());
    check_pos(pos.right());
    check_pos(pos.down());
    check_pos(pos.left());
  }

  None
}

fn parse_map(input: &str) -> (Map<i64>, Pos, Pos) {
  let mut start = None;
  let mut end = None;
  let mut map = vec![];
  for (y, line) in input.lines().enumerate() {
    let y = y as i64;
    let mut map_row = vec![];
    for (x, b) in line.bytes().enumerate() {
      let x = x as i64;
      let height = match b {
        b'S' => {
          start = Some(Pos { x, y });
          0
        }
        b'E' => {
          end = Some(Pos { x, y });
          b'z' - b'a'
        }
        n => n - b'a',
      };
      map_row.push(height as i64);
    }

    map.push(map_row);
  }

  (Map { tiles: map }, start.unwrap(), end.unwrap())
}

#[derive(Debug)]
struct Map<T> {
  tiles: Vec<Vec<T>>,
}

impl<T: Copy> Map<T> {
  fn get_tile(&self, pos: Pos) -> Option<T> {
    if pos.x < 0 || pos.x >= self.tiles[0].len() as i64 {
      return None;
    }

    if pos.y < 0 || pos.y >= self.tiles.len() as i64 {
      return None;
    }

    Some(self.tiles[pos.y as usize][pos.x as usize])
  }

  fn set_tile(&mut self, pos: Pos, val: T) {
    self.tiles[pos.y as usize][pos.x as usize] = val;
  }

  fn map<F, U>(&self, f: F) -> Map<U>
  where
    F: Fn(&T) -> U + Copy,
  {
    Map {
      tiles: self
        .tiles
        .iter()
        .map(|row| row.iter().map(f).collect())
        .collect(),
    }
  }
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
struct Pos {
  x: i64,
  y: i64,
}

impl Pos {
  fn up(&self) -> Self {
    Self {
      x: self.x,
      y: self.y - 1,
    }
  }

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
