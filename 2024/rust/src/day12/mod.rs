use std::collections::{HashMap, HashSet};

use aoc_util::{Solver, SolverOutput};

#[derive(Default)]
pub struct Day12;

impl Solver for Day12 {
  fn part_one(&self, input: &str) -> SolverOutput {
    SolverOutput::Num(
      groups(parse(input))
        .into_iter()
        .map(|group| group.len() * perimeter(group).len())
        .sum::<usize>() as i64,
    )
  }

  fn part_two(&self, input: &str) -> SolverOutput {
    SolverOutput::Num(
      groups(parse(input))
        .into_iter()
        .map(|group| group.len() * sides(group))
        .sum::<usize>() as i64,
    )
  }
}

const ADJ4: [(i64, i64); 4] = [(0, -1), (1, 0), (0, 1), (-1, 0)];

fn perimeter(group: HashSet<(i64, i64)>) -> HashSet<((i64, i64), i64)> {
  let mut fence = HashSet::new();
  for &(x, y) in group.iter() {
    for ((dx, dy), dir) in ADJ4.into_iter().zip([0, 1, 2, 3]) {
      if !group.contains(&(x + dx, y + dy)) {
        fence.insert(((x + dx, y + dy), dir));
      }
    }
  }
  fence
}

fn sides(group: HashSet<(i64, i64)>) -> usize {
  let mut p = perimeter(group);
  let mut sides = 0;
  loop {
    if p.is_empty() {
      return sides;
    }

    let pos = *p.iter().next().unwrap();
    find_side(&mut p, pos);
    sides += 1;
  }
}

fn find_side(perimeter: &mut HashSet<((i64, i64), i64)>, pos: ((i64, i64), i64)) {
  perimeter.remove(&pos);
  let (pos, dir) = pos;
  for d in ADJ4 {
    let pos = (pos.0 + d.0, pos.1 + d.1);
    if perimeter.contains(&(pos, dir)) {
      find_side(perimeter, (pos, dir));
    }
  }
}

fn groups(mut grid: HashMap<(i64, i64), u8>) -> Vec<HashSet<(i64, i64)>> {
  let mut groups = Vec::new();
  loop {
    if grid.is_empty() {
      return groups;
    }

    let mut group = HashSet::new();
    let pos = *grid.keys().next().unwrap();
    find_group(&mut grid, &mut group, pos);
    groups.push(group);
  }
}

fn find_group(
  grid: &mut HashMap<(i64, i64), u8>,
  group: &mut HashSet<(i64, i64)>,
  pos: (i64, i64),
) {
  let v = *grid.get(&pos).unwrap();
  grid.remove(&pos);
  group.insert(pos);
  for d in ADJ4 {
    let pos = (pos.0 + d.0, pos.1 + d.1);
    if let Some(nv) = grid.get(&pos) {
      if *nv == v {
        find_group(grid, group, pos);
      }
    }
  }
}

fn parse(input: &str) -> HashMap<(i64, i64), u8> {
  let mut grid = HashMap::new();
  for (y, line) in input.lines().enumerate() {
    for (x, b) in line.bytes().enumerate() {
      grid.insert((x as i64, y as i64), b);
    }
  }
  grid
}
