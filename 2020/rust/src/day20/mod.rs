use std::collections::{HashMap, HashSet};

use crate::lib::Solver;

pub struct Day20Solver;

impl Solver for Day20Solver {
	fn solve_part_one(&self, input: &str) -> i64 {
		let (images, _) = parse_images(input);
		let images_map = fit_map_pieces(images);
		// let first_image = {
		// 	let (_, first_image) = images.iter().next().unwrap();
		// 	*first_image
		// };
		// images_map.insert((0, 0), first_image);
		// images.remove(&first_image.id);
		// while !images.is_empty() {
		// 	let mut new_pos_and_image = None;
		// 	'root: for ((pos_x, pos_y), _) in images_map.iter() {
		// 		let deltas = [(0, 1), (1, 0), (0, -1), (-1, 0)];
		// 		for (dx, dy) in deltas.iter() {
		// 			for (_, image) in images.iter() {
		// 				if !images_map.contains_key(&(pos_x + dx, pos_y + dy)) {
		// 					let pos_x = pos_x + dx;
		// 					let pos_y = pos_y + dy;
		// 					// This point in the image map does not exist, let's try to fit it
		// 					if let Some(fitted_image) = piece_fits(
		// 						images_map.get(&(pos_x, pos_y + 1)),
		// 						images_map.get(&(pos_x + 1, pos_y)),
		// 						images_map.get(&(pos_x, pos_y - 1)),
		// 						images_map.get(&(pos_x - 1, pos_y)),
		// 						*image,
		// 					) {
		// 						new_pos_and_image = Some(((pos_x, pos_y), fitted_image));
		// 						break 'root;
		// 					}
		// 				}
		// 			}
		// 		}
		// 	}

		// 	if let Some((pos, new_image)) = new_pos_and_image {
		// 		images_map.insert(pos, new_image);
		// 		images.remove(&new_image.id);
		// 	} else {
		// 		panic!("Could not place all images");
		// 	}
		// }

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

		top_right_corner * bottom_right_corner * bottom_left_corner * top_left_corner
	}

