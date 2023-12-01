use aoc_util::{Solver, SolverOutput};

#[derive(Default)]
pub struct Day1;

impl Solver for Day1 {
  fn part_one(&self, input: &str) -> SolverOutput {
    SolverOutput::Num(
      parse_calibrations(input)
        .into_iter()
        .map(|(first, last)| first * 10 + last)
        .sum::<i64>(),
    )
  }

  fn part_two(&self, input: &str) -> SolverOutput {
    SolverOutput::Num(
      parse_calibrations_with_alpha(input)
        .into_iter()
        .map(|(first, last)| first * 10 + last)
        .sum::<i64>(),
    )
  }
}

fn parse_calibrations<'a>(input: &'a str) -> impl Iterator<Item = (i64, i64)> + 'a {
  input.lines().map(|line| {
    let digits = line
      .bytes()
      .filter(|b| b.is_ascii_digit())
      .map(|b| (b - b'0') as i64);
    (digits.clone().next().unwrap(), digits.last().unwrap())
  })
}

fn parse_calibrations_with_alpha<'a>(input: &'a str) -> impl Iterator<Item = (i64, i64)> + 'a {
  const DIGITS: [(&[u8], i64); 18] = [
    (b"1", 1),
    (b"2", 2),
    (b"3", 3),
    (b"4", 4),
    (b"5", 5),
    (b"6", 6),
    (b"7", 7),
    (b"8", 8),
    (b"9", 9),
    (b"one", 1),
    (b"two", 2),
    (b"three", 3),
    (b"four", 4),
    (b"five", 5),
    (b"six", 6),
    (b"seven", 7),
    (b"eight", 8),
    (b"nine", 9),
  ];
  input.lines().map(|line| {
    let mut first = None;
    let mut last = None;
    let line_bytes = line.as_bytes();
    for i in 0..line.len() {
      for (string, value) in DIGITS.iter() {
        if line_bytes[i..].starts_with(string) {
          if first.is_none() {
            first = Some(*value);
          } else {
            last = Some(*value);
          }
          break;
        }
      }
    }

    if last.is_none() {
      last = first;
    }
    (first.unwrap(), last.unwrap())
  })
}
