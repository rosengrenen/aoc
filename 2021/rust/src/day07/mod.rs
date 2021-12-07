use aoc_util::{Solver, SolverOutput};

#[derive(Default)]
pub struct Day7;

impl Solver for Day7 {
	fn part_one(&self, input: &str) -> SolverOutput {
		let a = parse(input);
		let max = a.iter().max().unwrap();
		let mut cheapest_pos = i64::MAX;
		for pos in 0..*max {
			cheapest_pos = cheapest_pos.min(a.iter().fold(0, |prev, cur| prev + (pos - cur).abs()));
		}
		SolverOutput::Num(cheapest_pos)
	}

	fn part_two(&self, input: &str) -> SolverOutput {
		let a = parse(input);
		let max = a.iter().max().unwrap();
		let mut cheapest_pos = i64::MAX;
		for pos in 0..*max {
			cheapest_pos = cheapest_pos.min(a.iter().fold(0, |prev, cur| {
				let sum: i64 = (1..=(pos - cur).abs()).sum();
				prev + sum
			}));
		}
		SolverOutput::Num(cheapest_pos)
	}
}

fn parse(input: &str) -> Vec<i64> {
	input.split(",").map(|line| line.parse().unwrap()).collect()
}

// #[cfg(test)]
// mod benches {
// 	use super::*;
// 	use aoc_util::get_input;
// 	use test::{black_box, Bencher};

// 	#[bench]
// 	fn parse(bencher: &mut Bencher) {
// 		let input = get_input(2021, 6).unwrap();
// 		bencher.iter(|| parse_(black_box(&input)));
// 	}
// }
