use std::{cmp::Ordering, collections::HashSet};

use aoc_util::{Solver, SolverOutput};

#[derive(Default)]
pub struct Day9;

impl Solver for Day9 {
  fn part_one(&self, input: &str) -> SolverOutput {
    let movs = parse_movs(input);
    SolverOutput::Num(sim_ropes::<2>(movs))
  }

  fn part_two(&self, input: &str) -> SolverOutput {
    let movs = parse_movs(input);
    SolverOutput::Num(sim_ropes::<10>(movs))
  }
}

fn parse_movs<'a>(input: &'a str) -> impl Iterator<Item = (Dir, i64)> + 'a {
  input
    .lines()
    .map(|line| (line.as_bytes()[0].into(), line[2..].parse().unwrap()))
}

fn sim_ropes<const N: usize>(movs: impl Iterator<Item = (Dir, i64)>) -> i64 {
  let mut visited = HashSet::new();
  let mut knots = [Pos::default(); N];
  visited.insert(knots[0]);

  for (dir, steps) in movs {
    for _ in 0..steps {
      match dir {
        Dir::Up => knots[0].y += 1,
        Dir::Right => knots[0].x += 1,
        Dir::Down => knots[0].y -= 1,
        Dir::Left => knots[0].x -= 1,
      }
      for i in 1..N {
        let dx = (knots[i - 1].x - knots[i].x).abs();
        let dy = (knots[i - 1].y - knots[i].y).abs();

        if dx.max(dy) >= 2 {
          match knots[i - 1].x.cmp(&knots[i].x) {
            Ordering::Less => knots[i].x += -1,
            Ordering::Equal => knots[i].x += 0,
            Ordering::Greater => knots[i].x += 1,
          };
          match knots[i - 1].y.cmp(&knots[i].y) {
            Ordering::Less => knots[i].y += -1,
            Ordering::Equal => knots[i].y += 0,
            Ordering::Greater => knots[i].y += 1,
          };
        }
      }

      visited.insert(knots[N - 1]);
    }
  }

  visited.len() as i64
}

#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq)]
struct Pos {
  x: i64,
  y: i64,
}

enum Dir {
  Up,
  Right,
  Down,
  Left,
}

impl From<u8> for Dir {
  fn from(b: u8) -> Self {
    match b {
      b'U' => Self::Up,
      b'R' => Self::Right,
      b'D' => Self::Down,
      b'L' => Self::Left,
      _ => panic!("Invalid direction"),
    }
  }
}
