use aoc_util::{Solver, SolverOutput};

#[derive(Default)]
pub struct Day6;

impl Solver for Day6 {
  fn part_one(&self, input: &str) -> SolverOutput {
    SolverOutput::Num(
      parse(input)
        .into_iter()
        .map(|(time, distance)| find_highest(time, distance) - find_lowest(time, distance) + 1)
        .product(),
    )
  }

  fn part_two(&self, input: &str) -> SolverOutput {
    let (time, distance) = parse2(input);
    SolverOutput::Num(find_highest(time, distance) - find_lowest(time, distance) + 1)
  }
}

fn find_lowest(race_duration: i64, target_distance: i64) -> i64 {
  let mut low = 0;
  let mut high = race_duration;
  while low < high - 1 {
    let mid = (low + high) / 2;
    let distance = boat_distance(mid, race_duration);
    if distance > target_distance {
      high = mid;
    } else {
      low = mid;
    }
  }

  high
}

fn find_highest(race_duration: i64, target_distance: i64) -> i64 {
  let mut low = 0;
  let mut high = race_duration;
  while low < high - 1 {
    let mid = (low + high) / 2;
    let distance = boat_distance(mid, race_duration);
    if distance > target_distance {
      low = mid;
    } else {
      high = mid;
    }
  }

  low
}

fn boat_distance(time_charged: i64, race_duration: i64) -> i64 {
  time_charged * (race_duration - time_charged)
}

fn parse<'a>(input: &'a str) -> Vec<(i64, i64)> {
  let (times, distances) = input.split_once("\n").unwrap();
  let times = times
    .strip_prefix("Time:")
    .unwrap()
    .split_whitespace()
    .filter(|p| !p.is_empty())
    .filter_map(|time| time.parse().ok());
  let distances = distances
    .strip_prefix("Distance:")
    .unwrap()
    .split_whitespace()
    .filter(|p| !p.is_empty())
    .filter_map(|time| time.parse().ok());
  times.zip(distances).collect()
}

fn parse2<'a>(input: &'a str) -> (i64, i64) {
  let (time, distance) = input.split_once("\n").unwrap();
  let time = time
    .strip_prefix("Time:")
    .unwrap()
    .chars()
    .filter(|&c| c != ' ')
    .collect::<String>()
    .parse()
    .unwrap();
  let distance = distance
    .strip_prefix("Distance:")
    .unwrap()
    .chars()
    .filter(|&c| c != ' ')
    .collect::<String>()
    .parse()
    .unwrap();
  (time, distance)
}
