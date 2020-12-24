use hashbrown::HashSet;

use crate::lib::Solver;

pub struct Day24Solver;

impl Solver for Day24Solver {
	fn solve_part_one(&self, input: &str) -> i64 {
		let directions_list = parse(input);
		let mut tiles = HashSet::new();
		for directions in directions_list.iter() {
			let mut pos = (0, 0);
			for &direction in directions.iter() {
				match direction {
					Direction::E => pos = (pos.0 + 1, pos.1),
					Direction::NE => {
						if pos.1 % 2 == 0 {
							pos = (pos.0 + 1, pos.1 + 1)
						} else {
							pos = (pos.0, pos.1 + 1)
						}
					}
					Direction::NW => {
						if pos.1 % 2 == 0 {
							pos = (pos.0, pos.1 + 1)
						} else {
							pos = (pos.0 - 1, pos.1 + 1)
						}
					}
					Direction::SE => {
						if pos.1 % 2 == 0 {
							pos = (pos.0 + 1, pos.1 - 1)
						} else {
							pos = (pos.0, pos.1 - 1)
						}
					}
					Direction::SW => {
						if pos.1 % 2 == 0 {
							pos = (pos.0, pos.1 - 1)
						} else {
							pos = (pos.0 - 1, pos.1 - 1)
						}
					}
					Direction::W => pos = (pos.0 - 1, pos.1),
				}
			}

			if tiles.contains(&pos) {
				tiles.remove(&pos);
			} else {
				tiles.insert(pos);
			}
		}

		tiles.len() as i64
	}

	fn solve_part_two(&self, input: &str) -> i64 {
		let directions_list = parse(input);
		let mut tiles = HashSet::new();
		for directions in directions_list.iter() {
			let mut pos = (0, 0);
			for &direction in directions.iter() {
				match direction {
					Direction::E => pos = (pos.0 + 1, pos.1),
					Direction::NE => {
						if pos.1 % 2 == 0 {
							pos = (pos.0 + 1, pos.1 + 1)
						} else {
							pos = (pos.0, pos.1 + 1)
						}
					}
					Direction::NW => {
						if pos.1 % 2 == 0 {
							pos = (pos.0, pos.1 + 1)
						} else {
							pos = (pos.0 - 1, pos.1 + 1)
						}
					}
					Direction::SE => {
						if pos.1 % 2 == 0 {
							pos = (pos.0 + 1, pos.1 - 1)
						} else {
							pos = (pos.0, pos.1 - 1)
						}
					}
					Direction::SW => {
						if pos.1 % 2 == 0 {
							pos = (pos.0, pos.1 - 1)
						} else {
							pos = (pos.0 - 1, pos.1 - 1)
						}
					}
					Direction::W => pos = (pos.0 - 1, pos.1),
				}
			}

			if tiles.contains(&pos) {
				tiles.remove(&pos);
			} else {
				tiles.insert(pos);
			}
		}

		let even_neighbours = [(0, 1), (1, 1), (1, 0), (1, -1), (0, -1), (-1, 0)];
		let odd_neighbours = [(-1, 1), (0, 1), (1, 0), (0, -1), (-1, -1), (-1, 0)];

		// println!("{}", tiles.len());
		// print_tiles(&tiles);
		for _ in 0..100 {
			// println!("{:?}", tiles);
			let mut new_tiles = HashSet::new();
			for &(x, y) in tiles.iter() {
				let neighbours = if y % 2 == 0 {
					even_neighbours
				} else {
					odd_neighbours
				};
				for &(dx, dy) in neighbours.iter() {
					let ax = x + dx;
					let ay = y + dy;
					let count = count_neighbours(&tiles, (ax, ay));
					// println!("{},{} has {} neighbours", ax, ay, count);

					if tiles.contains(&(ax, ay)) {
						if count == 1 || count == 2 {
							new_tiles.insert((ax, ay));
						}
					} else {
						if count == 2 {
							new_tiles.insert((ax, ay));
						}
					}
				}
			}
			tiles = new_tiles;

			println!("{}", tiles.len());
			// print_tiles(&tiles);
		}

		tiles.len() as i64
	}
}

