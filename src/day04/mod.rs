use crate::lib::Solver;

pub struct Day4Solver;

impl Solver for Day4Solver {
  fn solve(&self, lines: Vec<String>, part_two: bool) -> String {
    let numbers: Vec<u32> = lines[0]
      .split('-')
      .map(|s| s.parse::<u32>().unwrap())
      .collect();
    let min = numbers[0];
    let max = numbers[1];
    let mut count = 0;
    for number in min..=max {
      if is_valid_code(number, part_two) {
        count += 1;
      }
    }
    count.to_string()
  }
}

fn is_valid_code(mut code: u32, part_two: bool) -> bool {
  let mut previous_digit = 10;
  let mut current_digit = code % 10;
  let mut digit_counts = [0; 10];

  loop {
    if code == 0 {
      break;
    }

    if current_digit > previous_digit {
      return false;
    }

    digit_counts[current_digit as usize] += 1;

    previous_digit = current_digit;
    code /= 10;
    current_digit = code % 10;
  }

  for &count in digit_counts.iter() {
    if part_two {
      if count == 2 {
        return true;
      }
    } else {
      if count >= 2 {
        return true;
      }
    }
  }

  false
}
