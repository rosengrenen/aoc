use hashbrown::HashSet;

use crate::lib::Solver;

pub struct Day24Solver;

impl Solver for Day24Solver {
	fn solve_part_one(&self, input: &str) -> i64 {
		let instructions = parse_instructions(input);
		let mut tiles = HashSet::new();
		for instruction in instructions.iter() {
			let mut pos = (0, 0);
			for &direction in instruction.iter() {
				match direction {
					Dir::E => pos = (pos.0 + 1, pos.1),
					Dir::NE => {
						if pos.1 % 2 == 0 {
							pos = (pos.0 + 1, pos.1 + 1)
						} else {
							pos = (pos.0, pos.1 + 1)
						}
					}
					Dir::NW => {
						if pos.1 % 2 == 0 {
							pos = (pos.0, pos.1 + 1)
						} else {
							pos = (pos.0 - 1, pos.1 + 1)
						}
					}
					Dir::SE => {
						if pos.1 % 2 == 0 {
							pos = (pos.0 + 1, pos.1 - 1)
						} else {
							pos = (pos.0, pos.1 - 1)
						}
					}
					Dir::SW => {
						if pos.1 % 2 == 0 {
							pos = (pos.0, pos.1 - 1)
						} else {
							pos = (pos.0 - 1, pos.1 - 1)
						}
					}
					Dir::W => pos = (pos.0 - 1, pos.1),
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
		let directions_list = parse_instructions(input);
		let mut tiles = HashSet::new();
		for directions in directions_list.iter() {
			let mut pos = (0, 0);
			for &direction in directions.iter() {
				match direction {
					Dir::E => pos = (pos.0 + 1, pos.1),
					Dir::NE => {
						if pos.1 % 2 == 0 {
							pos = (pos.0 + 1, pos.1 + 1)
						} else {
							pos = (pos.0, pos.1 + 1)
						}
					}
					Dir::NW => {
						if pos.1 % 2 == 0 {
							pos = (pos.0, pos.1 + 1)
						} else {
							pos = (pos.0 - 1, pos.1 + 1)
						}
					}
					Dir::SE => {
						if pos.1 % 2 == 0 {
							pos = (pos.0 + 1, pos.1 - 1)
						} else {
							pos = (pos.0, pos.1 - 1)
						}
					}
					Dir::SW => {
						if pos.1 % 2 == 0 {
							pos = (pos.0, pos.1 - 1)
						} else {
							pos = (pos.0 - 1, pos.1 - 1)
						}
					}
					Dir::W => pos = (pos.0 - 1, pos.1),
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

		for _ in 0..100 {
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
			count += 1;
		}
	}

	count
}

#[derive(Copy, Clone, Debug)]
enum Dir {
	E,
	SE,
	SW,
	W,
	NW,
	NE,
}

fn parse_instructions(input: &str) -> Vec<Vec<Dir>> {
	let mut directions_list = Vec::new();
	for line in input.lines() {
		let mut directions = Vec::new();
		let mut i = 0;
		let bytes = line.as_bytes();
		while i < bytes.len() {
			directions.push(match bytes[i] {
				b'e' => {
					i += 1;
					Dir::E
				}
				b's' => {
					i += 2;
					match bytes[i - 1] {
						b'e' => Dir::SE,
						b'w' => Dir::SW,
						_ => panic!(),
					}
				}
				b'w' => {
					i += 1;
					Dir::W
				}
				b'n' => {
					i += 2;
					match bytes[i - 1] {
						b'e' => Dir::NE,
						b'w' => Dir::NW,
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

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
struct Pos {
	x: i64,
	y: i64,
}

impl Pos {
	fn new(x: i64, y: i64) -> Self {
		Self { x, y }
	}
}

fn tiles_from_instructions(instructions: &Vec<Vec<Dir>>) -> HashSet<Pos> {
	let mut tiles = HashSet::new();
	for instruction in instructions.iter() {
		let mut pos = Pos::new(0, 0);
		for &direction in instruction.iter() {
			match direction {
				Dir::E => pos.x += 1,
				Dir::NE => {
					if pos.y % 2 == 0 {
						pos.x += 1;
					}
					pos.y += 1;
				}
				Dir::NW => {
					if pos.y % 2 == 1 {
						pos.x -= 1;
					}
					pos.y += 1;
				}
				Dir::SE => {
					if pos.y % 2 == 0 {
						pos.x += 1;
					}
					pos.y -= 1;
				}
				Dir::SW => {
					if pos.y % 2 == 1 {
						pos.x -= 1;
					}
					pos.y -= 1;
				}
				Dir::W => pos.x -= 1,
			}
		}

		if tiles.contains(&pos) {
			tiles.remove(&pos);
		} else {
			tiles.insert(pos);
		}
	}

	tiles
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