	fn solve_part_two(&self, input: &str) -> i64 {
		let (images, raw_images_data) = parse_images(input);

		let image = images[0];

		// let pos = (8, 0);
		// println!("{:?}", pos);
		// println!("{:?}", rotate_coord(pos, 9.0, 1));
		// println!("{:?}", rotate_coord(pos, 9.0, 1));

		// // return 0;
		// // let meta_data = print_image_meta(image);
		// // let image_data = raw_images_data.get(&image.id).unwrap();
		// // let data_data = print_image_data(image_data, false, false, 0, true);

		// // for i in 0..10 {
		// // 	println!("{}\t{}", meta_data[i], data_data[i]);
		// // }

		// for r in 0..=3 {
		// 	for &(h, v) in &[(false, false), (true, false), (false, true), (true, true)] {
		// 		let mut image = image;
		// 		if r > 0 {
		// 			image = rotate_image(image, r);
		// 		}

		// 		if h {
		// 			image = flip_image_horizontal(image);
		// 		}

		// 		if v {
		// 			image = flip_image_vertical(image);
		// 		}

		// 		let meta_data = print_image_meta(image);
		// 		let image_data = raw_images_data.get(&image.id).unwrap();
		// 		let data_data = print_image_data(image_data, false, false, 0, true);
		// 		let data_data1 = print_image_data(image_data, false, false, r as i64, true);
		// 		let data_data2 = print_image_data(image_data, h, false, r as i64, true);
		// 		let data_data3 = print_image_data(image_data, h, v, r as i64, true);

		// 		println!("{} {} {}", h, v, r);

		// 		for i in 0..10 {
		// 			println!(
		// 				"{}\t{}\t{}\t{}\t{}",
		// 				meta_data[i], data_data[i], data_data1[i], data_data2[i], data_data3[i]
		// 			);
		// 		}
		// 		println!("\n\n");
		// 	}
		// }

		// return 0;
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

		{
			println!("PRINTING META MAP");
			for map_y in min_y..=max_y {
				for y in 0..10 {
					for map_x in min_x..=max_x {
						for x in 0..10 {
							let image_meta = images_map.get(&(map_x, map_y)).unwrap();

							if image_meta.rotation == 0 {
								print!(" ");
								continue;
							}

							if (1..9).contains(&x) && (1..9).contains(&y) {
								print!(" ");
							} else if y == 0 {
								if image_meta.top & 1 << x != 0 {
									print!("#");
								} else {
									print!(".");
								}
							} else if y == 9 {
								if image_meta.bottom & 1 << (9 - x) != 0 {
									print!("#");
								} else {
									print!(".");
								}
							} else if x == 0 {
								if image_meta.left & 1 << (9 - y) != 0 {
									print!("#");
								} else {
									print!(".");
								}
							} else if x == 9 {
								if image_meta.right & 1 << y != 0 {
									print!("#");
								} else {
									print!(".");
								}
							}
						}
						print!(" ");
					}
					print!("\n");
				}
				print!("\n");
			}
		}

		{
			let mut prob = HashSet::new();
			println!("PRINTING MAP");
			for map_y in min_y..=max_y {
				for y in 0..10 {
					for map_x in min_x..=max_x {
						for x in 0..10 {
							let image_meta = images_map.get(&(map_x, map_y)).unwrap();
							let image_data = raw_images_data.get(&image_meta.id).unwrap();

							if image_meta.rotation == 0 {
								print!(" ");
								continue;
							}

							let mut ax = x;
							let mut ay = y;

							if image_meta.h_flip {
								ax = 9 - ax;
							}

							if image_meta.v_flip {
								ay = 9 - ay;
							}

							if image_meta.rotation > 0 {
								(ax, ay) = rotate_coord((ax, ay), 9.0, image_meta.rotation);
							}

							let d = image_data[ay as usize][ax as usize];
							if d {
								print!("#");
							} else {
								print!(".");
							}

							if y == 0 {
								if (image_meta.top & 1 << x != 0) != d {
									prob.insert((map_x, map_y));
								}
							} else if y == 9 {
								if (image_meta.bottom & 1 << (9 - x) != 0) != d {
									prob.insert((map_x, map_y));
								}
							} else if x == 0 {
								if (image_meta.left & 1 << (9 - y) != 0) != d {
									prob.insert((map_x, map_y));
								}
							} else if x == 9 {
								if (image_meta.right & 1 << y != 0) != d {
									prob.insert((map_x, map_y));
								}
							}
						}
						print!(" ");
					}
					print!("\n");
				}
				print!("\n");
			}

			println!("PRINTING META");
			for map_y in min_y..=max_y {
				for map_x in min_x..=max_x {
					let image_meta = images_map.get(&(map_x, map_y)).unwrap();
					print!("{} ", image_meta.id);
				}
				print!("\n");
			}

			for map_y in min_y..=max_y {
				for map_x in min_x..=max_x {
					let image_meta = images_map.get(&(map_x, map_y)).unwrap();
					print!(
						"(h:{}, v:{}, r:{}) ",
						if image_meta.h_flip { 1 } else { 0 },
						if image_meta.v_flip { 1 } else { 0 },
						image_meta.rotation
					);
				}
				print!("\n");
			}

			for p in prob.iter() {
				let image_meta = images_map.get(p).unwrap();
				println!(
					"({}, {}) (h:{}, v:{}, r:{})",
					p.0,
					p.1,
					if image_meta.h_flip { 1 } else { 0 },
					if image_meta.v_flip { 1 } else { 0 },
					image_meta.rotation
				);
			}
		}
		return 0;

		let joined_map = join_map(images_map, raw_images_data);

		println!("PRINTING JOINED MAP WITH GAPS");
		for (i, y) in joined_map.iter().enumerate() {
			if i != 0 && i % 8 == 0 {
				print!("\n");
			}
			for (j, x) in y.iter().enumerate() {
				if j != 0 && j % 8 == 0 {
					print!(" ");
				}
				if *x {
					print!("#");
				} else {
					print!(".");
				}
			}
			print!("\n");
		}

		// return 0;

		// let mut min_x = std::i64::MAX;
		// let mut max_x = std::i64::MIN;
		// let mut min_y = std::i64::MAX;
		// let mut max_y = std::i64::MIN;

		// for map_y in min_y..=max_y {
		// 	for map_x in min_x..=max_x {
		// 		let image_meta = images_map.get(&(map_x, map_y)).unwrap();
		// 		print!("{} ", image_meta.id);
		// 	}
		// 	print!("\n");
		// }

		// let joined_map = join_map(images_map, raw_images_data);

		// return 0;

		// print!("\n");
		// for (i, y) in joined_map.iter().rev().enumerate() {
		// 	if i % 8 == 0 {
		// 		// print!("\n");
		// 	}
		// 	print!("\n");
		// 	for (j, x) in y.iter().enumerate() {
		// 		if j % 8 == 0 {
		// 			// print!(" ");
		// 		}
		// 		if *x {
		// 			print!("#");
		// 		} else {
		// 			print!(".");
		// 		}
		// 	}
		// }
		// print!("\n\n");

		let monster_w = 19;
		let monster_h = 2;
		let monster = [
			(19, 0),
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

		let mut monster_count = 0; // = HashSet::new();

		let mut r = 0;
		let mut h = false;
		let mut v = false;

		'out: for rotations in 0..=3 {
			for &(flip_h, flip_v) in FLIPS.iter() {
				// let mut local_monster_count = 0;
				let mut local_monster_coords = HashSet::new();
				for y in 0..joined_map.len() - monster_h {
					for x in 0..joined_map[y].len() - monster_w {
						// println!("{} {}", x, y);
						if monster.iter().all(|&(mx, my)| {
							let mut ax = x as i64 + mx;
							let mut ay = y as i64 + my;

							// println!("!! {} {} {} {}", ax, ay, mx, my);

							if rotations > 0 {
								(ax, ay) = rotate_coord(
									(ax as i64, ay as i64),
									joined_map.len() as f64 - 1.0,
									rotations,
								);
							}

							if flip_h {
								ax = joined_map.len() as i64 - ax - 1;
							}

							if flip_v {
								ay = joined_map.len() as i64 - ay - 1;
							}

							joined_map[ay as usize][ax as usize]
						}) {
							r = rotations;
							h = flip_h;
							v = flip_v;
							// break 'out;
							println!("FOUND DRAGON AT {} {}", x, y);
							for &(mx, my) in monster.iter() {
								let mut ax = x as i64 + mx;
								let mut ay = y as i64 + my;

								if rotations > 0 {
									(ax, ay) = rotate_coord(
										(ax as i64, ay as i64),
										joined_map.len() as f64 - 1.0,
										rotations,
									);
								}

								if flip_h {
									ax = joined_map.len() as i64 - ax;
								}

								if flip_v {
									ay = joined_map.len() as i64 - ay;
								}

								local_monster_coords.insert((ax, ay));
							}
							// local_monster_count += 1;
						}
					}
				}

				if local_monster_coords.len() > monster_count {
					monster_count = local_monster_coords.len();
				}
			}
		}

		println!("{} {} {}", r, h, v);

		// println!("{}", monster_coords.len());

		joined_map.iter().fold(0, |prev, cur| {
			prev + cur
				.iter()
				.fold(0, |prev, cur| prev + if *cur { 1 } else { 0 })
		}) - monster_count as i64
	}
}

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

	// for map_y in (min_y..=max_y).rev() {
	// 	for y in 0..10 {
	// 		for map_x in min_x..=max_x {
	// 			let image_meta = images_map.get(&(map_x, map_y)).unwrap();
	// 			let image_data = raw_images_data.get(&image_meta.id).unwrap();
	// 			for x in 0..10 {
	// 				if x == 0 || x == 9 || y == 0 || y == 9 {
	// 					// print!(".");
	// 					// continue;
	// 				}
	// 				let mut ay = y;
	// 				let mut ax = x;

	// 				if image_meta.rotation > 0 {
	// 					// println!("Rotating ({}, {}) {} times...", ax, ay, image_meta.rotation);
	// 					(ax, ay) = rotate_coord((ax, ay), 9.0, image_meta.rotation);
	// 					// println!("...to ({}, {})", ax, ay);
	// 				}

	// 				if image_meta.v_flip {
	// 					ay = 9 - ay;
	// 				}

	// 				if image_meta.h_flip {
	// 					ax = 9 - ax;
	// 				}

	// 				print!(
	// 					"{}",
	// 					if image_data[ay as usize][ax as usize] {
	// 						"#"
	// 					} else {
	// 						"."
	// 					}
	// 				);
	// 			}
	// 			print!(" ");
	// 		}
	// 		print!("\n");
	// 	}
	// 	print!("\n");
	// }

	// for map_y in min_y..=max_y {
	// 	for map_x in min_x..=max_x {
	// 		let image_meta = images_map.get(&(map_x, map_y)).unwrap();
	// 		print!("{} ", image_meta.id);
	// 	}
	// 	print!("\n");
	// }

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

					if image_meta.rotation > 0 {
						(ax, ay) = rotate_coord((ax, ay), 7.0, image_meta.rotation);
					}

					if image_meta.v_flip {
						ay = 7 - ay;
					}

					if image_meta.h_flip {
						ax = 7 - ax;
					}

					// let hy = 8 * ((max_y - min_y - map_y) as usize + ay;
					// let hx = 8 * (map_x - min_x) as usize + ax;

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

