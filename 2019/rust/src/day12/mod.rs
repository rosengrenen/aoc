use std::{cmp::Ordering, collections::HashMap};

use aoc_util::{Solver, SolverOutput};

#[derive(Default)]
pub struct Day12;

impl Solver for Day12 {
	fn part_one(&self, input: &str) -> SolverOutput {
		let moons: Vec<Moon> = input
			.lines()
			.map(|line| {
				let components: Vec<i64> = line.trim()[1..line.len() - 1]
					.split(',')
					.map(|component| {
						component.trim().split('=').collect::<Vec<&str>>()[1]
							.parse::<i64>()
							.unwrap()
					})
					.collect();
				Moon {
					position: Vector {
						x: components[0],
						y: components[1],
						z: components[2],
					},
					velocity: Vector { x: 0, y: 0, z: 0 },
				}
			})
			.collect();
		let mut system = System::new(&moons);
		for _ in 0..1000 {
			system.step();
		}

		SolverOutput::Num(system.total_energy())
	}

	fn part_two(&self, input: &str) -> SolverOutput {
		let moons: Vec<Moon> = input
			.lines()
			.map(|line| {
				let components: Vec<i64> = line.trim()[1..line.len() - 1]
					.split(',')
					.map(|component| {
						component.trim().split('=').collect::<Vec<&str>>()[1]
							.parse::<i64>()
							.unwrap()
					})
					.collect();
				Moon {
					position: Vector {
						x: components[0],
						y: components[1],
						z: components[2],
					},
					velocity: Vector { x: 0, y: 0, z: 0 },
				}
			})
			.collect();
		let mut system = System::new(&moons);
		let system_interval = find_system_interval(&mut system);
		SolverOutput::Num(system_interval)
	}
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
struct Vector {
	x: i64,
	y: i64,
	z: i64,
}

impl Vector {
	fn magnitude(&self) -> i64 {
		self.x.abs() + self.y.abs() + self.z.abs()
	}
}

#[derive(Clone, Copy, Eq, Hash, PartialEq)]
struct Moon {
	position: Vector,
	velocity: Vector,
}

impl Moon {
	fn total_energy(&self) -> i64 {
		self.position.magnitude() * self.velocity.magnitude()
	}
}

#[derive(Clone)]
struct System {
	moons: Vec<Moon>,
}

impl System {
	fn new(moons: &Vec<Moon>) -> System {
		System {
			moons: moons.clone(),
		}
	}

	fn total_energy(&self) -> i64 {
		self.moons
			.iter()
			.fold(0, |previous, moon| previous + moon.total_energy())
	}

