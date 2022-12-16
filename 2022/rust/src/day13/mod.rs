use std::{cmp::Ordering, fmt::Display};

use aoc_util::{Solver, SolverOutput};

#[derive(Default)]
pub struct Day13;

impl Solver for Day13 {
  fn part_one(&self, input: &str) -> SolverOutput {
    SolverOutput::Num(
      parse_packet_pairs(input)
        .iter()
        .enumerate()
        .filter(|(_, (packet0, packet1))| packet0.cmp(&packet1) != Ordering::Greater)
        .map(|(i, _)| 1 + i as i64)
        .sum(),
    )
  }

  fn part_two(&self, input: &str) -> SolverOutput {
    let divider_packet0 = Val::List(vec![Val::List(vec![Val::Num(2)])]);
    let divider_packet1 = Val::List(vec![Val::List(vec![Val::Num(6)])]);
    let mut packets = parse_packets(input);
    packets.push(divider_packet0.clone());
    packets.push(divider_packet1.clone());
    packets.sort_unstable();
    let divider_packet0_index = packets.binary_search(&divider_packet0).unwrap() + 1;
    let divider_packet1_index = packets.binary_search(&divider_packet1).unwrap() + 1;
    SolverOutput::Num((divider_packet0_index * divider_packet1_index) as i64)
  }
}

fn parse_packet_pairs(input: &str) -> Vec<(Val, Val)> {
  input
    .split("\n\n")
    .map(|group| {
      let (packet0_raw, packet1_raw) = group.split_once("\n").unwrap();
      let packet0 = parse_packet(&packet0_raw[1..packet0_raw.len() - 1]).0;
      let packet1 = parse_packet(&packet1_raw[1..packet1_raw.len() - 1]).0;
      (packet0, packet1)
    })
    .collect()
}

fn parse_packets(input: &str) -> Vec<Val> {
  input
    .lines()
    .filter(|line| !line.is_empty())
    .map(|line| parse_packet(&line[1..line.len() - 1]).0)
    .collect()
}

fn parse_packet(input: &str) -> (Val, usize) {
  let bytes = input.as_bytes();
  let mut start = 0;
  let mut vals = vec![];
  let mut i = 0;
  while i < input.len() {
    match bytes[i] {
      b'[' => {
        let (val, di) = parse_packet(&input[i + 1..]);
        i += di;
        start = i + 1;
        vals.push(val);
      }
      b']' => {
        if start != i {
          vals.push(Val::Num(input[start..i].parse().unwrap()));
        }

        return (Val::List(vals), i + 1);
      }
      b',' => {
        if start != i {
          vals.push(Val::Num(input[start..i].parse().unwrap()));
        }

        start = i + 1;
      }
      _ => (),
    }

    i += 1;
  }

  if start != i {
    vals.push(Val::Num(input[start..i].parse().unwrap()));
  }

  (Val::List(vals), 0)
}

#[derive(Clone, Debug, Eq, PartialEq)]
enum Val {
  Num(i64),
  List(Vec<Val>),
}

impl PartialOrd for Val {
  fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
    Some(self.cmp(other))
  }
}

impl Ord for Val {
  fn cmp(&self, other: &Self) -> Ordering {
    match (self, other) {
      (Val::Num(num0), Val::Num(num1)) => num0.cmp(&num1),
      (Val::List(list0), Val::List(list1)) => {
        let mut list0_iter = list0.iter();
        let mut list1_iter = list1.iter();
        loop {
          match (list0_iter.next(), list1_iter.next()) {
            (Some(val0), Some(val1)) => match val0.cmp(val1) {
              Ordering::Equal => (),
              o => return o,
            },
            (Some(_), None) => return Ordering::Greater,

            (None, Some(_)) => return Ordering::Less,
            (None, None) => return Ordering::Equal,
          }
        }
      }
      (num @ Val::Num(_), list @ Val::List(_)) => Val::List(vec![num.clone()]).cmp(list),
      (list @ Val::List(_), num @ Val::Num(_)) => list.cmp(&Val::List(vec![num.clone()])),
    }
  }
}

impl Display for Val {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    match self {
      Val::Num(n) => write!(f, "{}", n),
      Val::List(list) => {
        write!(f, "[")?;
        let mut first = true;
        for item in list.iter() {
          if first {
            first = false;
            write!(f, "{}", item)?;
          } else {
            write!(f, ",{}", item)?;
          }
        }

        write!(f, "]")
      }
    }
  }
}
