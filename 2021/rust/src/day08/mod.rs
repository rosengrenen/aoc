use std::collections::HashSet;

use aoc_util::{Solver, SolverOutput};

#[derive(Default)]
pub struct Day8;

impl Solver for Day8 {
  fn part_one(&self, input: &str) -> SolverOutput {
    let mut a = parse_(input);

    SolverOutput::Num(a.iter().fold(0, |prev, (_, a)| {
      prev
        + a.iter().fold(0, |prev, a| match a.len() {
          2 | 3 | 4 | 7 => prev + 1,
          _ => prev,
        })
    }))
  }

  fn part_two(&self, input: &str) -> SolverOutput {
    let a = parse_(input);

    // println!("{:?}", a);
    // let mut s = [None; 8];

    let (segments, digits) = a[0].clone();
    let k = lmao(segments, &digits);
    println!("{:?}", k);
    // segments.sort_by(|left, right| left.len().cmp(&right.len()));
    // println!("{:?}", segments);
    // loop {
    // 	if segments[0].len() == 2 {
    // 		s[2] = Some(segments[0].as_bytes()[0]);
    // 		s[6] = Some(segments[0].as_bytes()[1]);
    // 		segments.remove(0);
    // 	}

    // 	if segments[0].len() == 3 {
    // 		let b = segments[0]
    // 			.as_bytes()
    // 			.iter()
    // 			.filter(|&b| *b != s[2].unwrap() && *b != s[6].unwrap())
    // 			.next()
    // 			.unwrap();
    // 		s[0] = Some(*b);
    // 		segments.remove(0);
    // 		break;
    // 	}
    // }

    // println!("{:?}", s);
    SolverOutput::Num(
      a.into_iter()
        .fold(0, |prev, (segments, digits)| prev + lmao(segments, &digits)),
    )
  }
}

fn lmao(segments: Vec<&str>, digits: &Vec<&str>) -> i64 {
  let mut segments1 = vec![Vec::new(); 7];
  for s in segments {
    let s: HashSet<_> = s.bytes().map(|b| (b - b'a') as usize).collect();
    segments1[s.len() - 1].push(s);
  }

  // 7 - 1
  let s0 = *segments1[2][0].difference(&segments1[1][0]).next().unwrap();

  let mut counts235 = [0; 7];
  for segment in &segments1[4] {
    for s in segment {
      counts235[*s as usize] += 1;
    }
  }

  let ns235 = |n: usize| -> HashSet<usize> {
    counts235
      .iter()
      .enumerate()
      .filter(|(_, c)| **c == n)
      .map(|(i, _)| i)
      .collect()
  };

  let singles235 = ns235(1);
  let doubles235 = ns235(2);
  let triples235 = ns235(3);

  // 4 - singles in 2,3,5
  let s1 = *segments1[3][0].intersection(&singles235).next().unwrap();

  // 4 - 1 - s1
  let mut s3: HashSet<_> = segments1[3][0]
    .difference(&segments1[1][0])
    .cloned()
    .collect();
  s3.remove(&s1);
  let s3 = s3.into_iter().next().unwrap();

  // singles in 2,3,5 - s1
  let mut s4 = singles235.clone();
  s4.remove(&s1);
  let s4 = s4.into_iter().next().unwrap();

  // triples in 2,3,5 - s0 - s3
  let mut s6 = triples235.clone();
  s6.remove(&s0);
  s6.remove(&s3);
  let s6 = s6.into_iter().next().unwrap();

  let mut counts690 = [0; 7];
  for segment in &segments1[5] {
    for s in segment {
      counts690[*s as usize] += 1;
    }
  }

  let ns690 = |n: usize| -> HashSet<usize> {
    counts690
      .iter()
      .enumerate()
      .filter(|(_, c)| **c == n)
      .map(|(i, _)| i)
      .collect()
  };

  let doubles690 = ns690(2);
  let triples690 = ns690(3);

  // doubles in 6,9,0 - s3 - s4
  let mut s2 = doubles690.clone();
  s2.remove(&s3);
  s2.remove(&s4);
  let s2 = s2.into_iter().next().unwrap();

  // 1 - s2
  let mut s5 = segments1[1][0].clone();
  s5.remove(&s2);
  let s5 = s5.into_iter().next().unwrap();

  let segments = [s0, s1, s2, s3, s4, s5, s6];

  let digits: Vec<HashSet<_>> = digits
    .iter()
    .map(|d| {
      d.as_bytes()
        .iter()
        .map(|d| (d - b'a') as usize)
        .collect::<Vec<_>>()
    })
    .map(|digits| {
      digits
        .iter()
        .map(|digit| {
          segments
            .iter()
            .enumerate()
            .find(|(i, s)| *s == digit)
            .unwrap()
            .0
        })
        .collect()
    })
    .collect();

  let digit_segments: Vec<HashSet<usize>> = vec![
    [0usize, 1, 2, 4, 5, 6].into_iter().collect(),
    [2usize, 5].into_iter().collect(),
    [0usize, 2, 3, 4, 6].into_iter().collect(),
    [0usize, 2, 3, 5, 6].into_iter().collect(),
    [1usize, 2, 3, 5].into_iter().collect(),
    [0usize, 1, 3, 5, 6].into_iter().collect(),
    [0usize, 1, 3, 4, 5, 6].into_iter().collect(),
    [0usize, 2, 5].into_iter().collect(),
    [0usize, 1, 2, 3, 4, 5, 6].into_iter().collect(),
    [0usize, 1, 2, 3, 5, 6].into_iter().collect(),
  ];

  let is_digit = |number: usize, input: &HashSet<usize>| -> bool {
    if input.len() != digit_segments[number].len() {
      return false;
    }

    &digit_segments[number] == input
  };
  let number = digits
    .iter()
    .map(|digit| {
      for num in 0..10 {
        if is_digit(num, digit) {
          return num;
        }
      }

      unreachable!();
    })
    .fold(0, |prev, cur| prev * 10 + cur);

  number as i64
}

fn parse_(input: &str) -> Vec<(Vec<&str>, Vec<&str>)> {
  input
    .lines()
    .map(|line| {
      let (segments, digits) = line.split_once(" | ").unwrap();
      (
        segments.split_whitespace().collect(),
        digits.split_whitespace().collect(),
      )
    })
    .collect()
}

fn wait_input() {
  let mut s = String::new();
  std::io::stdin().read_line(&mut s).unwrap();
}

#[cfg(test)]
mod benches {
  use super::*;
  use aoc_util::get_input;
  use test::{black_box, Bencher};

  #[bench]
  fn parse(bencher: &mut Bencher) {
    let input = get_input(2021, 8).unwrap();
    bencher.iter(|| parse_(black_box(&input)));
  }
}
