use std::collections::HashMap;

use aoc_util::{Solver, SolverOutput};

#[derive(Default)]
pub struct Day15;

impl Solver for Day15 {
  fn part_one(&self, input: &str) -> SolverOutput {
    let sensors = parse_sensors_and_beacons(input);
    let y = 2000000;
    let ranges = sensors
      .iter()
      .filter_map(|(sensor, beacon)| {
        let radius = sensor.manhattan_distance(&beacon);
        line_pos_intersection(*sensor, radius, y)
      })
      .collect::<Vec<_>>();
    let ranges = combine_ranges(ranges);
    SolverOutput::Num(
      ranges
        .iter()
        .fold(0, |prev, (start, end)| prev + (end - start)),
    )
  }

  fn part_two(&self, input: &str) -> SolverOutput {
    let sensors = parse_sensors_and_beacons(input);
    let min = 0;
    let max = 4000000;
    let distress_beacon = (min..=max)
      .find_map(|y| {
        let ranges = sensors
          .iter()
          .filter_map(|(sensor, beacon)| {
            let radius = sensor.manhattan_distance(&beacon);
            line_pos_intersection(*sensor, radius, y)
          })
          .filter(|(start, end)| *end >= min && *start <= max)
          .collect::<Vec<_>>();
        let ranges = combine_ranges(ranges);
        let (start, end) = ranges[0];
        if start > min || end < max {
          Some(Pos { x: end + 1, y })
        } else {
          None
        }
      })
      .unwrap();
    SolverOutput::Num(distress_beacon.x * 4000000 + distress_beacon.y)
  }
}

type Range = (i64, i64);

fn combine_ranges(mut ranges: Vec<Range>) -> Vec<Range> {
  ranges.sort_unstable();
  let mut min = ranges[0].0;
  let mut max = min;
  let mut combined_ranges = vec![];
  for (start, end) in ranges {
    if end < max {
      continue;
    }

    if start > max + 1 {
      combined_ranges.push((min, max));
      min = start;
    }

    max = end;
  }

  combined_ranges.push((min, max));

  combined_ranges
}

fn line_pos_intersection(center: Pos, radius: i64, y: i64) -> Option<(i64, i64)> {
  let dy = (center.y - y).abs();
  let dx = radius - dy;
  if dy > radius {
    return None;
  }

  Some((center.x - dx, center.x + dx))
}

fn parse_sensors_and_beacons(input: &str) -> HashMap<Pos, Pos> {
  input
    .lines()
    .map(|line| {
      let (sensor_x, rest) = line
        .strip_prefix("Sensor at x=")
        .unwrap()
        .split_once(", y=")
        .unwrap();
      let (sensor_y, rest) = rest.split_once(": closest beacon is at x=").unwrap();
      let (beacon_x, beacon_y) = rest.split_once(", y=").unwrap();
      let sensor = Pos {
        x: sensor_x.parse().unwrap(),
        y: sensor_y.parse().unwrap(),
      };
      let beacon = Pos {
        x: beacon_x.parse().unwrap(),
        y: beacon_y.parse().unwrap(),
      };
      (sensor, beacon)
    })
    .collect()
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
struct Pos {
  x: i64,
  y: i64,
}

impl Pos {
  fn manhattan_distance(&self, other: &Self) -> i64 {
    (self.x - other.x).abs() + (self.y - other.y).abs()
  }
}
