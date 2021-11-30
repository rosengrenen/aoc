use aoc_util::{Solver, SolverOutput};
use hashbrown::HashMap;

#[derive(Default)]
pub struct Day18;

impl Solver for Day18 {
	fn part_one(&self, input: &str) -> SolverOutput {
		let op_precedences = HashMap::new();
		SolverOutput::Num(input.lines().fold(0, |prev, cur_line| {
			let infix = parse_string_to_tokens(cur_line);
			let postfix = infix_to_postfix(&infix, &op_precedences);
			prev + eval_postfix(&postfix)
		}))
	}

	fn part_two(&self, input: &str) -> SolverOutput {
		let mut op_precedences = HashMap::new();
		op_precedences.insert(b'+', 1);
		op_precedences.insert(b'*', 0);
		SolverOutput::Num(input.lines().fold(0, |prev, cur_line| {
			let infix = parse_string_to_tokens(cur_line);
			let postfix = infix_to_postfix(&infix, &op_precedences);
			prev + eval_postfix(&postfix)
		}))
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

fn infix_to_postfix(infix: &[Token], op_precedences: &HashMap<u8, i64>) -> Vec<Token> {
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

fn eval_postfix(postfix: &[Token]) -> i64 {
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
							_ => panic!("Unknown operator: {}", op as char),
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
