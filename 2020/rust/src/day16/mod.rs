use std::collections::HashMap;

use aoc_util::{Solver, SolverOutput};

#[derive(Default)]
pub struct Day16;

impl Solver for Day16 {
  fn part_one(&self, input: &str) -> SolverOutput {
    let (ranges, values) = parse_simple(input);
    let mut ticket_error_rate = 0;
    for value in values.iter() {
      let mut invalid = true;
      for range in ranges.iter() {
        if (range.min..=range.max).contains(&value) {
          invalid = false;
          break;
        }
      }
      if invalid {
        ticket_error_rate += value;
      }
    }

    SolverOutput::Num(ticket_error_rate)
  }

  fn part_two(&self, input: &str) -> SolverOutput {
    let data = parse_advanced(input);
    let valid_tickets: Vec<_> = data
      .others_tickets
      .iter()
      .filter(|ticket| {
        for value in ticket.iter() {
          let mut invalid = true;
          for (_, first_range, second_range) in data.field_rules.iter() {
            if (first_range.min..=first_range.max).contains(&value)
              || (second_range.min..=second_range.max).contains(&value)
            {
              invalid = false;
              break;
            }
          }
          if invalid {
            return false;
          }
        }
        true
      })
      .collect();

    let mut index_to_names: HashMap<usize, Vec<&str>> = HashMap::new();
    let ticket_len = data.my_ticket.len();
    for (name, first_range, second_range) in data.field_rules.iter() {
      for i in 0..ticket_len {
        let mut matched = true;
        for ticket in valid_tickets.iter() {
          if !((first_range.min..=first_range.max).contains(&ticket[i])
            || (second_range.min..=second_range.max).contains(&ticket[i]))
          {
            matched = false;
            break;
          }
        }
        if matched {
          let entry = index_to_names.entry(i).or_insert_with(Vec::new);
          entry.push(name);
        }
      }
    }

    let mut name_to_index: HashMap<&str, usize> = HashMap::new();
    let mut index_to_names_vec: Vec<_> = index_to_names.iter().collect();
    index_to_names_vec
      .sort_by(|(_, left_vec), (_, right_vec)| left_vec.len().cmp(&right_vec.len()));
    for &(index, names) in index_to_names_vec.iter() {
      for name in names.iter() {
        if !name_to_index.contains_key(name) {
          name_to_index.insert(name, *index);
        }
      }
    }

    SolverOutput::Num(
      name_to_index
        .iter()
        .filter(|(key, _)| key.contains("departure"))
        .fold(1, |prev, (_, index)| prev * data.my_ticket[*index]),
    )
  }
}

fn parse_simple(input: &str) -> (Vec<Range>, Vec<i64>) {
  let mut split = input.split("\n\n");
  let mut ranges = Vec::new();
  for rule in split.next().unwrap().lines() {
    let (_name, rest) = rule.split_once(':').unwrap();
    let (first_range, second_range) = rest.trim().split_once("or").unwrap();
    let (first_min, first_max) = first_range.trim().split_once('-').unwrap();
    let (second_min, second_max) = second_range.trim().split_once('-').unwrap();
    ranges.push(Range {
      min: first_min.parse().unwrap(),
      max: first_max.parse().unwrap(),
    });
    ranges.push(Range {
      min: second_min.parse().unwrap(),
      max: second_max.parse().unwrap(),
    });
  }

  let mut others_tickets: Vec<i64> = Vec::new();
  for nums in split.nth(1).unwrap().lines().skip(1) {
    for num in nums.split(',') {
      others_tickets.push(num.parse().unwrap());
    }
  }

  (ranges, others_tickets)
}

struct Range {
  min: i64,
  max: i64,
}

struct Data<'a> {
  field_rules: Vec<(&'a str, Range, Range)>,
  my_ticket: Vec<i64>,
  others_tickets: Vec<Vec<i64>>,
}

fn parse_advanced(input: &str) -> Data {
  let mut split = input.split("\n\n");
  let mut field_rules = Vec::new();
  for rule in split.next().unwrap().lines() {
    let (name, rest) = rule.split_once(':').unwrap();
    let (first_range, second_range) = rest.trim().split_once("or").unwrap();
    let (first_min, first_max) = first_range.trim().split_once('-').unwrap();
    let (second_min, second_max) = second_range.trim().split_once('-').unwrap();
    field_rules.push((
      name,
      Range {
        min: first_min.parse().unwrap(),
        max: first_max.parse().unwrap(),
      },
      Range {
        min: second_min.parse().unwrap(),
        max: second_max.parse().unwrap(),
      },
    ))
  }

  let mut my_ticket: Vec<i64> = Vec::new();
  for num in split.next().unwrap().lines().nth(1).unwrap().split(',') {
    my_ticket.push(num.parse().unwrap());
  }

  let mut others_tickets = Vec::new();
  for nums in split.next().unwrap().lines().skip(1) {
    let mut others_ticket: Vec<i64> = Vec::new();
    for num in nums.split(',') {
      others_ticket.push(num.parse().unwrap());
    }
    others_tickets.push(others_ticket);
  }

  Data {
    field_rules,
    my_ticket,
    others_tickets,
  }
}
