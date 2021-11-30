use aoc_util::{Solver, SolverOutput};

#[derive(Default)]
pub struct Day17;

impl Solver for Day17 {
	fn part_one(&self, input: &str) -> SolverOutput {
		const ITERATIONS: usize = 6;
		let (plane, x, y) = parse_plane(input);
		let x_size = x + 2 * ITERATIONS;
		let y_size = y + 2 * ITERATIONS;
		let z_size = 1 + 2 * ITERATIONS;

		let lo_x = -(x as i64) / 2;
		let hi_x = (x as i64 - 1) / 2;
		let lo_y = -(y as i64) / 2;
		let hi_y = (y as i64 - 1) / 2;
		let lo_z = 0;
		let hi_z = 0;

		let mut space = vec![false; x_size * y_size * z_size];

		for (i, &value) in plane.iter().enumerate() {
			if value {
				let ax = (i % x) as i64 - (x / 2) as i64;
				let ay = (y / 2) as i64 - (i / y) as i64;
				set_xyz(&mut space, (x_size, y_size, z_size), (ax, ay, 0), true);
			}
		}

		for i in 0..ITERATIONS as i64 {
			let mut new_space = space.clone();
			for x in lo_x - i - 1..=hi_x + i + 1 {
				for y in lo_y - i - 1..=hi_y + i + 1 {
					for z in lo_z - i - 1..=hi_z + i + 1 {
						let active = get_xyz(&space, (x_size, y_size, z_size), (x, y, z));
						let mut count = 0;
						'delta: for &dx in RANGE.iter() {
							for &dy in RANGE.iter() {
								for &dz in RANGE.iter() {
									if dx == 0 && dy == 0 && dz == 0 {
										continue;
									}

									let ax = x + dx;
									let ay = y + dy;
									let az = z + dz;

									if ax < lo_x - ITERATIONS as i64
										|| ax > hi_x + ITERATIONS as i64
									{
										continue;
									}

									if ay < lo_y - ITERATIONS as i64
										|| ay > hi_y + ITERATIONS as i64
									{
										continue;
									}

									if az < lo_z - ITERATIONS as i64
										|| az > hi_z + ITERATIONS as i64
									{
										continue;
									}

									if get_xyz(&space, (x_size, y_size, z_size), (ax, ay, az)) {
										count += 1;
									}
									if count > 3 {
										break 'delta;
									}
								}
							}
						}
						if active && !(count == 2 || count == 3) {
							set_xyz(&mut new_space, (x_size, y_size, z_size), (x, y, z), false);
						}
						if !active && count == 3 {
							set_xyz(&mut new_space, (x_size, y_size, z_size), (x, y, z), true);
						}
					}
				}
			}
			space = new_space;
		}

		SolverOutput::Num(
			space
				.iter()
				.fold(0, |prev, &curr| if curr { prev + 1 } else { prev }),
		)
	}

	fn part_two(&self, input: &str) -> SolverOutput {
		const ITERATIONS: usize = 6;
		let (plane, x, y) = parse_plane(input);
		let x_size = x + 2 * ITERATIONS;
		let y_size = y + 2 * ITERATIONS;
		let z_size = 1 + 2 * ITERATIONS;
		let w_size = 1 + 2 * ITERATIONS;

		let lo_x = -(x as i64) / 2;
		let hi_x = (x as i64 - 1) / 2;
		let lo_y = -(y as i64) / 2;
		let hi_y = (y as i64 - 1) / 2;
		let lo_z = 0;
		let hi_z = 0;
		let lo_w = 0;
		let hi_w = 0;

		let mut space = vec![false; x_size * y_size * z_size * w_size];

		for (i, &value) in plane.iter().enumerate() {
			if value {
				let ax = (i % x) as i64 - (x / 2) as i64;
				let ay = (y / 2) as i64 - (i / y) as i64;
				set_xyzw(
					&mut space,
					(x_size, y_size, z_size, w_size),
					(ax, ay, 0, 0),
					true,
				);
			}
		}

		for i in 0..ITERATIONS as i64 {
			let mut new_space = space.clone();
			for x in lo_x - i - 1..=hi_x + i + 1 {
				for y in lo_y - i - 1..=hi_y + i + 1 {
					for z in lo_z - i - 1..=hi_z + i + 1 {
						for w in lo_w - i - 1..=hi_w + i + 1 {
							let active =
								get_xyzw(&space, (x_size, y_size, z_size, w_size), (x, y, z, w));
							let mut count = 0;
							'delta: for &dx in RANGE.iter() {
								for &dy in RANGE.iter() {
									for &dz in RANGE.iter() {
										for &dw in RANGE.iter() {
											if dx == 0 && dy == 0 && dz == 0 && dw == 0 {
												continue;
											}

											let ax = x + dx;
											let ay = y + dy;
											let az = z + dz;
											let aw = w + dw;

											if ax < lo_x - ITERATIONS as i64
												|| ax > hi_x + ITERATIONS as i64
											{
												continue;
											}

											if ay < lo_y - ITERATIONS as i64
												|| ay > hi_y + ITERATIONS as i64
											{
												continue;
											}

											if az < lo_z - ITERATIONS as i64
												|| az > hi_z + ITERATIONS as i64
											{
												continue;
											}

											if aw < lo_w - ITERATIONS as i64
												|| aw > hi_w + ITERATIONS as i64
											{
												continue;
											}

											if get_xyzw(
												&space,
												(x_size, y_size, z_size, w_size),
												(ax, ay, az, aw),
											) {
												count += 1;
											}
											if count > 3 {
												break 'delta;
											}
										}
									}
								}
							}
							if active && !(count == 2 || count == 3) {
								set_xyzw(
									&mut new_space,
									(x_size, y_size, z_size, w_size),
									(x, y, z, w),
									false,
								);
							}
							if !active && count == 3 {
								set_xyzw(
									&mut new_space,
									(x_size, y_size, z_size, w_size),
									(x, y, z, w),
									true,
								);
							}
						}
					}
				}
			}
			space = new_space;
		}

		SolverOutput::Num(
			space
				.iter()
				.fold(0, |prev, &curr| if curr { prev + 1 } else { prev }),
		)
	}
}

