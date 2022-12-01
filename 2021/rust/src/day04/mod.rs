use aoc_util::{Solver, SolverOutput};

#[derive(Default)]
pub struct Day4;

impl Solver for Day4 {
  fn part_one(&self, input: &str) -> SolverOutput {
    let (numbers, mut boards) = parse_boards(input);
    for num in numbers {
      for board in boards.iter_mut() {
        board.mark(num);
        if board.has_bingo() {
          return SolverOutput::Num(num * board.sum_unmarked());
        }
      }
    }

    unreachable!();
  }

  fn part_two(&self, input: &str) -> SolverOutput {
    let (numbers, mut boards) = parse_boards(input);
    for num in numbers {
      if boards.len() <= 1 {
        boards[0].mark(num);
        if boards[0].has_bingo() {
          return SolverOutput::Num(num * boards[0].sum_unmarked());
        }
      } else {
        for board in boards.iter_mut() {
          board.mark(num);
        }

        boards = boards
          .into_iter()
          .filter(|board| !board.has_bingo())
          .collect();
      }
    }

    unreachable!();
  }
}

#[derive(Debug)]
struct Board {
  numbers: Vec<Vec<(i64, bool)>>,
}

impl Board {
  fn mark(&mut self, number: i64) {
    for i in 0..5 {
      for j in 0..5 {
        if self.numbers[i][j].0 == number {
          self.numbers[i][j].1 = true;
        }
      }
    }
  }

  fn has_bingo(&self) -> bool {
    self.check_rows() || self.check_cols()
  }

  fn check_rows(&self) -> bool {
    'outer: for i in 0..5 {
      for j in 0..5 {
        if !self.numbers[i][j].1 {
          continue 'outer;
        }
      }

      return true;
    }

    false
  }

  fn check_cols(&self) -> bool {
    'outer: for i in 0..5 {
      for j in 0..5 {
        if !self.numbers[j][i].1 {
          continue 'outer;
        }
      }

      return true;
    }

    false
  }

  fn sum_unmarked(&self) -> i64 {
    let mut sum = 0;
    for i in 0..5 {
      for j in 0..5 {
        if !self.numbers[i][j].1 {
          sum += self.numbers[i][j].0;
        }
      }
    }

    sum
  }
}

fn parse_boards(input: &str) -> (Vec<i64>, Vec<Board>) {
  let mut parts = input.split("\n\n");
  let numbers = parts
    .next()
    .unwrap()
    .split(',')
    .map(|num| num.parse().unwrap())
    .collect();

  let boards = parts
    .map(|part| Board {
      numbers: part
        .lines()
        .map(|line| {
          line
            .trim()
            .split_whitespace()
            .map(|num| (num.parse().unwrap(), false))
            .collect()
        })
        .collect(),
    })
    .collect();

  (numbers, boards)
}

#[cfg(test)]
mod benches {
  use super::*;
  use aoc_util::get_input;
  use test::{black_box, Bencher};

  #[bench]
  fn parse(bencher: &mut Bencher) {
    let input = get_input(2021, 4).unwrap();
    bencher.iter(|| parse_boards(black_box(&input)));
  }
}
