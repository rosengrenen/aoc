use crate::lib::Solver;
use hashbrown::HashSet;

pub struct Day17Solver;

impl Solver for Day17Solver {
	fn solve_part_one(&self, input: &str) -> i64 {
		let mut input = parse(input);
		let mut min_x = std::i64::MAX;
		let mut max_x = std::i64::MIN;
		let mut min_y = std::i64::MAX;
		let mut max_y = std::i64::MIN;
		let mut min_z = std::i64::MAX;
		let mut max_z = std::i64::MIN;
		for &(x, y, z) in input.iter() {
			if x < min_x {
				min_x = x;
			}
			if x > max_x {
				max_x = x;
			}
			if y < min_y {
				min_y = y;
			}
			if y > max_y {
				max_y = y;
			}
			if z < min_z {
				min_z = z;
			}
			if z > max_z {
				max_z = z;
			}
		}
		for _ in 0..6 {
			let mut new_input = input.clone();
			let mut new_min_x = min_x;
			let mut new_max_x = max_x;
			let mut new_min_y = min_y;
			let mut new_max_y = max_y;
			let mut new_min_z = min_z;
			let mut new_max_z = max_z;
			for x in min_x - 1..=max_x + 1 {
				for y in min_y - 1..=max_y + 1 {
					for z in min_z - 1..=max_z + 1 {
						// println!("z:{}", z);
						let active = input.contains(&(x, y, z));
						let mut count = 0;
						'dx: for dx in -1..=1 {
							for dy in -1..=1 {
								for dz in -1..=1 {
									if dx == 0 && dy == 0 && dz == 0 {
										continue;
									}
									if input.contains(&(x + dx, y + dy, z + dz)) {
										count += 1;
									}
									if count > 3 {
										break 'dx;
									}
								}
							}
						}
						if active && !(count == 2 || count == 3) {
							new_input.remove(&(x, y, z));
						}
						if !active && count == 3 {
							new_input.insert((x, y, z));
							if x < new_min_x {
								new_min_x = x;
							}
							if x > new_max_x {
								new_max_x = x;
							}
							if y < new_min_y {
								new_min_y = y;
							}
							if y > new_max_y {
								new_max_y = y;
							}
							if z < new_min_z {
								new_min_z = z;
							}
							if z > new_max_z {
								new_max_z = z;
							}
						}
					}
				}
			}
			input = new_input;
			min_x = new_min_x;
			max_x = new_max_x;
			min_y = new_min_y;
			max_y = new_max_y;
			min_z = new_min_z;
			max_z = new_max_z;
		}
		input.len() as i64
	}

	fn solve_part_two(&self, input: &str) -> i64 {
		let mut input = parse2(input);
		let mut min_x = std::i64::MAX;
		let mut max_x = std::i64::MIN;
		let mut min_y = std::i64::MAX;
		let mut max_y = std::i64::MIN;
		let mut min_z = std::i64::MAX;
		let mut max_z = std::i64::MIN;
		let mut min_w = std::i64::MAX;
		let mut max_w = std::i64::MIN;
		for &(x, y, z, w) in input.iter() {
			if x < min_x {
				min_x = x;
			}
			if x > max_x {
				max_x = x;
			}
			if y < min_y {
				min_y = y;
			}
			if y > max_y {
				max_y = y;
			}
			if z < min_z {
				min_z = z;
			}
			if z > max_z {
				max_z = z;
			}
			if w < min_w {
				min_w = w;
			}
			if w > max_w {
				max_w = w;
			}
		}

		for _ in 0..6 {
			let mut new_input = input.clone();
			let mut new_min_x = min_x;
			let mut new_max_x = max_x;
			let mut new_min_y = min_y;
			let mut new_max_y = max_y;
			let mut new_min_z = min_z;
			let mut new_max_z = max_z;
			let mut new_min_w = min_w;
			let mut new_max_w = max_w;
			for x in min_x - 1..=max_x + 1 {
				for y in min_y - 1..=max_y + 1 {
					for z in min_z - 1..=max_z + 1 {
						for w in min_w - 1..=max_w + 1 {
							// println!("z:{}", z);
							let active = input.contains(&(x, y, z, w));
							let mut count = 0;
							'dx: for dx in -1..=1 {
								for dy in -1..=1 {
									for dz in -1..=1 {
										for dw in -1..=1 {
											if dx == 0 && dy == 0 && dz == 0 && dw == 0 {
												continue;
											}
											if input.contains(&(x + dx, y + dy, z + dz, w + dw)) {
												count += 1;
											}
											if count > 3 {
												break 'dx;
											}
										}
									}
								}
							}
							if active && !(count == 2 || count == 3) {
								new_input.remove(&(x, y, z, w));
							}
							if !active && count == 3 {
								new_input.insert((x, y, z, w));
								if x < new_min_x {
									new_min_x = x;
								}
								if x > new_max_x {
									new_max_x = x;
								}
								if y < new_min_y {
									new_min_y = y;
								}
								if y > new_max_y {
									new_max_y = y;
								}
								if z < new_min_z {
									new_min_z = z;
								}
								if z > new_max_z {
									new_max_z = z;
								}
								if w < new_min_w {
									new_min_w = w;
								}
								if w > new_max_w {
									new_max_w = w;
								}
							}
						}
					}
				}
			}
			input = new_input;
			min_x = new_min_x;
			max_x = new_max_x;
			min_y = new_min_y;
			max_y = new_max_y;
			min_z = new_min_z;
			max_z = new_max_z;
			min_w = new_min_w;
			max_w = new_max_w;
		}
		input.len() as i64
	}
}

fn parse(input: &str) -> HashSet<(i64, i64, i64)> {
	let mut space = HashSet::new();
	for (y, line) in input.lines().enumerate() {
		for (x, c) in line.as_bytes().iter().enumerate() {
			if *c == b'#' {
				space.insert((x as i64, y as i64, 0));
			}
		}
	}
	space
}

fn parse2(input: &str) -> HashSet<(i64, i64, i64, i64)> {
	let mut space = HashSet::new();
	for (y, line) in input.lines().enumerate() {
		for (x, c) in line.as_bytes().iter().enumerate() {
			if *c == b'#' {
				space.insert((x as i64, y as i64, 0, 0));
			}
		}
	}
	space
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::lib::fetch_input;
	use test::Bencher;

	#[test]
	fn part_one_test_cases() {
		let input = include_str!("input.test1.txt");
		let solver = Day17Solver {};
		assert_eq!(solver.solve_part_one(input), 112);
	}

	#[test]
	fn part_two_test_cases() {
		let input = include_str!("input.test1.txt");
		let solver = Day17Solver {};
		assert_eq!(solver.solve_part_two(input), 848);
	}

	#[bench]
	fn bench_parse(bencher: &mut Bencher) {
		let input = fetch_input(17);
		bencher.iter(|| parse(&input));
	}

	#[bench]
	fn bench_part_one(bencher: &mut Bencher) {
		let input = fetch_input(17);
		let solver = Day17Solver {};
		bencher.iter(|| solver.solve_part_one(&input));
	}

	#[bench]
	fn bench_part_two(bencher: &mut Bencher) {
		let input = fetch_input(17);
		let solver = Day17Solver {};
		bencher.iter(|| solver.solve_part_two(&input));
	}
}
