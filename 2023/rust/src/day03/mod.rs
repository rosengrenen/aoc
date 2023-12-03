use std::collections::{HashMap, HashSet};

use aoc_util::{Solver, SolverOutput};

#[derive(Default)]
pub struct Day3;

impl Solver for Day3 {
  fn part_one(&self, input: &str) -> SolverOutput {
    let schematic = parse_schematic(input);
    let mut sum: i64 = 0;
    for y in 0..schematic.rows.len() {
      let mut current_num = None;
      let mut symbol_around = false;
      for x in 0..schematic.rows[0].len() {
        match schematic.rows[y][x] {
          A::Number(n) => {
            current_num = current_num
              .map(|num| num * 10 + n as i64)
              .or(Some(n as i64));
            symbol_around |= schematic.has_symbol_around(y as _, x as _);
          }
          _ => {
            if let Some(num) = current_num {
              if symbol_around {
                sum += num;
              }

              current_num = None;
              symbol_around = false;
            }
          }
        }
      }

      if let Some(num) = current_num {
        if symbol_around {
          sum += num;
        }
      }
    }

    SolverOutput::Num(sum.into())
  }

  fn part_two(&self, input: &str) -> SolverOutput {
    let schematic = parse_schematic(input);
    let mut gears = HashMap::new();
    for y in 0..schematic.rows.len() {
      let mut current_num = None;
      let mut gears_around = HashSet::new();
      for x in 0..schematic.rows[0].len() {
        match schematic.rows[y][x] {
          A::Number(n) => {
            current_num = current_num
              .map(|num| num * 10 + n as i64)
              .or(Some(n as i64));
            for gear_pos in schematic.get_gears_around(y as _, x as _) {
              gears_around.insert(gear_pos);
            }
          }
          _ => {
            if let Some(num) = current_num {
              for gear_pos in gears_around.iter() {
                gears.entry(*gear_pos).or_insert(vec![]).push(num);
              }

              current_num = None;
              gears_around.clear();
            }
          }
        }
      }

      if let Some(num) = current_num {
        for gear_pos in gears_around {
          gears.entry(gear_pos).or_insert(vec![]).push(num);
        }
      }
    }

    SolverOutput::Num(
      gears
        .into_iter()
        .map(|(_, gear_parts)| gear_parts)
        .filter(|parts| parts.len() == 2)
        .fold(0, |sum, part| sum + part[0] * part[1]),
    )
  }
}

#[derive(Debug)]
struct Schematic {
  rows: Vec<Vec<A>>,
}

impl Schematic {
  fn has_symbol_around(&self, col: isize, row: isize) -> bool {
    for x in -1..=1 {
      for y in -1..=1 {
        if self.get_symbol(row + y, col + x).is_some() {
          return true;
        }
      }
    }

    false
  }

  fn get_gears_around(&self, col: isize, row: isize) -> Vec<(usize, usize)> {
    let mut symbols = vec![];
    for x in -1..=1 {
      for y in -1..=1 {
        if let Some(b'*') = self.get_symbol(row + y, col + x) {
          symbols.push(((row + y) as _, (col + x) as _));
        }
      }
    }

    symbols
  }

  fn get_symbol(&self, col: isize, row: isize) -> Option<u8> {
    if row < 0 || row >= self.rows.len() as _ {
      return None;
    }

    if col < 0 || col >= self.rows[0].len() as _ {
      return None;
    }

    if let A::Symbol(s) = self.rows[row as usize][col as usize] {
      Some(s)
    } else {
      None
    }
  }
}

#[derive(Debug)]
enum A {
  Number(u8),
  Dot,
  Symbol(u8),
}

fn parse_schematic<'a>(input: &'a str) -> Schematic {
  let rows = input
    .lines()
    .map(|line| {
      line
        .bytes()
        .map(|b| match b {
          b'0'..=b'9' => A::Number(b - b'0'),
          b'.' => A::Dot,
          b => A::Symbol(b),
        })
        .collect::<Vec<_>>()
    })
    .collect::<Vec<_>>();
  Schematic { rows }
}
