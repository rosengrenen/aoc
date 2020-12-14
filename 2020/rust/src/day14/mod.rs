use crate::lib::Solver;
use itertools::Itertools;
use std::collections::HashMap;

pub struct Day14Solver;

impl Solver for Day14Solver {
	fn solve_part_one(&self, input: &str) -> i64 {
		let instructions = parse(input);
		let mut mask = (0, 0);
		let mut mem: HashMap<i64, i64> = HashMap::new();
		for instruction in instructions.iter() {
			match instruction {
				Instruction::Mask(mask_bits, override_bits) => mask = (*mask_bits, *override_bits),
				Instruction::Mem(address, value) => {
					let result = (value & !mask.0) | (mask.0 & mask.1);
					mem.insert(*address, result);
				}
			}
		}
		mem.iter().fold(0, |p, c| p + c.1)
	}

	fn solve_part_two(&self, input: &str) -> i64 {
		let instructions = parse(input);
		let mut mask = (0, 0);
		let mut mem: HashMap<i64, i64> = HashMap::new();
		for instruction in instructions.iter() {
			match instruction {
				Instruction::Mask(mask_bits, override_bits) => mask = (*mask_bits, *override_bits),
				Instruction::Mem(address, value) => {
					// println!("orig {:b}, {}", address, address);
					// println!("mask {:b}, {:b}", mask.0, mask.1);
					// println!("yes {:b}, {:b}", (address & mask.0), (mask.0 & mask.1));
					let address = (address & mask.0) | (mask.0 & mask.1);

					let mut floating_bit_indices = Vec::new();
					for i in 0..36 {
						if (!mask.0) & 1 << i != 0 {
							floating_bit_indices.push(i);
						}
					}

					// let result = (value & !mask.0) | (mask.0 & mask.1);

					let p = create_yes(floating_bit_indices.len() as u32);

					for c in p {
						// println!("{:?}", c);
						let mut local_address = address;
						for (i, &bit) in c.iter().enumerate() {
							if bit == 1 {
								local_address |= 1 << floating_bit_indices[i];
							} else {
								local_address &= !(1 << floating_bit_indices[i]);
							}
						}
						// println!("changed {:b} to {:b}", address, local_address);
						mem.insert(local_address, *value);
						println!("wrinting {} to {}", value, local_address);
					}
				}
			}
		}
		mem.iter().fold(0, |p, c| p + c.1)
	}
}

enum Instruction {
	Mask(i64, i64),
	Mem(i64, i64),
}

fn parse(input: &str) -> Vec<Instruction> {
	input
		.lines()
		.map(|line| {
			if let Some(mask) = line.strip_prefix("mask = ") {
				let mask_bits = mask.replace("0", "1").replace("X", "0");
				let mask_override_bits = mask.replace("X", "0");
				Instruction::Mask(
					i64::from_str_radix(&mask_bits, 2).unwrap(),
					i64::from_str_radix(&mask_override_bits, 2).unwrap(),
				)
			} else {
				let mut split = line.split("] = ");
				let address = split
					.next()
					.unwrap()
					.strip_prefix("mem[")
					.unwrap()
					.parse()
					.unwrap();
				let value = split.next().unwrap().parse().unwrap();
				Instruction::Mem(address, value)
			}
		})
		.collect()
}

fn create_yes(n: u32) -> Vec<Vec<i64>> {
	let mut v = Vec::new();
	for i in 0..(2i64.pow(n)) {
		let mut c = Vec::new();
		for j in 0..n {
			if i & 1 << j != 0 {
				c.push(1);
			} else {
				c.push(0);
			}
		}
		v.push(c);
	}
	v
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::lib::fetch_input;
	use test::Bencher;

	#[test]
	fn part_one_test_cases() {
		let input = include_str!("input.test1.txt");
		let solver = Day14Solver {};
		assert_eq!(solver.solve_part_one(input), 165);
	}

	#[test]
	fn part_two_test_cases() {
		let input = include_str!("input.test2.txt");
		let solver = Day14Solver {};
		assert_eq!(solver.solve_part_two(input), 208);
	}

	#[bench]
	fn bench_parse(bencher: &mut Bencher) {
		let input = fetch_input(14);
		bencher.iter(|| parse(&input));
	}

	#[bench]
	fn bench_part_one(bencher: &mut Bencher) {
		let input = fetch_input(14);
		let solver = Day14Solver {};
		bencher.iter(|| solver.solve_part_one(&input));
	}

	#[bench]
	fn bench_part_two(bencher: &mut Bencher) {
		let input = fetch_input(14);
		let solver = Day14Solver {};
		bencher.iter(|| solver.solve_part_two(&input));
	}
}
