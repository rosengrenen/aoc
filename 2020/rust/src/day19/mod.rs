use aoc_util::{Solver, SolverOutput};
use hashbrown::HashMap;
use regex::Regex;

#[derive(Default)]
pub struct Day19;

impl Solver for Day19 {
  fn part_one(&self, input: &str) -> SolverOutput {
    let (rules, messages) = parse(input);
    let regex = Regex::new(&create_regex(&rules, 0)).unwrap();
    SolverOutput::Num(messages.iter().fold(
      0,
      |prev, cur| {
        if regex.is_match(cur) {
          prev + 1
        } else {
          prev
        }
      },
    ))
  }

  fn part_two(&self, input: &str) -> SolverOutput {
    let (mut rules, messages) = parse(input);
    rules
      .entry(8)
      .and_modify(|entry| *entry = ProductionRule::NonTerminal(vec![vec![42], vec![42, 8]]));
    rules.entry(11).and_modify(|entry| {
      *entry = ProductionRule::NonTerminal(vec![vec![42, 31], vec![42, 11, 31]])
    });

    let regex_42 = &create_regex(&rules, 42);
    let regex_31 = &create_regex(&rules, 31);

    let mut regexes = Vec::new();
    for n in 1..=5 {
      for r in 1..=5 {
        regexes
          .push(Regex::new(&format!("^{}{{{}}}{}{{{}}}$", regex_42, n + r, regex_31, r)).unwrap());
      }
    }
    SolverOutput::Num(messages.iter().fold(0, |prev, cur| {
      if regexes.iter().any(|regex| regex.is_match(cur)) {
        prev + 1
      } else {
        prev
      }
    }))
  }
}

fn create_regex(rules: &HashMap<i64, ProductionRule>, rule: i64) -> String {
  let mut regex_string = String::new();

  if rule == 0 {
    regex_string.push('^');
  }

  match rules.get(&rule).unwrap() {
    ProductionRule::Terminal(t) => regex_string.push(*t),
    ProductionRule::NonTerminal(ors) => {
      if ors.len() > 1 {
        regex_string.push('(');
      }
      for (i, or) in ors.iter().enumerate() {
        if i > 0 {
          regex_string.push('|');
        }

        for rule_num in or.iter() {
          if *rule_num == rule {
          } else {
            regex_string.push_str(&create_regex(rules, *rule_num));
          }
        }
      }
      if ors.len() > 1 {
        regex_string.push(')');
      }
    }
  }

  if rule == 0 {
    regex_string.push('$');
  }

  regex_string
}

#[derive(Clone, Debug)]
enum ProductionRule {
  Terminal(char),
  NonTerminal(Vec<Vec<i64>>),
}

fn parse(input: &str) -> (HashMap<i64, ProductionRule>, Vec<&str>) {
  let (rules_raw, messages_raw) = input.split_once("\n\n").unwrap();
  let messages: Vec<_> = messages_raw.lines().collect();
  let mut rules = HashMap::new();

  for rule_raw in rules_raw.lines() {
    let (num, rule) = rule_raw.split_once(": ").unwrap();
    let num: i64 = num.parse().unwrap();
    if rule == "\"a\"" || rule == "\"b\"" {
      rules.insert(num, ProductionRule::Terminal(rule.chars().nth(1).unwrap()));
      continue;
    }

    let mut r = Vec::new();
    for p in rule.split('|') {
      let mut m = Vec::new();
      for x in p.trim().split(' ') {
        m.push(x.parse::<i64>().unwrap());
      }
      r.push(m);
    }

    rules.insert(num, ProductionRule::NonTerminal(r));
  }

  (rules, messages)
}
