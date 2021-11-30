use aoc_util::{Solver, SolverOutput};

#[derive(Default)]
pub struct Day8;

impl Solver for Day8 {
	fn part_one(&self, input: &str) -> SolverOutput {
		let input: Vec<i64> = input
			.as_bytes()
			.iter()
			.map(|&c| (c - b'0') as i64)
			.collect();
		let result = calculate_min_zeroes(&parse_raw(&input, 25, 6));
		SolverOutput::Num(result)
	}

	fn part_two(&self, input: &str) -> SolverOutput {
		let input: Vec<i64> = input
			.as_bytes()
			.iter()
			.map(|&c| (c - b'0') as i64)
			.collect();
		let result = print_layer(&combine_layers(&parse_raw(&input, 25, 6)), 25);
		SolverOutput::String(result)
	}
}

fn parse_raw(digits: &Vec<i64>, width: usize, height: usize) -> Vec<Vec<i64>> {
	let mut layers: Vec<Vec<i64>> = Vec::new();
	let mut digits = digits.clone();
	while digits.len() >= width * height {
		let tmp = digits.split_off(width * height);
		layers.push(digits.clone());
		digits = tmp;
	}
	layers
}

fn calculate_min_zeroes(layers: &Vec<Vec<i64>>) -> i64 {
	layers
		.iter()
		.fold(
			(std::i64::MAX, std::i64::MAX),
			|(number_of_zeroes, ones_by_twos), layer| {
				let mut zeroes = 0;
				let mut ones = 0;
				let mut twos = 0;
				for pixel in layer.iter() {
					match pixel {
						0 => zeroes += 1,
						1 => ones += 1,
						2 => twos += 1,
						_ => (),
					}
				}
				if zeroes < number_of_zeroes {
					(zeroes, ones * twos)
				} else {
					(number_of_zeroes, ones_by_twos)
				}
			},
		)
		.1
}

fn combine_layers(layers: &Vec<Vec<i64>>) -> Vec<i64> {
	let mut combined = layers.last().unwrap().clone();
	for (index, layer) in layers.iter().rev().enumerate() {
		if index == 0 {
			continue;
		}
		for (pixel_index, &pixel) in layer.iter().enumerate() {
			if pixel != 2 {
				combined[pixel_index] = pixel;
			}
		}
	}
	combined
}

fn print_layer(layer: &Vec<i64>, width: usize) -> String {
	let mut output = String::new();
	for (index, &pixel) in layer.iter().enumerate() {
		if index != 0 && index % width == 0 {
			output.push('\n');
		}
		match pixel {
			0 => output.push(' '),
			1 => output.push('O'),
			2 => output.push(' '),
			_ => panic!("Unknown pixel"),
		}
	}
	output
}
