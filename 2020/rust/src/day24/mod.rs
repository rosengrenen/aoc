use hashbrown::HashSet;

use crate::lib::Solver;

pub struct Day24Solver;

impl Solver for Day24Solver {
	fn solve_part_one(&self, input: &str) -> i64 {
		let instructions = parse_instructions(input);
		let tiles = tiles_from_instructions(&instructions);
		tiles.len() as i64
	}

	fn solve_part_two(&self, input: &str) -> i64 {
		let instructions = parse_instructions(input);
		let mut tiles = tiles_from_instructions(&instructions);
		for _ in 0..100 {
			tiles = simulate(&tiles);
		}

		tiles.len() as i64
	}
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

fn tiles_from_instructions(instructions: &[Vec<Dir>]) -> HashSet<Pos> {
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
					if pos.y % 2 != 0 {
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
					if pos.y % 2 != 0 {
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

static EVEN_NEIGHBOURS: [(i64, i64); 6] = [(0, 1), (1, 1), (1, 0), (1, -1), (0, -1), (-1, 0)];
static ODD_NEIGHBOURS: [(i64, i64); 6] = [(-1, 1), (0, 1), (1, 0), (0, -1), (-1, -1), (-1, 0)];

fn count_neighbours(tiles: &HashSet<Pos>, pos: Pos) -> i64 {
	let neightbours = if pos.y % 2 == 0 {
		EVEN_NEIGHBOURS
	} else {
		ODD_NEIGHBOURS
	};
	let mut count = 0;
	for (dx, dy) in neightbours.iter() {
		let cur_pos = Pos::new(pos.x + dx, pos.y + dy);
		if tiles.contains(&cur_pos) {
			count += 1;
		}
	}

	count
}

fn simulate(tiles: &HashSet<Pos>) -> HashSet<Pos> {
	let mut new_tiles = HashSet::new();
	for &Pos { x, y } in tiles.iter() {
		let neighbours = if y % 2 == 0 {
			EVEN_NEIGHBOURS
		} else {
			ODD_NEIGHBOURS
		};
		for &(dx, dy) in neighbours.iter() {
			let cur_pos = Pos::new(x + dx, y + dy);
			let count = count_neighbours(&tiles, cur_pos);

			if tiles.contains(&cur_pos) {
				if count == 1 || count == 2 {
					new_tiles.insert(cur_pos);
				}
			} else if count == 2 {
				new_tiles.insert(cur_pos);
			}
		}
	}

	new_tiles
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

	#[bench]
	fn bench_parse_instructions(bencher: &mut Bencher) {
		let input = fetch_input(24);
		bencher.iter(|| parse_instructions(&input));
	}

	#[bench]
	fn bench_part_one(bencher: &mut Bencher) {
		let input = fetch_input(24);
		let solver = Day24Solver {};
		bencher.iter(|| solver.solve_part_one(&input));
	}

	#[bench]
	fn bench_part_two(bencher: &mut Bencher) {
		let input = fetch_input(24);
		let solver = Day24Solver {};
		bencher.iter(|| solver.solve_part_two(&input));
	}
}