	fn step(&mut self) {
		for first in 0..self.moons.len() {
			for second in first + 1..self.moons.len() {
				if second == first {
					continue;
				}
				match self.moons[first]
					.position
					.x
					.cmp(&self.moons[second].position.x)
				{
					Ordering::Equal => (),
					Ordering::Greater => {
						self.moons[first].velocity.x -= 1;
						self.moons[second].velocity.x += 1;
					}
					Ordering::Less => {
						self.moons[first].velocity.x += 1;
						self.moons[second].velocity.x -= 1;
					}
				}
				match self.moons[first]
					.position
					.y
					.cmp(&self.moons[second].position.y)
				{
					Ordering::Equal => (),
					Ordering::Greater => {
						self.moons[first].velocity.y -= 1;
						self.moons[second].velocity.y += 1;
					}
					Ordering::Less => {
						self.moons[first].velocity.y += 1;
						self.moons[second].velocity.y -= 1;
					}
				}
				match self.moons[first]
					.position
					.z
					.cmp(&self.moons[second].position.z)
				{
					Ordering::Equal => (),
					Ordering::Greater => {
						self.moons[first].velocity.z -= 1;
						self.moons[second].velocity.z += 1;
					}
					Ordering::Less => {
						self.moons[first].velocity.z += 1;
						self.moons[second].velocity.z -= 1;
					}
				}
			}
		}
		for moon in self.moons.iter_mut() {
			moon.position.x += moon.velocity.x;
			moon.position.y += moon.velocity.y;
			moon.position.z += moon.velocity.z;
		}
	}
}

fn lcm(a: i64, b: i64) -> i64 {
	a * b / num::integer::gcd(a, b)
}
fn find_system_interval(system: &mut System) -> i64 {
	let mut iteration = 0;
	let mut seen_x: HashMap<Vec<(i64, i64)>, bool> = HashMap::new();
	let mut seen_y: HashMap<Vec<(i64, i64)>, bool> = HashMap::new();
	let mut seen_z: HashMap<Vec<(i64, i64)>, bool> = HashMap::new();
	let mut interval_x: Option<i64> = None;
	let mut interval_y: Option<i64> = None;
	let mut interval_z: Option<i64> = None;
	loop {
		if interval_x != None && interval_y != None && interval_z != None {
			let i_x = interval_x.unwrap();
			let i_y = interval_y.unwrap();
			let i_z = interval_z.unwrap();
			return lcm(i_x, lcm(i_y, i_z));
		}
		if interval_x == None {
			let key: Vec<(i64, i64)> = system
				.moons
				.iter()
				.map(|moon| (moon.position.x, moon.velocity.x))
				.collect();
			if seen_x.contains_key(&key) {
				interval_x = Some(iteration);
			} else {
				seen_x.insert(key, true);
			}
		}
		if interval_y == None {
			let key: Vec<(i64, i64)> = system
				.moons
				.iter()
				.map(|moon| (moon.position.y, moon.velocity.y))
				.collect();
			if seen_y.contains_key(&key) {
				interval_y = Some(iteration);
			} else {
				seen_y.insert(key, true);
			}
		}
		if interval_z == None {
			let key: Vec<(i64, i64)> = system
				.moons
				.iter()
				.map(|moon| (moon.position.z, moon.velocity.z))
				.collect();
			if seen_z.contains_key(&key) {
				interval_z = Some(iteration);
			} else {
				seen_z.insert(key, true);
			}
		}
		system.step();
		iteration += 1;
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn part_one_test_cases() {
		let mut first_system = System::new(&vec![
			Moon {
				position: Vector { x: -1, y: 0, z: 2 },
				velocity: Vector { x: 0, y: 0, z: 0 },
			},
			Moon {
				position: Vector {
					x: 2,
					y: -10,
					z: -7,
				},
				velocity: Vector { x: 0, y: 0, z: 0 },
			},
			Moon {
				position: Vector { x: 4, y: -8, z: 8 },
				velocity: Vector { x: 0, y: 0, z: 0 },
			},
			Moon {
				position: Vector { x: 3, y: 5, z: -1 },
				velocity: Vector { x: 0, y: 0, z: 0 },
			},
		]);

		first_system.step();
		assert_eq!(first_system.moons[0].position, Vector { x: 2, y: -1, z: 1 });
		assert_eq!(
			first_system.moons[0].velocity,
			Vector { x: 3, y: -1, z: -1 }
		);
		assert_eq!(
			first_system.moons[1].position,
			Vector { x: 3, y: -7, z: -4 }
		);
		assert_eq!(first_system.moons[1].velocity, Vector { x: 1, y: 3, z: 3 });
		assert_eq!(first_system.moons[2].position, Vector { x: 1, y: -7, z: 5 });
		assert_eq!(
			first_system.moons[2].velocity,
			Vector { x: -3, y: 1, z: -3 }
		);
		assert_eq!(first_system.moons[3].position, Vector { x: 2, y: 2, z: 0 });
		assert_eq!(
			first_system.moons[3].velocity,
			Vector { x: -1, y: -3, z: 1 }
		);

		first_system.step();
		assert_eq!(
			first_system.moons[0].position,
			Vector { x: 5, y: -3, z: -1 }
		);
		assert_eq!(
			first_system.moons[0].velocity,
			Vector { x: 3, y: -2, z: -2 }
		);
		assert_eq!(first_system.moons[1].position, Vector { x: 1, y: -2, z: 2 });
		assert_eq!(first_system.moons[1].velocity, Vector { x: -2, y: 5, z: 6 });
		assert_eq!(
			first_system.moons[2].position,
			Vector { x: 1, y: -4, z: -1 }
		);
		assert_eq!(first_system.moons[2].velocity, Vector { x: 0, y: 3, z: -6 });
		assert_eq!(first_system.moons[3].position, Vector { x: 1, y: -4, z: 2 });
		assert_eq!(
			first_system.moons[3].velocity,
			Vector { x: -1, y: -6, z: 2 }
		);

		first_system.step();
		assert_eq!(
			first_system.moons[0].position,
			Vector { x: 5, y: -6, z: -1 }
		);
		assert_eq!(first_system.moons[0].velocity, Vector { x: 0, y: -3, z: 0 });
		assert_eq!(first_system.moons[1].position, Vector { x: 0, y: 0, z: 6 });
		assert_eq!(first_system.moons[1].velocity, Vector { x: -1, y: 2, z: 4 });
		assert_eq!(first_system.moons[2].position, Vector { x: 2, y: 1, z: -5 });
		assert_eq!(first_system.moons[2].velocity, Vector { x: 1, y: 5, z: -4 });
		assert_eq!(first_system.moons[3].position, Vector { x: 1, y: -8, z: 2 });
		assert_eq!(first_system.moons[3].velocity, Vector { x: 0, y: -4, z: 0 });

		first_system.step();
		assert_eq!(first_system.moons[0].position, Vector { x: 2, y: -8, z: 0 });
		assert_eq!(
			first_system.moons[0].velocity,
			Vector { x: -3, y: -2, z: 1 }
		);
		assert_eq!(first_system.moons[1].position, Vector { x: 2, y: 1, z: 7 });
		assert_eq!(first_system.moons[1].velocity, Vector { x: 2, y: 1, z: 1 });
		assert_eq!(first_system.moons[2].position, Vector { x: 2, y: 3, z: -6 });
		assert_eq!(first_system.moons[2].velocity, Vector { x: 0, y: 2, z: -1 });
		assert_eq!(first_system.moons[3].position, Vector { x: 2, y: -9, z: 1 });
		assert_eq!(
			first_system.moons[3].velocity,
			Vector { x: 1, y: -1, z: -1 }
		);

		first_system.step();
		assert_eq!(
			first_system.moons[0].position,
			Vector { x: -1, y: -9, z: 2 }
		);
		assert_eq!(
			first_system.moons[0].velocity,
			Vector { x: -3, y: -1, z: 2 }
		);
		assert_eq!(first_system.moons[1].position, Vector { x: 4, y: 1, z: 5 });
		assert_eq!(first_system.moons[1].velocity, Vector { x: 2, y: 0, z: -2 });
		assert_eq!(first_system.moons[2].position, Vector { x: 2, y: 2, z: -4 });
		assert_eq!(first_system.moons[2].velocity, Vector { x: 0, y: -1, z: 2 });
		assert_eq!(
			first_system.moons[3].position,
			Vector { x: 3, y: -7, z: -1 }
		);
		assert_eq!(first_system.moons[3].velocity, Vector { x: 1, y: 2, z: -2 });

		first_system.step();
		assert_eq!(
			first_system.moons[0].position,
			Vector { x: -1, y: -7, z: 3 }
		);
		assert_eq!(first_system.moons[0].velocity, Vector { x: 0, y: 2, z: 1 });
		assert_eq!(first_system.moons[1].position, Vector { x: 3, y: 0, z: 0 });
		assert_eq!(
			first_system.moons[1].velocity,
			Vector {
				x: -1,
				y: -1,
				z: -5
			}
		);
		assert_eq!(first_system.moons[2].position, Vector { x: 3, y: -2, z: 1 });
		assert_eq!(first_system.moons[2].velocity, Vector { x: 1, y: -4, z: 5 });
		assert_eq!(
			first_system.moons[3].position,
			Vector { x: 3, y: -4, z: -2 }
		);
		assert_eq!(first_system.moons[3].velocity, Vector { x: 0, y: 3, z: -1 });

		first_system.step();
		assert_eq!(first_system.moons[0].position, Vector { x: 2, y: -2, z: 1 });
		assert_eq!(first_system.moons[0].velocity, Vector { x: 3, y: 5, z: -2 });
		assert_eq!(
			first_system.moons[1].position,
			Vector { x: 1, y: -4, z: -4 }
		);
		assert_eq!(
			first_system.moons[1].velocity,
			Vector {
				x: -2,
				y: -4,
				z: -4
			}
		);
		assert_eq!(first_system.moons[2].position, Vector { x: 3, y: -7, z: 5 });
		assert_eq!(first_system.moons[2].velocity, Vector { x: 0, y: -5, z: 4 });
		assert_eq!(first_system.moons[3].position, Vector { x: 2, y: 0, z: 0 });
		assert_eq!(first_system.moons[3].velocity, Vector { x: -1, y: 4, z: 2 });

		first_system.step();
		assert_eq!(first_system.moons[0].position, Vector { x: 5, y: 2, z: -2 });
		assert_eq!(first_system.moons[0].velocity, Vector { x: 3, y: 4, z: -3 });
		assert_eq!(
			first_system.moons[1].position,
			Vector { x: 2, y: -7, z: -5 }
		);
		assert_eq!(
			first_system.moons[1].velocity,
			Vector { x: 1, y: -3, z: -1 }
		);
		assert_eq!(first_system.moons[2].position, Vector { x: 0, y: -9, z: 6 });
		assert_eq!(
			first_system.moons[2].velocity,
			Vector { x: -3, y: -2, z: 1 }
		);
		assert_eq!(first_system.moons[3].position, Vector { x: 1, y: 1, z: 3 });
		assert_eq!(first_system.moons[3].velocity, Vector { x: -1, y: 1, z: 3 });

		first_system.step();
		assert_eq!(first_system.moons[0].position, Vector { x: 5, y: 3, z: -4 });
		assert_eq!(first_system.moons[0].velocity, Vector { x: 0, y: 1, z: -2 });
		assert_eq!(
			first_system.moons[1].position,
			Vector { x: 2, y: -9, z: -3 }
		);
		assert_eq!(first_system.moons[1].velocity, Vector { x: 0, y: -2, z: 2 });
		assert_eq!(first_system.moons[2].position, Vector { x: 0, y: -8, z: 4 });
		assert_eq!(first_system.moons[2].velocity, Vector { x: 0, y: 1, z: -2 });
		assert_eq!(first_system.moons[3].position, Vector { x: 1, y: 1, z: 5 });
		assert_eq!(first_system.moons[3].velocity, Vector { x: 0, y: 0, z: 2 });

		first_system.step();
		assert_eq!(first_system.moons[0].position, Vector { x: 2, y: 1, z: -3 });
		assert_eq!(
			first_system.moons[0].velocity,
			Vector { x: -3, y: -2, z: 1 }
		);
		assert_eq!(first_system.moons[1].position, Vector { x: 1, y: -8, z: 0 });
		assert_eq!(first_system.moons[1].velocity, Vector { x: -1, y: 1, z: 3 });
		assert_eq!(first_system.moons[2].position, Vector { x: 3, y: -6, z: 1 });
		assert_eq!(first_system.moons[2].velocity, Vector { x: 3, y: 2, z: -3 });
		assert_eq!(first_system.moons[3].position, Vector { x: 2, y: 0, z: 4 });
		assert_eq!(
			first_system.moons[3].velocity,
			Vector { x: 1, y: -1, z: -1 }
		);

		assert_eq!(first_system.total_energy(), 179);

		let mut second_system = System::new(&vec![
			Moon {
				position: Vector {
					x: -8,
					y: -10,
					z: 0,
				},
				velocity: Vector { x: 0, y: 0, z: 0 },
			},
			Moon {
				position: Vector { x: 5, y: 5, z: 10 },
				velocity: Vector { x: 0, y: 0, z: 0 },
			},
			Moon {
				position: Vector { x: 2, y: -7, z: 3 },
				velocity: Vector { x: 0, y: 0, z: 0 },
			},
			Moon {
				position: Vector { x: 9, y: -8, z: -3 },
				velocity: Vector { x: 0, y: 0, z: 0 },
			},
		]);

		for _ in 0..10 {
			second_system.step();
		}
		assert_eq!(
			second_system.moons[0].position,
			Vector {
				x: -9,
				y: -10,
				z: 1
			}
		);
		assert_eq!(
			second_system.moons[0].velocity,
			Vector {
				x: -2,
				y: -2,
				z: -1
			}
		);
		assert_eq!(
			second_system.moons[1].position,
			Vector { x: 4, y: 10, z: 9 }
		);
		assert_eq!(
			second_system.moons[1].velocity,
			Vector { x: -3, y: 7, z: -2 }
		);
		assert_eq!(
			second_system.moons[2].position,
			Vector {
				x: 8,
				y: -10,
				z: -3
			}
		);
		assert_eq!(
			second_system.moons[2].velocity,
			Vector { x: 5, y: -1, z: -2 }
		);
		assert_eq!(
			second_system.moons[3].position,
			Vector { x: 5, y: -10, z: 3 }
		);
		assert_eq!(
			second_system.moons[3].velocity,
			Vector { x: 0, y: -4, z: 5 }
		);

		for _ in 0..10 {
			second_system.step();
		}
		assert_eq!(
			second_system.moons[0].position,
			Vector {
				x: -10,
				y: 3,
				z: -4
			}
		);
		assert_eq!(
			second_system.moons[0].velocity,
			Vector { x: -5, y: 2, z: 0 }
		);
		assert_eq!(
			second_system.moons[1].position,
			Vector { x: 5, y: -25, z: 6 }
		);
		assert_eq!(
			second_system.moons[1].velocity,
			Vector { x: 1, y: 1, z: -4 }
		);
		assert_eq!(
			second_system.moons[2].position,
			Vector { x: 13, y: 1, z: 1 }
		);
		assert_eq!(
			second_system.moons[2].velocity,
			Vector { x: 5, y: -2, z: 2 }
		);
		assert_eq!(second_system.moons[3].position, Vector { x: 0, y: 1, z: 7 });
		assert_eq!(
			second_system.moons[3].velocity,
			Vector { x: -1, y: -1, z: 2 }
		);

		for _ in 0..10 {
			second_system.step();
		}
		assert_eq!(
			second_system.moons[0].position,
			Vector {
				x: 15,
				y: -6,
				z: -9
			}
		);
		assert_eq!(
			second_system.moons[0].velocity,
			Vector { x: -5, y: 4, z: 0 }
		);
		assert_eq!(
			second_system.moons[1].position,
			Vector {
				x: -4,
				y: -11,
				z: 3
			}
		);
		assert_eq!(
			second_system.moons[1].velocity,
			Vector {
				x: -3,
				y: -10,
				z: 0
			}
		);
		assert_eq!(
			second_system.moons[2].position,
			Vector { x: 0, y: -1, z: 11 }
		);
		assert_eq!(second_system.moons[2].velocity, Vector { x: 7, y: 4, z: 3 });
		assert_eq!(
			second_system.moons[3].position,
			Vector { x: -3, y: -2, z: 5 }
		);
		assert_eq!(
			second_system.moons[3].velocity,
			Vector { x: 1, y: 2, z: -3 }
		);

		for _ in 0..10 {
			second_system.step();
		}
		assert_eq!(
			second_system.moons[0].position,
			Vector {
				x: 14,
				y: -12,
				z: -4
			}
		);
		assert_eq!(
			second_system.moons[0].velocity,
			Vector { x: 11, y: 3, z: 0 }
		);
		assert_eq!(
			second_system.moons[1].position,
			Vector { x: -1, y: 18, z: 8 }
		);
		assert_eq!(
			second_system.moons[1].velocity,
			Vector { x: -5, y: 2, z: 3 }
		);
		assert_eq!(
			second_system.moons[2].position,
			Vector {
				x: -5,
				y: -14,
				z: 8
			}
		);
		assert_eq!(
			second_system.moons[2].velocity,
			Vector { x: 1, y: -2, z: 0 }
		);
		assert_eq!(
			second_system.moons[3].position,
			Vector {
				x: 0,
				y: -12,
				z: -2
			}
		);
		assert_eq!(
			second_system.moons[3].velocity,
			Vector {
				x: -7,
				y: -3,
				z: -3
			}
		);

		for _ in 0..10 {
			second_system.step();
		}
		assert_eq!(
			second_system.moons[0].position,
			Vector { x: -23, y: 4, z: 1 }
		);
		assert_eq!(
			second_system.moons[0].velocity,
			Vector { x: -7, y: -1, z: 2 }
		);
		assert_eq!(
			second_system.moons[1].position,
			Vector {
				x: 20,
				y: -31,
				z: 13
			}
		);
		assert_eq!(second_system.moons[1].velocity, Vector { x: 5, y: 3, z: 4 });
		assert_eq!(
			second_system.moons[2].position,
			Vector { x: -4, y: 6, z: 1 }
		);
		assert_eq!(
			second_system.moons[2].velocity,
			Vector { x: -1, y: 1, z: -3 }
		);
		assert_eq!(
			second_system.moons[3].position,
			Vector { x: 15, y: 1, z: -5 }
		);
		assert_eq!(
			second_system.moons[3].velocity,
			Vector { x: 3, y: -3, z: -3 }
		);

		for _ in 0..10 {
			second_system.step();
		}
		assert_eq!(
			second_system.moons[0].position,
			Vector {
				x: 36,
				y: -10,
				z: 6
			}
		);
		assert_eq!(second_system.moons[0].velocity, Vector { x: 5, y: 0, z: 3 });
		assert_eq!(
			second_system.moons[1].position,
			Vector {
				x: -18,
				y: 10,
				z: 9
			}
		);
		assert_eq!(
			second_system.moons[1].velocity,
			Vector { x: -3, y: -7, z: 5 }
		);
		assert_eq!(
			second_system.moons[2].position,
			Vector {
				x: 8,
				y: -12,
				z: -3
			}
		);
		assert_eq!(
			second_system.moons[2].velocity,
			Vector { x: -2, y: 1, z: -7 }
		);
		assert_eq!(
			second_system.moons[3].position,
			Vector {
				x: -18,
				y: -8,
				z: -2
			}
		);
		assert_eq!(
			second_system.moons[3].velocity,
			Vector { x: 0, y: 6, z: -1 }
		);

		for _ in 0..10 {
			second_system.step();
		}
		assert_eq!(
			second_system.moons[0].position,
			Vector {
				x: -33,
				y: -6,
				z: 5
			}
		);
		assert_eq!(
			second_system.moons[0].velocity,
			Vector { x: -5, y: -4, z: 7 }
		);
		assert_eq!(
			second_system.moons[1].position,
			Vector { x: 13, y: -9, z: 2 }
		);
		assert_eq!(
			second_system.moons[1].velocity,
			Vector { x: -2, y: 11, z: 3 }
		);
		assert_eq!(
			second_system.moons[2].position,
			Vector { x: 11, y: -8, z: 2 }
		);
		assert_eq!(
			second_system.moons[2].velocity,
			Vector { x: 8, y: -6, z: -7 }
		);
		assert_eq!(
			second_system.moons[3].position,
			Vector { x: 17, y: 3, z: 1 }
		);
		assert_eq!(
			second_system.moons[3].velocity,
			Vector {
				x: -1,
				y: -1,
				z: -3
			}
		);

		for _ in 0..10 {
			second_system.step();
		}
		assert_eq!(
			second_system.moons[0].position,
			Vector { x: 30, y: -8, z: 3 }
		);
		assert_eq!(second_system.moons[0].velocity, Vector { x: 3, y: 3, z: 0 });
		assert_eq!(
			second_system.moons[1].position,
			Vector { x: -2, y: -4, z: 0 }
		);
		assert_eq!(
			second_system.moons[1].velocity,
			Vector { x: 4, y: -13, z: 2 }
		);
		assert_eq!(
			second_system.moons[2].position,
			Vector {
				x: -18,
				y: -7,
				z: 15
			}
		);
		assert_eq!(
			second_system.moons[2].velocity,
			Vector { x: -8, y: 2, z: -2 }
		);
		assert_eq!(
			second_system.moons[3].position,
			Vector {
				x: -2,
				y: -1,
				z: -8
			}
		);
		assert_eq!(second_system.moons[3].velocity, Vector { x: 1, y: 8, z: 0 });

		for _ in 0..10 {
			second_system.step();
		}
		assert_eq!(
			second_system.moons[0].position,
			Vector {
				x: -25,
				y: -1,
				z: 4
			}
		);
		assert_eq!(
			second_system.moons[0].velocity,
			Vector { x: 1, y: -3, z: 4 }
		);
		assert_eq!(
			second_system.moons[1].position,
			Vector { x: 2, y: -9, z: 0 }
		);
		assert_eq!(
			second_system.moons[1].velocity,
			Vector {
				x: -3,
				y: 13,
				z: -1
			}
		);
		assert_eq!(
			second_system.moons[2].position,
			Vector {
				x: 32,
				y: -8,
				z: 14
			}
		);
		assert_eq!(
			second_system.moons[2].velocity,
			Vector { x: 5, y: -4, z: 6 }
		);
		assert_eq!(
			second_system.moons[3].position,
			Vector {
				x: -1,
				y: -2,
				z: -8
			}
		);
		assert_eq!(
			second_system.moons[3].velocity,
			Vector {
				x: -3,
				y: -6,
				z: -9
			}
		);

		for _ in 0..10 {
			second_system.step();
		}
		assert_eq!(
			second_system.moons[0].position,
			Vector {
				x: 8,
				y: -12,
				z: -9
			}
		);
		assert_eq!(
			second_system.moons[0].velocity,
			Vector { x: -7, y: 3, z: 0 }
		);
		assert_eq!(
			second_system.moons[1].position,
			Vector {
				x: 13,
				y: 16,
				z: -3
			}
		);
		assert_eq!(
			second_system.moons[1].velocity,
			Vector {
				x: 3,
				y: -11,
				z: -5
			}
		);
		assert_eq!(
			second_system.moons[2].position,
			Vector {
				x: -29,
				y: -11,
				z: -1
			}
		);
		assert_eq!(
			second_system.moons[2].velocity,
			Vector { x: -3, y: 7, z: 4 }
		);
		assert_eq!(
			second_system.moons[3].position,
			Vector {
				x: 16,
				y: -13,
				z: 23
			}
		);
		assert_eq!(second_system.moons[3].velocity, Vector { x: 7, y: 1, z: 1 });

		assert_eq!(second_system.total_energy(), 1940);
	}
}
