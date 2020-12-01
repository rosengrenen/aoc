use crate::lib::Solver;

pub struct Day1Solver;

impl Solver for Day1Solver {
  fn solve(&self, lines: Vec<String>, part_two: bool) -> String {
    let mut numbers: Vec<i32> = lines.iter().map(|line| line.parse().unwrap()).collect();
    numbers.sort();

    let mut max = 0;
    if !part_two {
      for first_number in numbers.iter() {
        for second_number in numbers.iter() {
          if first_number + second_number == 2020 {
            let product = first_number * second_number;
            if product > max {
              max = product;
            }
          }
        }
      }
    } else {
      for first_number in numbers.iter() {
        for second_number in numbers.iter() {
          for third_number in numbers.iter() {
            if first_number + second_number + third_number == 2020 {
              let product = first_number * second_number * third_number;
              if product > max {
                max = product;
              }
            }
          }
        }
      }
    }

    max.to_string()
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn part_one_test_cases() {
    let input: Vec<String> = vec![
      "1721".to_string(),
      "979".to_string(),
      "366".to_string(),
      "299".to_string(),
      "675".to_string(),
      "1456".to_string(),
    ];
    let solver: Day1Solver = Day1Solver {};
    assert_eq!(solver.solve(input, false), "514579");
  }

  #[test]
  fn part_two_test_cases() {
    let input: Vec<String> = vec![
      "1721".to_string(),
      "979".to_string(),
      "366".to_string(),
      "299".to_string(),
      "675".to_string(),
      "1456".to_string(),
    ];
    let solver: Day1Solver = Day1Solver {};
    assert_eq!(solver.solve(input, true), "241861950");
  }
}