fn print_image_meta(image: Image) -> Vec<String> {
	let mut rows = Vec::new();
	for y in 0..10 {
		let mut row = String::new();
		for x in 0..10 {
			if (1..9).contains(&x) && (1..9).contains(&y) {
				row.push(' ');
			} else if y == 0 {
				if image.top & 1 << x != 0 {
					row.push('#');
				} else {
					row.push('.');
				}
			} else if y == 9 {
				if image.bottom & 1 << (9 - x) != 0 {
					row.push('#');
				} else {
					row.push('.');
				}
			} else if x == 0 {
				if image.left & 1 << (9 - y) != 0 {
					row.push('#');
				} else {
					row.push('.');
				}
			} else if x == 9 {
				if image.right & 1 << y != 0 {
					row.push('#');
				} else {
					row.push('.');
				}
			}
		}
		rows.push(row);
	}

	rows
}

fn print_image_data(data: &Vec<Vec<bool>>, h: bool, v: bool, r: i64, hollow: bool) -> Vec<String> {
	let mut rows = Vec::new();
	for y in 0..10 {
		let mut row = String::new();
		for x in 0..10 {
			if hollow && (1..9).contains(&x) && (1..9).contains(&y) {
				row.push(' ');
				continue;
			}

			let mut ax = x;
			let mut ay = y;

			if h {
				ax = 9 - ax;
			}

			if v {
				ay = 9 - ay;
			}

			if r > 0 {
				(ax, ay) = rotate_coord((ax, ay), 9.0, r);
			}

			if data[ay as usize][ax as usize] {
				row.push('#');
			} else {
				row.push('.');
			}
		}
		rows.push(row);
	}

	rows
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

#[derive(Clone, Copy, Debug)]
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

	// let raw_images_data: HashMap<i64, Vec<Vec<bool>>> = input
	// 	.split("\n\n")
	// 	.map(|raw_image| {
	// 		let id = raw_image.lines().next().unwrap()[5..=8].parse().unwrap();
	// 		(
	// 			id,
	// 			raw_image
	// 				.lines()
	// 				.skip(1)
	// 				.map(|line| line.chars().map(|c| c == '#').collect())
	// 				.collect(),
	// 		)
	// 	})
	// 	.collect();

	// let mut images = Vec::new();
	// for (id, raw_image) in raw_images_data.iter() {
	// 	let mut top = 0;
	// 	let mut right = 0;
	// 	let mut bottom = 0;
	// 	let mut left = 0;
	// 	for i in 0..10 {
	// 		top <<= 1;
	// 		top |= if raw_image[0][9 - i] { 1 } else { 0 };
	// 		right <<= 1;
	// 		right |= if raw_image[9 - i][9] { 1 } else { 0 };
	// 		bottom <<= 1;
	// 		bottom |= if raw_image[9][i] { 1 } else { 0 };
	// 		left <<= 1;
	// 		left |= if raw_image[i][0] { 1 } else { 0 };
	// 	}
	// 	images.push(Image {
	// 		id: *id,
	// 		top,
	// 		right,
	// 		bottom,
	// 		left,
	// 		h_flip: false,
	// 		v_flip: false,
	// 		rotation: 0,
	// 	});
	// }

	(images, raw_images_data)
}

