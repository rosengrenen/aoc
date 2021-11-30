use aoc_util::{Solver, SolverOutput};
use hashbrown::HashMap;

#[derive(Default)]
pub struct Day20;

impl Solver for Day20 {
	fn part_one(&self, input: &str) -> SolverOutput {
		let (images, _) = parse_images(input);
		let images_map = fit_map_pieces(images);

		let mut min_x = std::i64::MAX;
		let mut max_x = std::i64::MIN;
		let mut min_y = std::i64::MAX;
		let mut max_y = std::i64::MIN;
		for (&(x, y), _) in images_map.iter() {
			min_x = min_x.min(x);
			max_x = max_x.max(x);
			min_y = min_y.min(y);
			max_y = max_y.max(y);
		}

		let top_right_corner = images_map.get(&(max_x, max_y)).unwrap().id;
		let bottom_right_corner = images_map.get(&(max_x, min_y)).unwrap().id;
		let bottom_left_corner = images_map.get(&(min_x, min_y)).unwrap().id;
		let top_left_corner = images_map.get(&(min_x, max_y)).unwrap().id;

		SolverOutput::Num(
			top_right_corner * bottom_right_corner * bottom_left_corner * top_left_corner,
		)
	}

	fn part_two(&self, input: &str) -> SolverOutput {
		let (images, raw_images_data) = parse_images(input);
		let images_map = fit_map_pieces(images);
		let joined_map = join_map(images_map, raw_images_data);

		let monster_w = 19;
		let monster_h = 2;
		let monster = [
			(18, 0),
			(0, 1),
			(5, 1),
			(6, 1),
			(11, 1),
			(12, 1),
			(17, 1),
			(18, 1),
			(19, 1),
			(1, 2),
			(4, 2),
			(7, 2),
			(10, 2),
			(13, 2),
			(16, 2),
		];
		let mut monster_count = 0;
		'find_monsters: for rotations in 0..=3 {
			for &(flip_h, flip_v) in FLIPS.iter() {
				for y in 0..joined_map.len() - monster_h {
					for x in 0..joined_map[y].len() - monster_w {
						let is_monster = monster.iter().all(|&(mx, my)| {
							let mut ax = x as i64 + mx;
							let mut ay = y as i64 + my;

							if flip_h {
								ax = joined_map.len() as i64 - ax - 1;
							}

							if flip_v {
								ay = joined_map.len() as i64 - ay - 1;
							}

							if rotations > 0 {
								(ax, ay) = rotate_coord(
									(ax as i64, ay as i64),
									joined_map.len() as f64 - 1.0,
									rotations,
								);
							}

							joined_map[ay as usize][ax as usize]
						});
						if is_monster {
							monster_count += 1;
						}
					}
				}
			}

			if monster_count > 0 {
				break 'find_monsters;
			}
		}

		SolverOutput::Num(
			joined_map.iter().fold(0, |prev, cur| {
				prev + cur
					.iter()
					.fold(0, |prev, cur| prev + if *cur { 1 } else { 0 })
			}) - monster_count * 15,
		)
	}
}

const FLIPS: [(bool, bool); 4] = [(false, false), (true, false), (false, true), (true, true)];

fn join_map(
	images_map: HashMap<(i64, i64), Image>,
	raw_images_data: HashMap<i64, Vec<Vec<bool>>>,
) -> Vec<Vec<bool>> {
	let mut min_x = std::i64::MAX;
	let mut max_x = std::i64::MIN;
	let mut min_y = std::i64::MAX;
	let mut max_y = std::i64::MIN;

	for (&(x, y), _) in images_map.iter() {
		min_x = min_x.min(x);
		max_x = max_x.max(x);
		min_y = min_y.min(y);
		max_y = max_y.max(y);
	}

	let mut joined_map =
		vec![vec![false; 8 * (max_x - min_x) as usize + 8]; 8 * (max_y - min_y) as usize + 8];
	for map_y in min_y..=max_y {
		for map_x in min_x..=max_x {
			let image_meta = images_map.get(&(map_x, map_y)).unwrap();
			let image_data = raw_images_data.get(&image_meta.id).unwrap();
			for y in 0..8 {
				for x in 0..8 {
					// Fix rotation
					let mut ay = y;
					let mut ax = x;

					if image_meta.v_flip {
						ay = 7 - ay;
					}

					if image_meta.h_flip {
						ax = 7 - ax;
					}

					if image_meta.rotation > 0 {
						(ax, ay) = rotate_coord((ax, ay), 7.0, image_meta.rotation);
					}

					if image_data[ay as usize + 1][ax as usize + 1] {
						joined_map[8 * (map_y - min_y) as usize + y as usize]
							[8 * (map_x - min_x) as usize + x as usize] = true;
					}
				}
			}
		}
	}

	joined_map
}

fn rotate_coord(coord: (i64, i64), size: f64, turns: i64) -> (i64, i64) {
	let half_size = size / 2.0;
	let (mut x, mut y) = coord;
	for _ in 0..turns {
		let fx = x as f64 - half_size;
		let fy = y as f64 - half_size;
		let (fx, fy) = (fy, -fx);
		x = (fx + half_size) as i64;
		y = (fy + half_size) as i64;
	}

	(x, y)
}

