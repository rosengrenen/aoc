use crate::lib::Solver;

pub struct Day18Solver;

impl Solver for Day18Solver {
	fn solve_part_one(&self, input: &str) -> i64 {
		input
			.lines()
			.fold(0, |prev, cur_line| prev + solve_recursive(cur_line))
	}

	fn solve_part_two(&self, input: &str) -> i64 {
		input.lines().fold(0, |prev, cur_line| {
			prev + solve_recursive_with_precedence(cur_line)
		})
	}
}

fn solve_recursive(expression: &str) -> i64 {
	let expression_bytes = expression.as_bytes();
	let mut result = 0;
	let mut raw_num = String::new();
	let mut operator = ' ';
	let mut i = 0;

	while i < expression_bytes.len() {
		let c = expression_bytes[i];
		if c.is_ascii_digit() {
			raw_num.push(c as char);
			if i == expression_bytes.len() - 1 {
				let num = raw_num.parse().unwrap();
				match operator {
					' ' => result = num,
					'+' => result += num,
					'*' => result *= num,
					_ => (),
				}
			}
		}

		if c == b'+' {
			operator = '+';
		}

		if c == b'*' {
			operator = '*';
		}

		if c == b' ' && !raw_num.is_empty() {
			let num = raw_num.parse().unwrap();
			raw_num.clear();
			match operator {
				' ' => result = num,
				'+' => result += num,
				'*' => result *= num,
				_ => (),
			}
		}

		if c == b'(' {
			let mut brackets = 0;
			for j in i + 1..expression.len() {
				let c = expression_bytes[j];
				if c == b'(' {
					brackets += 1;
				}
				if c == b')' {
					if brackets == 0 {
						match operator {
							' ' => result = solve_recursive(&expression[i + 1..j]),
							'+' => result += solve_recursive(&expression[i + 1..j]),
							'*' => result *= solve_recursive(&expression[i + 1..j]),
							_ => panic!(),
						}
						i = j + 1;
					} else {
						brackets -= 1;
					}
				}
			}
		}

		i += 1;
	}
	result
}

fn solve_recursive_with_precedence(expression: &str) -> i64 {
	let expression_bytes = expression.as_bytes();
	let mut raw_num = String::new();
	let mut i = 0;
	let mut expression_parsed = Vec::new();

	while i < expression_bytes.len() {
		let c = expression_bytes[i];
		if c.is_ascii_digit() {
			raw_num.push(c as char);
			if i == expression_bytes.len() - 1 {
				expression_parsed.push(raw_num.to_owned());
			}
		}

		if c == b'+' {
			expression_parsed.push("+".to_owned());
		}

		if c == b'*' {
			expression_parsed.push("*".to_owned());
		}

		if c == b' ' && !raw_num.is_empty() {
			expression_parsed.push(raw_num.to_owned());
			raw_num.clear();
		}

		if c == b'(' {
			let mut brackets = 0;
			for j in i + 1..expression.len() {
				let c = expression_bytes[j];
				if c == b'(' {
					brackets += 1;
				}
				if c == b')' {
					if brackets == 0 {
						expression_parsed.push(
							solve_recursive_with_precedence(&expression[i + 1..j]).to_string(),
						);
						i = j + 1;
					} else {
						brackets -= 1;
					}
				}
			}
		}

		i += 1;
	}

	let mut prod: Option<i64> = None;
	let mut result = 1;

	for i in 0..expression_parsed.len() {
		if prod.is_none() {
			prod = Some(expression_parsed[i].parse().unwrap());
			continue;
		}
		if expression_parsed[i] == "*" {
			result *= prod.unwrap();
			prod = None;
			continue;
		}

		if expression_parsed[i] != "+" {
			let a = prod.unwrap();
			let b = expression_parsed[i].parse::<i64>().unwrap();
			prod = Some(a + b);
		}
	}

	result * prod.unwrap()
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::lib::fetch_input;
	use test::Bencher;

	#[test]
	fn part_one_test_cases() {
		let solver = Day18Solver {};
		assert_eq!(solver.solve_part_one(&"1 + 2 * 3 + 4 * 5 + 6"), 71);
		assert_eq!(solver.solve_part_one(&"1 + (2 * 3) + (4 * (5 + 6))"), 51);
		assert_eq!(solver.solve_part_one(&"2 * 3 + (4 * 5)"), 26);
		assert_eq!(solver.solve_part_one(&"5 + (8 * 3 + 9 + 3 * 4 * 3)"), 437);
		assert_eq!(
			solver.solve_part_one(&"5 * 9 * (7 * 3 * 3 + 9 * 3 + (8 + 6 * 4))"),
			12240
		);
		assert_eq!(
			solver.solve_part_one(&"((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2"),
			13632
		);
	}

	#[test]
	fn part_two_test_cases() {
		let solver = Day18Solver {};
		assert_eq!(solver.solve_part_two(&"1 + 2 * 3 + 4 * 5 + 6"), 231);

		assert_eq!(
			solver.solve_part_two(&"1 + (2 * 3) + (4 * (5 + 6)) still"),
			51
		);
		assert_eq!(solver.solve_part_two(&"2 * 3 + (4 * 5)"), 46);
		assert_eq!(solver.solve_part_two(&"5 + (8 * 3 + 9 + 3 * 4 * 3)"), 1445);
		assert_eq!(
			solver.solve_part_two(&"5 * 9 * (7 * 3 * 3 + 9 * 3 + (8 + 6 * 4))"),
			669060
		);
		assert_eq!(
			solver.solve_part_two(&"((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2"),
			23340
		);
	}

	#[bench]
	fn bench_part_one(bencher: &mut Bencher) {
		let input = fetch_input(18);
		let solver = Day18Solver {};
		bencher.iter(|| solver.solve_part_one(&input));
	}

	#[bench]
	fn bench_part_two(bencher: &mut Bencher) {
		let input = fetch_input(18);
		let solver = Day18Solver {};
		bencher.iter(|| solver.solve_part_two(&input));
	}
}