fn fit_map_pieces(mut images: Vec<Image>) -> HashMap<(i64, i64), Image> {
	let mut images_map = HashMap::new();
	let starting_image = images.pop().unwrap();
	images_map.insert((0, 0), starting_image);
	println!("First is {}", starting_image.id);
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

const FLIPS: [(bool, bool); 4] = [(false, false), (true, false), (false, true), (true, true)];
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

// fn print_image(image: &Image) {
// 	for y in image.data.iter() {
// 		print!("\n");
// 		for x in y.iter() {
// 			if *x {
// 				print!("#");
// 			} else {
// 				print!(".");
// 			}
// 		}
// 	}
// }

#[cfg(test)]
mod tests {
	use super::*;
	use crate::lib::fetch_input;
	use test::Bencher;

	#[test]
	fn part_one_test_cases() {
		let input = include_str!("input.test1.txt");
		let solver = Day20Solver {};
		assert_eq!(solver.solve_part_one(input), 20899048083289);
	}

	#[test]
	fn part_two_test_cases() {
		let input = include_str!("input.test1.txt");
		let solver = Day20Solver {};
		assert_eq!(solver.solve_part_two(input), 273);
	}

	// #[bench]
	// fn bench_parse(bencher: &mut Bencher) {
	// 	let input = fetch_input(20);
	// 	bencher.iter(|| parse(&input));
	// }

	// #[bench]
	// fn bench_part_one(bencher: &mut Bencher) {
	// 	let input = fetch_input(20);
	// 	let solver = Day20Solver {};
	// 	bencher.iter(|| solver.solve_part_one(&input));
	// }

	// #[bench]
	// fn bench_part_two(bencher: &mut Bencher) {
	// 	let input = fetch_input(20);
	// 	let solver = Day20Solver {};
	// 	bencher.iter(|| solver.solve_part_two(&input));
	// }
}