#[derive(Clone, Copy)]
struct Image {
	id: i64,
	top: i64,
	right: i64,
	bottom: i64,
	left: i64,
	h_flip: bool,
	v_flip: bool,
	rotation: i64,
}

fn parse_images(input: &str) -> (Vec<Image>, HashMap<i64, Vec<Vec<bool>>>) {
	let mut raw_images_data = HashMap::new();
	let mut images = Vec::new();
	for raw_image in input.split("\n\n") {
		let id: i64 = raw_image.lines().next().unwrap()[5..=8].parse().unwrap();
		let image_data: Vec<Vec<bool>> = raw_image
			.lines()
			.skip(1)
			.map(|line| line.chars().map(|c| c == '#').collect())
			.collect();

		let mut top = 0;
		let mut right = 0;
		let mut bottom = 0;
		let mut left = 0;
		for i in 0..10 {
			top <<= 1;
			top |= if image_data[0][9 - i] { 1 } else { 0 };
			right <<= 1;
			right |= if image_data[9 - i][9] { 1 } else { 0 };
			bottom <<= 1;
			bottom |= if image_data[9][i] { 1 } else { 0 };
			left <<= 1;
			left |= if image_data[i][0] { 1 } else { 0 };
		}

		images.push(Image {
			id,
			top,
			right,
			bottom,
			left,
			h_flip: false,
			v_flip: false,
			rotation: 0,
		});
		raw_images_data.insert(id, image_data);
	}

	(images, raw_images_data)
}

fn fit_map_pieces(mut images: Vec<Image>) -> HashMap<(i64, i64), Image> {
	let mut images_map = HashMap::new();
	let starting_image = images.pop().unwrap();
	images_map.insert((0, 0), starting_image);
	while !images.is_empty() {
		let mut found_index = 0;
		let mut new_pos_and_image = None;
		'root: for ((pos_x, pos_y), _) in images_map.iter() {
			let deltas = [(0, 1), (1, 0), (0, -1), (-1, 0)];
			for (dx, dy) in deltas.iter() {
				for (i, image) in images.iter().enumerate() {
					if !images_map.contains_key(&(pos_x + dx, pos_y + dy)) {
						let pos_x = pos_x + dx;
						let pos_y = pos_y + dy;
						// This point in the image map does not exist, let's try to fit it
						if let Some(fitted_image) = piece_fits(
							images_map.get(&(pos_x, pos_y - 1)),
							images_map.get(&(pos_x + 1, pos_y)),
							images_map.get(&(pos_x, pos_y + 1)),
							images_map.get(&(pos_x - 1, pos_y)),
							*image,
						) {
							new_pos_and_image = Some(((pos_x, pos_y), fitted_image));
							found_index = i;
							break 'root;
						}
					}
				}
			}
		}

		if let Some((pos, new_image)) = new_pos_and_image {
			images_map.insert(pos, new_image);
			images.remove(found_index);
		} else {
			panic!("Could not place all images");
		}
	}

	images_map
}

fn piece_fits(
	top: Option<&Image>,
	right: Option<&Image>,
	bottom: Option<&Image>,
	left: Option<&Image>,
	image: Image,
) -> Option<Image> {
	for rotation in 0..=3 {
		for &(h_flip, v_flip) in FLIPS.iter() {
			let mut current_image = rotate_image(image, rotation);
			if h_flip {
				current_image = flip_image_horizontal(current_image);
			}

			if v_flip {
				current_image = flip_image_vertical(current_image);
			}

			let mut fits = true;
			if let Some(top_image) = top {
				if top_image.bottom != flip_bits(current_image.top) {
					fits = false;
				}
			}

			if fits {
				if let Some(right_image) = right {
					if right_image.left != flip_bits(current_image.right) {
						fits = false;
					}
				}
			}

			if fits {
				if let Some(bottom_image) = bottom {
					if bottom_image.top != flip_bits(current_image.bottom) {
						fits = false;
					}
				}
			}

			if fits {
				if let Some(left_image) = left {
					if left_image.right != flip_bits(current_image.left) {
						fits = false;
					}
				}
			}

			if fits {
				return Some(current_image);
			}
		}
	}
	None
}

fn flip_bits(bits: i64) -> i64 {
	let mut new_bits = 0;
	for i in 0..10 {
		new_bits <<= 1;
		if bits & 1 << i != 0 {
			new_bits |= 1;
		}
	}

	new_bits
}

fn flip_image_horizontal(image: Image) -> Image {
	Image {
		top: flip_bits(image.top),
		right: flip_bits(image.left),
		bottom: flip_bits(image.bottom),
		left: flip_bits(image.right),
		h_flip: true,
		..image
	}
}

fn flip_image_vertical(image: Image) -> Image {
	Image {
		top: flip_bits(image.bottom),
		right: flip_bits(image.right),
		bottom: flip_bits(image.top),
		left: flip_bits(image.left),
		v_flip: true,
		..image
	}
}

fn rotate_image(image: Image, rotation: usize) -> Image {
	let mut tmp_image = image;
	for _ in 0..rotation {
		tmp_image = rotate_image_once(tmp_image);
	}
	tmp_image
}

fn rotate_image_once(image: Image) -> Image {
	Image {
		top: image.left,
		right: image.top,
		bottom: image.right,
		left: image.bottom,
		rotation: image.rotation + 1 % 4,
		..image
	}
}