static RANGE: [i64; 3] = [-1, 0, 1];

fn parse_plane(input: &str) -> (Vec<bool>, usize, usize) {
	let mut space = Vec::new();
	let mut x = 0;
	let mut y = 0;
	for (cy, line) in input.lines().enumerate() {
		if cy + 1 > y {
			y = cy + 1;
		}
		for (cx, &c) in line.as_bytes().iter().enumerate() {
			if cx + 1 > x {
				x = cx + 1;
			}
			if c == b'#' {
				space.push(true);
			} else {
				space.push(false);
			}
		}
	}
	(space, x, y)
}

fn get_xyz(
	vec: &[bool],
	dimension_sizes: (usize, usize, usize),
	position: (i64, i64, i64),
) -> bool {
	let (x_size, y_size, z_size) = dimension_sizes;
	let (x, y, z) = position;
	let ax = (x + (x_size as i64 / 2)) as usize;
	let ay = (y + (y_size as i64 / 2)) as usize;
	let az = (z + (z_size as i64 / 2)) as usize;
	vec[az * y_size * x_size + ay * y_size + ax]
}

fn set_xyz(
	vec: &mut Vec<bool>,
	dimension_sizes: (usize, usize, usize),
	position: (i64, i64, i64),
	value: bool,
) {
	let (x_size, y_size, z_size) = dimension_sizes;
	let (x, y, z) = position;
	let ax = (x + (x_size as i64 / 2)) as usize;
	let ay = (y + (y_size as i64 / 2)) as usize;
	let az = (z + (z_size as i64 / 2)) as usize;
	vec[az * y_size * x_size + ay * y_size + ax] = value;
}

fn get_xyzw(
	vec: &[bool],
	dimension_sizes: (usize, usize, usize, usize),
	position: (i64, i64, i64, i64),
) -> bool {
	let (x_size, y_size, z_size, w_size) = dimension_sizes;
	let (x, y, z, w) = position;
	let ax = (x + (x_size as i64 / 2)) as usize;
	let ay = (y + (y_size as i64 / 2)) as usize;
	let az = (z + (z_size as i64 / 2)) as usize;
	let aw = (w + (w_size as i64 / 2)) as usize;
	vec[aw * z_size * y_size * x_size + az * y_size * x_size + ay * y_size + ax]
}

fn set_xyzw(
	vec: &mut Vec<bool>,
	dimension_sizes: (usize, usize, usize, usize),
	position: (i64, i64, i64, i64),
	value: bool,
) {
	let (x_size, y_size, z_size, w_size) = dimension_sizes;
	let (x, y, z, w) = position;
	let ax = (x + (x_size as i64 / 2)) as usize;
	let ay = (y + (y_size as i64 / 2)) as usize;
	let az = (z + (z_size as i64 / 2)) as usize;
	let aw = (w + (w_size as i64 / 2)) as usize;
	vec[aw * z_size * y_size * x_size + az * y_size * x_size + ay * y_size + ax] = value;
}
