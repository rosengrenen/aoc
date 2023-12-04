use std::collections::{HashMap, HashSet};

use aoc_util::{Solver, SolverOutput};

#[derive(Default)]
pub struct Day4;

impl Solver for Day4 {
  fn part_one(&self, input: &str) -> SolverOutput {
    SolverOutput::Num(
      parse_schematic(input)
        .map(|(winning, current)| {
          let matches = current.into_iter().filter(|c| winning.contains(c)).count();
          if matches == 0 {
            0
          } else {
            2i64.pow(matches as u32 - 1)
          }
        })
        .sum(),
    )
  }

  fn part_two(&self, input: &str) -> SolverOutput {
    let cards = parse_schematic(input).collect::<Vec<_>>();
    let mut copies = Vec::new();
    let mut cache = HashMap::new();
    let mut sum = 0;
    for i in (0..cards.len()).rev() {
      copies.push(i);
    }

    while let Some(copy_index) = copies.pop() {
      sum += 1;
      let (winning, current) = &cards[copy_index];
      let matches = *cache
        .entry(copy_index)
        .or_insert_with(|| current.into_iter().filter(|c| winning.contains(c)).count());
      for i in (copy_index + 1..=copy_index + matches).rev() {
        copies.push(i);
      }
    }
    SolverOutput::Num(sum)
  }
}

fn parse_schematic<'a>(input: &'a str) -> impl Iterator<Item = (HashSet<i64>, Vec<i64>)> + 'a {
  input.lines().map(|line| {
    let (_, line) = line.split_once(":").unwrap();
    let line = line.trim();
    let (winning, current) = line.split_once("|").unwrap();
    let winning = winning
      .split(" ")
      .filter(|n| !n.is_empty())
      .map(|n| n.parse().unwrap())
      .collect::<HashSet<i64>>();
    let current = current
      .split(" ")
      .filter(|n| !n.is_empty())
      .map(|n| n.parse().unwrap())
      .collect::<Vec<i64>>();
    (winning, current)
  })
}
