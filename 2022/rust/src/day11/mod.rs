use aoc_util::{Solver, SolverOutput};

#[derive(Default)]
pub struct Day11;

impl Solver for Day11 {
  fn part_one(&self, input: &str) -> SolverOutput {
    let mut monkeys = parse_monkeys(input);
    let mut monkey_business = vec![0; monkeys.len()];
    for _ in 0..20 {
      for i in 0..monkeys.len() {
        let items = monkeys[i].items.clone();
        monkeys[i].items.clear();
        for &item in items.iter() {
          monkey_business[i] += 1;
          let (left_operand, operator, right_operand) = monkeys[i].op;
          let (test_val, to_true, to_false) = monkeys[i].test;
          let left_val = match left_operand {
            Operand::Old => item,
            Operand::Number(n) => n,
          };
          let right_val = match right_operand {
            Operand::Old => item,
            Operand::Number(n) => n,
          };
          let mut new_item = match operator {
            Operator::Add => left_val + right_val,
            Operator::Mul => left_val * right_val,
          };
          new_item /= 3;
          if new_item % test_val == 0 {
            monkeys[to_true].items.push(new_item);
          } else {
            monkeys[to_false].items.push(new_item);
          }
        }
      }
    }

    monkey_business.sort_unstable_by(|a, b| b.cmp(&a));

    SolverOutput::Num(monkey_business[0] * monkey_business[1])
  }

  fn part_two(&self, input: &str) -> SolverOutput {
    let mut monkeys = parse_monkeys(input);
    let mut monkey_business = vec![0; monkeys.len()];
    let modulli = monkeys
      .iter()
      .map(|monkey| monkey.test.0)
      .collect::<Vec<_>>();
    for _ in 0..10000 {
      for i in 0..monkeys.len() {
        let items = monkeys[i].items.clone();
        monkeys[i].items.clear();
        for &item in items.iter() {
          monkey_business[i] += 1;
          let (left_operand, operator, right_operand) = monkeys[i].op;
          let (test_val, to_true, to_false) = monkeys[i].test;
          let left_val = match left_operand {
            Operand::Old => item,
            Operand::Number(n) => n,
          };
          let right_val = match right_operand {
            Operand::Old => item,
            Operand::Number(n) => n,
          };
          let mut new_item = match operator {
            Operator::Add => left_val + right_val,
            Operator::Mul => left_val * right_val,
          };
          let residues = modulli
            .iter()
            .map(|&base| new_item % base)
            .collect::<Vec<_>>();
          new_item = chinese_remainder_theorem(&residues, &modulli);
          if new_item % test_val == 0 {
            monkeys[to_true].items.push(new_item);
          } else {
            monkeys[to_false].items.push(new_item);
          }
        }
      }
    }

    monkey_business.sort_unstable_by(|a, b| b.cmp(&a));

    SolverOutput::Num(monkey_business[0] * monkey_business[1])
  }
}

fn parse_monkeys(input: &str) -> Vec<Monkey> {
  input
    .split("\n\n")
    .map(|monkey| {
      let lines = monkey.lines().skip(1).collect::<Vec<_>>();
      let (_, items) = lines[0].split_once(":").unwrap();
      let items = items
        .trim()
        .split(", ")
        .map(|item| item.parse().unwrap())
        .collect();
      let op_parts = lines[1]
        .trim()
        .strip_prefix("Operation: new = ")
        .unwrap()
        .split(" ")
        .collect::<Vec<_>>();
      let parse_operand = |s: &str| match s {
        "old" => Operand::Old,
        s => Operand::Number(s.parse().unwrap()),
      };
      let l_operand = parse_operand(op_parts[0]);
      let operator = match op_parts[1] {
        "+" => Operator::Add,
        "*" => Operator::Mul,
        _ => panic!(),
      };
      let r_operand = parse_operand(op_parts[2]);

      let test_num = lines[2]
        .trim()
        .strip_prefix("Test: divisible by ")
        .unwrap()
        .parse()
        .unwrap();
      let to_true = lines[3]
        .trim()
        .strip_prefix("If true: throw to monkey ")
        .unwrap()
        .parse()
        .unwrap();
      let to_false = lines[4]
        .trim()
        .strip_prefix("If false: throw to monkey ")
        .unwrap()
        .parse()
        .unwrap();

      Monkey {
        items,
        op: (l_operand, operator, r_operand),
        test: (test_num, to_true, to_false),
      }
    })
    .collect()
}

#[derive(Clone, Debug)]
struct Monkey {
  items: Vec<i64>,
  op: (Operand, Operator, Operand),
  test: (i64, usize, usize),
}

#[derive(Clone, Copy, Debug)]
enum Operator {
  Add,
  Mul,
}

#[derive(Clone, Copy, Debug)]
enum Operand {
  Old,
  Number(i64),
}

// Helper from https://github.com/TheAlgorithms/Rust
fn update_step(a: &mut i64, old_a: &mut i64, quotient: i64) {
  let temp = *a;
  *a = *old_a - quotient * temp;
  *old_a = temp;
}

pub fn extended_euclidean_algorithm(a: i64, b: i64) -> (i64, i64, i64) {
  let (mut old_r, mut rem) = (a, b);
  let (mut old_s, mut coeff_s) = (1, 0);
  let (mut old_t, mut coeff_t) = (0, 1);

  while rem != 0 {
    let quotient = old_r / rem;

    update_step(&mut rem, &mut old_r, quotient);
    update_step(&mut coeff_s, &mut old_s, quotient);
    update_step(&mut coeff_t, &mut old_t, quotient);
  }

  (old_r, old_s, old_t)
}

fn mod_inv(x: i64, n: i64) -> i64 {
  let (_, x, _) = extended_euclidean_algorithm(x, n);
  (x % n + n) % n
}

pub fn chinese_remainder_theorem(residues: &[i64], modulli: &[i64]) -> i64 {
  let prod = modulli.iter().product::<i64>();

  let mut sum = 0;

  for (&residue, &modulus) in residues.iter().zip(modulli) {
    let p = prod / modulus;
    sum += residue * mod_inv(p, modulus) * p
  }
  sum % prod
}