fn count_neighbours(tiles: &HashSet<(i64, i64)>, pos: (i64, i64)) -> i64 {
	let (x, y) = pos;
	let even_neighbours = [(0, 1), (1, 1), (1, 0), (1, -1), (0, -1), (-1, 0)];
	let odd_neighbours = [(-1, 1), (0, 1), (1, 0), (0, -1), (-1, -1), (-1, 0)];

	let neightbours = if y % 2 == 0 {
		even_neighbours
	} else {
		odd_neighbours
	};
	let mut count = 0;
	for (dx, dy) in neightbours.iter() {
		if tiles.contains(&(x + dx, y + dy)) {
			// println!("{},{} has neighbour {},{}", x, y, x + dx, y + dy);
			count += 1;
		}
	}

	count
}

fn print_tiles(tiles: &HashSet<(i64, i64)>) {
	let mut min_x = std::i64::MAX;
	let mut max_x = std::i64::MIN;
	let mut min_y = std::i64::MAX;
	let mut max_y = std::i64::MIN;

	for &(x, y) in tiles.iter() {
		min_x = min_x.min(x);
		max_x = max_x.max(x);
		min_y = min_y.min(y);
		max_y = max_y.max(y);
	}

	min_x = -2;
	max_x = 2;
	min_y = -4;
	max_y = 3;

	println!(
		"Printing map in range {},{} {},{}",
		min_x, max_x, min_y, max_y
	);
	for y in min_y..=max_y {
		if (y + 1000) % 2 == 0 {
			print!(" ");
		}
		for x in min_x..=max_x {
			if tiles.contains(&(x, y)) {
				print!("x ");
			} else {
				print!("  ");
			}
		}
		print!("\n");
	}
}

#[derive(Copy, Clone, Debug)]
enum Direction {
	E,
	SE,
	SW,
	W,
	NW,
	NE,
}

fn parse(input: &str) -> Vec<Vec<Direction>> {
	let mut directions_list = Vec::new();
	for line in input.lines() {
		let mut directions = Vec::new();
		let mut i = 0;
		let bytes = line.as_bytes();
		while i < bytes.len() {
			directions.push(match bytes[i] {
				b'e' => {
					i += 1;
					Direction::E
				}
				b's' => {
					i += 2;
					match bytes[i - 1] {
						b'e' => Direction::SE,
						b'w' => Direction::SW,
						_ => panic!(),
					}
				}
				b'w' => {
					i += 1;
					Direction::W
				}
				b'n' => {
					i += 2;
					match bytes[i - 1] {
						b'e' => Direction::NE,
						b'w' => Direction::NW,
						_ => panic!(),
					}
				}
				_ => panic!(),
			})
		}

		directions_list.push(directions);
	}

	directions_list
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::lib::fetch_input;
	use test::Bencher;

	#[test]
	fn part_one_test_cases() {
		let input = include_str!("input.test1.txt");
		let solver = Day24Solver {};
		assert_eq!(solver.solve_part_one(input), 10);
	}

	#[test]
	fn part_two_test_cases() {
		let input = include_str!("input.test1.txt");
		let solver = Day24Solver {};
		assert_eq!(solver.solve_part_two(input), 2208);
	}

	// #[bench]
	// fn bench_parse_cups(bencher: &mut Bencher) {
	// 	let input = fetch_input(24);
	// 	bencher.iter(|| parse(&input));
	// }

	// #[bench]
	// fn bench_part_one(bencher: &mut Bencher) {
	// 	let input = fetch_input(24);
	// 	let solver = Day24Solver {};
	// 	bencher.iter(|| solver.solve_part_one(&input));
	// }

	// #[bench]
	// fn bench_part_two(bencher: &mut Bencher) {
	// 	let input = fetch_input(24);
	// 	let solver = Day24Solver {};
	// 	bencher.iter(|| solver.solve_part_two(&input));
	// }
}
