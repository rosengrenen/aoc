use crate::lib::Solver;
use hashbrown::HashMap;

pub struct Day18Solver;

impl Solver for Day18Solver {
	fn solve_part_one(&self, input: &str) -> i64 {
		let op_precedences = HashMap::new();
		input.lines().fold(0, |prev, cur_line| {
			let infix = parse_string_to_tokens(cur_line);
			let postfix = infix_to_postfix(&infix, &op_precedences);
			prev + eval_postfix(&postfix)
		})
	}

	fn solve_part_two(&self, input: &str) -> i64 {
		let mut op_precedences = HashMap::new();
		op_precedences.insert(b'+', 1);
		op_precedences.insert(b'*', 0);
		input.lines().fold(0, |prev, cur_line| {
			let infix = parse_string_to_tokens(cur_line);
			let postfix = infix_to_postfix(&infix, &op_precedences);
			prev + eval_postfix(&postfix)
		})
	}
}

#[derive(Clone, Copy, Debug, PartialEq)]
enum Token {
	Num(i64),
	Op(u8),
	OpenParens,
	CloseParens,
}

fn parse_string_to_tokens(input: &str) -> Vec<Token> {
	let bytes = input.as_bytes();
	let mut tokens = Vec::new();

	let mut i = 0;
	loop {
		if i >= input.len() {
			break;
		}

		match bytes[i] {
			b'(' => tokens.push(Token::CloseParens),
			b')' => tokens.push(Token::OpenParens),
			b'+' => tokens.push(Token::Op(b'+')),
			b'*' => tokens.push(Token::Op(b'*')),
			c if (48..=57).contains(&c) => {
				let mut j = i + 1;
				loop {
					if j >= input.len() {
						break;
					}

					if (48..=57).contains(&bytes[j]) {
						j += 1;
						continue;
					}

					break;
				}
				tokens.push(Token::Num(input[i..j].parse().unwrap()));
				i = j;
				continue;
			}
			_ => (),
		}

		i += 1;
	}

	tokens.reverse();
	tokens
}

fn infix_to_postfix(infix: &Vec<Token>, op_precedences: &HashMap<u8, i64>) -> Vec<Token> {
	let mut postfix = Vec::new();
	let mut op_stack = Vec::new();

	for &token in infix.iter() {
		match token {
			Token::OpenParens => op_stack.push(Token::OpenParens),
			Token::CloseParens => {
				while let Some(op) = op_stack.pop() {
					if op == Token::OpenParens {
						break;
					}

					postfix.push(op);
				}
			}
			Token::Op(op) => {
				if let Some(op_precedence) = op_precedences.get(&op) {
					while let Some(Token::Op(other_op)) = op_stack.last() {
						if let Some(other_op_precendence) = op_precedences.get(other_op) {
							if other_op_precendence > op_precedence {
								postfix.push(op_stack.pop().unwrap());
								// *other_op = op;
								continue;
							}
						}

						break;
					}
				}

				op_stack.push(Token::Op(op));
			}

			Token::Num(num) => postfix.push(Token::Num(num)),
		}
	}

	while let Some(op_token) = op_stack.pop() {
		if let Token::Op(op) = op_token {
			postfix.push(Token::Op(op));
		} else {
			panic!("Only operators should exist on the stack now, check for unmatched parentheses");
		}
	}

	postfix
}

fn eval_postfix(postfix: &Vec<Token>) -> i64 {
	let mut stack = Vec::new();
	for token in postfix.iter() {
		match *token {
			Token::Num(num) => stack.push(num),
			Token::Op(op) => {
				if let Some(a) = stack.pop() {
					if let Some(b) = stack.pop() {
						stack.push(match op {
							b'+' => a + b,
							b'*' => a * b,
							_ => panic!(format!("Unknown operator: {}", op as char)),
						});
						continue;
					}
				}
				panic!("There aren't two numbers on the stack that can be operated on");
			}
			_ => panic!("Invalid token at this stage"),
		}
	}

	*stack.last().unwrap()
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
	fn bench_parse_string_to_tokens(bencher: &mut Bencher) {
		let input = fetch_input(18);
		bencher.iter(|| parse_string_to_tokens(&input));
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
