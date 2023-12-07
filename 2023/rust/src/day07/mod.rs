use std::cmp;

use aoc_util::{Solver, SolverOutput};

#[derive(Default)]
pub struct Day7;

impl Solver for Day7 {
  fn part_one(&self, input: &str) -> SolverOutput {
    let mut hands = parse(input);
    sort_hands(&mut hands);
    SolverOutput::Num(sum_bets(&hands))
  }

  fn part_two(&self, input: &str) -> SolverOutput {
    let mut hands = parse(input);
    downgrade_jokers(&mut hands);
    sort_hands(&mut hands);
    SolverOutput::Num(sum_bets(&hands))
  }
}

fn sort_hands(hands: &mut [Hand]) {
  hands.sort_unstable_by(|(left_cards, _), (right_cards, _)| {
    match hand_rank(left_cards).cmp(&hand_rank(right_cards)) {
      cmp::Ordering::Equal => left_cards.cmp(&right_cards),
      not_eq => not_eq,
    }
  });
}

fn sum_bets(hands: &[Hand]) -> i64 {
  hands
    .into_iter()
    .enumerate()
    .map(|(i, (_, bet))| bet * (i as i64 + 1))
    .sum()
}

fn downgrade_jokers(hands: &mut [Hand]) {
  hands.iter_mut().for_each(|(cards, _)| {
    cards.iter_mut().for_each(|card| {
      if *card == 11 {
        *card = 0;
      }
    })
  });
}

fn hand_rank(cards: &[i64]) -> i64 {
  let mut jokers = 0;
  let mut card_counts = [0; 13];
  for &card in cards {
    if card == 0 {
      jokers += 1;
    } else {
      card_counts[card as usize - 2] += 1;
    }
  }

  card_counts.sort_by_key(|k| -k);

  let most = card_counts[0];
  let second_most = card_counts[1];

  if most + jokers == 5 {
    return 6;
  }

  if most + jokers == 4 {
    return 5;
  }

  if most + second_most + jokers == 5 {
    return 4;
  }

  if most + jokers == 3 {
    return 3;
  }

  if most + second_most + jokers == 4 {
    return 2;
  }

  if most + jokers == 2 {
    return 1;
  }

  0
}

type Hand = (Vec<i64>, i64);

fn parse<'a>(input: &'a str) -> Vec<Hand> {
  input
    .lines()
    .map(|line| {
      let (cards, bet) = line.split_once(" ").unwrap();
      let cards = cards
        .chars()
        .map(|c| match c {
          '2' => 2,
          '3' => 3,
          '4' => 4,
          '5' => 5,
          '6' => 6,
          '7' => 7,
          '8' => 8,
          '9' => 9,
          'T' => 10,
          'J' => 11,
          'Q' => 12,
          'K' => 13,
          'A' => 14,
          _ => unreachable!(),
        })
        .collect();
      let bet = bet.parse().unwrap();
      (cards, bet)
    })
    .collect::<Vec<_>>()
}
