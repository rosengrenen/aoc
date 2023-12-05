use std::collections::HashMap;

use aoc_util::{Solver, SolverOutput};

#[derive(Default)]
pub struct Day5;

impl Solver for Day5 {
  fn part_one(&self, input: &str) -> SolverOutput {
    let (seeds, maps) = parse(input);
    SolverOutput::Num(
      seeds
        .into_iter()
        .map(|seed| maps.seed_location(seed))
        .min()
        .unwrap(),
    )
  }

  fn part_two(&self, input: &str) -> SolverOutput {
    let (seed_ranges, maps) = parse(input);
    let mut seeds = Vec::new();
    for i in (0..seed_ranges.len()).step_by(2) {
      for seed in seed_ranges[i]..seed_ranges[i] + seed_ranges[i + 1] {
        seeds.push(seed);
      }
    }
    SolverOutput::Num(
      seeds
        .into_iter()
        .map(|seed| maps.seed_location(seed))
        .min()
        .unwrap(),
    )
  }
}

#[derive(Debug)]
struct Maps<'a> {
  maps: HashMap<&'a str, Map<'a>>,
}

impl<'a> Maps<'a> {
  fn seed_location(&self, seed: i64) -> i64 {
    let mut map = self.maps.get("seed").unwrap();
    let mut src = seed;
    loop {
      let next_src = map.translate(src);
      if map.name == "location" {
        return next_src;
      }

      map = self.maps.get(map.name).unwrap();
      src = next_src;
    }
  }
}

#[derive(Debug)]
struct Map<'a> {
  name: &'a str,
  ranges: Vec<Range>,
}

impl<'a> Map<'a> {
  fn translate(&self, src: i64) -> i64 {
    match self.ranges.iter().find_map(|range| range.translate(src)) {
      Some(dst) => dst,
      None => src,
    }
  }
}

#[derive(Debug)]
struct Range {
  src: i64,
  dst: i64,
  len: i64,
}

impl Range {
  fn translate(&self, src: i64) -> Option<i64> {
    if (self.src..self.src + self.len).contains(&src) {
      return Some(src + (self.dst - self.src));
    }

    None
  }
}

fn parse<'a>(input: &'a str) -> (Vec<i64>, Maps<'a>) {
  let (seeds, maps) = input.split_once("\n\n").unwrap();
  let seeds = seeds
    .strip_prefix("seeds: ")
    .unwrap()
    .trim()
    .split_whitespace()
    .map(|n| n.parse().unwrap())
    .collect::<Vec<i64>>();

  let maps = maps
    .split("\n\n")
    .map(|map| {
      let mut lines = map.lines();
      let mut names = lines
        .next()
        .unwrap()
        .strip_suffix(" map:")
        .unwrap()
        .split("-")
        .step_by(2);
      let from = names.next().unwrap();
      let to = names.next().unwrap();
      let ranges = lines
        .map(|range| {
          let mut parts = range.split_whitespace();
          Range {
            dst: parts.next().unwrap().parse().unwrap(),
            src: parts.next().unwrap().parse().unwrap(),
            len: parts.next().unwrap().parse().unwrap(),
          }
        })
        .collect::<Vec<_>>();
      (from, Map { name: to, ranges })
    })
    .collect::<HashMap<_, _>>();
  (seeds, Maps { maps })
}
