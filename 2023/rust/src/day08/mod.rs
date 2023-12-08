use std::collections::HashMap;

use aoc_util::{Solver, SolverOutput};

#[derive(Default)]
pub struct Day8;

impl Solver for Day8 {
  fn part_one(&self, input: &str) -> SolverOutput {
    let (directions, locations) = parse(input);
    SolverOutput::Num(find_interval("AAA", &directions, &locations) as _)
  }

  fn part_two(&self, input: &str) -> SolverOutput {
    let (directions, locations) = parse(input);
    let mut steps = 1;
    for interval in locations
      .keys()
      .filter(|name| name.as_bytes()[2] == b'A')
      .map(|ghost_location| find_interval(ghost_location, &directions, &locations))
    {
      steps = num::integer::lcm(steps, interval);
    }

    SolverOutput::Num(steps as _)
  }
}

fn find_interval(
  location: &str,
  directions: &[Direction],
  locations: &HashMap<&str, (&str, &str)>,
) -> usize {
  let mut location = location;
  let mut steps = 0;
  loop {
    if location.as_bytes()[2] == b'Z' {
      return steps;
    }

    let (left, right) = locations.get(location).unwrap();
    location = match directions[steps % directions.len()] {
      Direction::Left => left,
      Direction::Right => right,
    };

    steps += 1;
  }
}

#[derive(Debug)]
enum Direction {
  Left,
  Right,
}

fn parse<'a>(input: &'a str) -> (Vec<Direction>, HashMap<&'a str, (&'a str, &'a str)>) {
  let (directions, locations) = input.split_once("\n\n").unwrap();
  let directions = directions
    .chars()
    .map(|c| match c {
      'L' => Direction::Left,
      'R' => Direction::Right,
      _ => unreachable!(),
    })
    .collect();
  let locations = locations
    .lines()
    .map(|location| (&location[0..3], (&location[7..10], &location[12..15])))
    .collect();
  (directions, locations)
}
