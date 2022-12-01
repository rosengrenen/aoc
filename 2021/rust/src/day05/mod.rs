use aoc_util::{Solver, SolverOutput};

#[derive(Default)]
pub struct Day5;

impl Solver for Day5 {
  fn part_one(&self, input: &str) -> SolverOutput {
    let lines = parse_lines(input);
    let (max_x, max_y) = calculate_bounds(&lines);
    let mut map = vec![vec![0; max_x + 1]; max_y + 1];
    let lines = lines
      .iter()
      .filter(|Line { from, to }| from.x == to.x || from.y == to.y);
    for line in lines {
      for Point { x, y } in line.points() {
        map[y as usize][x as usize] += 1
      }
    }

    SolverOutput::Num(
      map
        .into_iter()
        .flat_map(|row| row.into_iter())
        .filter(|&cell| cell >= 2)
        .count() as i64,
    )
  }

  fn part_two(&self, input: &str) -> SolverOutput {
    let lines = parse_lines(input);
    let (max_x, max_y) = calculate_bounds(&lines);
    let mut map = vec![vec![0; max_x + 1]; max_y + 1];
    let lines = lines.iter().filter(|Line { from, to }| {
      from.x == to.x || from.y == to.y || (from.x - to.x).abs() == (from.y - to.y).abs()
    });
    for line in lines {
      for Point { x, y } in line.points() {
        map[y as usize][x as usize] += 1
      }
    }

    SolverOutput::Num(
      map
        .into_iter()
        .flat_map(|row| row.into_iter())
        .filter(|&cell| cell >= 2)
        .count() as i64,
    )
  }
}

#[derive(Debug)]
struct Line {
  from: Point,
  to: Point,
}

impl Line {
  fn points(&self) -> impl Iterator<Item = Point> {
    PointIter::new(self.from, self.to)
  }
}

#[derive(Clone, Copy, Debug)]
struct Point {
  x: i64,
  y: i64,
}

fn calculate_bounds(lines: &[Line]) -> (usize, usize) {
  lines.iter().fold(
    (usize::MIN, usize::MIN),
    |(max_x, max_y), Line { from, to }| {
      (
        max_x.max(from.x.max(to.x) as usize),
        max_y.max(from.y.max(to.y) as usize),
      )
    },
  )
}

struct PointIter {
  current: Point,
  delta_x: i64,
  delta_y: i64,
  rem_steps: i64,
}

impl PointIter {
  fn new(from: Point, to: Point) -> Self {
    Self {
      current: from,
      delta_x: to.x.cmp(&from.x) as i64,
      delta_y: to.y.cmp(&from.y) as i64,
      rem_steps: (from.x - to.x).abs().max((from.y - to.y).abs()) + 1,
    }
  }
}

impl Iterator for PointIter {
  type Item = Point;

  fn next(&mut self) -> Option<Self::Item> {
    if self.rem_steps == 0 {
      return None;
    }

    self.rem_steps -= 1;
    let next = self.current;
    self.current.x += self.delta_x;
    self.current.y += self.delta_y;
    Some(next)
  }
}

fn parse_lines(input: &str) -> Vec<Line> {
  let parse_point = |input: &str| -> Point {
    let (x, y) = input.split_once(",").unwrap();
    Point {
      x: x.parse().unwrap(),
      y: y.parse().unwrap(),
    }
  };

  input
    .lines()
    .map(|line| line.split_once("->").unwrap())
    .map(|(from, to)| Line {
      from: parse_point(from.trim()),
      to: parse_point(to.trim()),
    })
    .collect()
}

#[cfg(test)]
mod benches {
  use super::*;
  use aoc_util::get_input;
  use test::{black_box, Bencher};

  #[bench]
  fn parse(bencher: &mut Bencher) {
    let input = get_input(2021, 5).unwrap();
    bencher.iter(|| parse_lines(black_box(&input)));
  }
}
