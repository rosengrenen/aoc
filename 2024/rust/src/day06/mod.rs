use std::collections::HashSet;

use aoc_util::{Solver, SolverOutput};

#[derive(Default)]
pub struct Day6;

impl Solver for Day6 {
  fn part_one(&self, input: &str) -> SolverOutput {
    let (grid, mut pos) = parse(input);
    let mut dir = Dir::North;
    let mut visited = HashSet::new();
    loop {
      if pos.x < 0 || pos.y < 0 || pos.x >= grid[0].len() as isize || pos.y >= grid.len() as isize {
        break;
      }

      visited.insert(pos);
      let next_pos = pos.step(dir);
      if next_pos.x < 0
        || next_pos.y < 0
        || next_pos.x >= grid[0].len() as isize
        || next_pos.y >= grid.len() as isize
      {
        break;
      }

      let next_tile = grid[next_pos.y as usize][next_pos.x as usize];
      if next_tile == b'#' {
        dir = dir.rotate();
      } else {
        pos = next_pos;
      }
    }
    SolverOutput::Num(visited.len() as i64)
  }

  fn part_two(&self, input: &str) -> SolverOutput {
    let (mut grid, mut pos) = parse(input);
    let mut dir = Dir::North;
    let mut sum = 0;
    let mut visited = HashSet::new();
    loop {
      if pos.x < 0 || pos.y < 0 || pos.x >= grid[0].len() as isize || pos.y >= grid.len() as isize {
        break;
      }

      let next_pos = pos.step(dir);
      if next_pos.x < 0
        || next_pos.y < 0
        || next_pos.x >= grid[0].len() as isize
        || next_pos.y >= grid.len() as isize
      {
        break;
      }

      let next_tile = grid[next_pos.y as usize][next_pos.x as usize];
      grid[pos.y as usize][pos.x as usize] = b'x';
      let mut new_pos = pos;
      let mut new_dir = dir;
      if next_tile == b'#' {
        new_dir = dir.rotate();
      } else if next_tile != b'x' {
        if loops(visited.clone(), &grid, pos, dir, next_pos) {
          sum += 1;
        }

        new_pos = next_pos;
      } else {
        new_pos = next_pos;
      }

      visited.insert((pos, dir));
      pos = new_pos;
      dir = new_dir;
    }
    SolverOutput::Num(sum)
  }
}

fn loops(
  mut visited: HashSet<(Pos, Dir)>,
  grid: &[Vec<u8>],
  mut pos: Pos,
  mut dir: Dir,
  obstacle: Pos,
) -> bool {
  loop {
    if pos.x < 0 || pos.y < 0 || pos.x >= grid[0].len() as isize || pos.y >= grid.len() as isize {
      return false;
    }

    if visited.contains(&(pos, dir)) {
      return true;
    }

    visited.insert((pos, dir));
    let next_pos = pos.step(dir);
    if next_pos.x < 0
      || next_pos.y < 0
      || next_pos.x >= grid[0].len() as isize
      || next_pos.y >= grid.len() as isize
    {
      return false;
    }

    let next_tile = grid[next_pos.y as usize][next_pos.x as usize];
    if next_tile == b'#' || next_pos == obstacle {
      dir = dir.rotate();
    } else {
      pos = pos.step(dir);
    }
  }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
struct Pos {
  x: isize,
  y: isize,
}

impl Pos {
  fn step(self, dir: Dir) -> Self {
    match dir {
      Dir::North => Self {
        y: self.y - 1,
        ..self
      },
      Dir::East => Self {
        x: self.x + 1,
        ..self
      },
      Dir::South => Self {
        y: self.y + 1,
        ..self
      },
      Dir::West => Self {
        x: self.x - 1,
        ..self
      },
    }
  }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
enum Dir {
  North,
  East,
  South,
  West,
}

impl Dir {
  fn rotate(self) -> Self {
    match self {
      Self::North => Self::East,
      Self::East => Self::South,
      Self::South => Self::West,
      Self::West => Self::North,
    }
  }
}

fn parse(input: &str) -> (Vec<Vec<u8>>, Pos) {
  let mut start = None;
  (
    input
      .lines()
      .enumerate()
      .map(|(y, line)| {
        if let Some((x, _)) = line.bytes().enumerate().find(|(_, b)| *b == b'^') {
          start = Some(Pos {
            x: x as isize,
            y: y as isize,
          })
        }

        line.bytes().collect()
      })
      .collect(),
    start.unwrap(),
  )
}
