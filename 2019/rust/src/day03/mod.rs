use crate::lib::{Solver, SolverResult};
use std::cmp;

pub struct Day3Solver;

impl Solver for Day3Solver {
	fn solve_part_one(&self, input: &str) -> SolverResult {
		let (first_path, second_path) = parse_paths(input);
		let intersections = find_intersections(&first_path, &second_path);
		let closest_intersection_distance = intersections
			.iter()
			.filter(|Point { x, y }| *x != 0 && *y != 0)
			.fold(
				std::i64::MAX,
				|closest_intersection_distance, intersection| {
					cmp::min(
						closest_intersection_distance,
						manhattan_distance_from_origin(intersection),
					)
				},
			);
		SolverResult::Num(closest_intersection_distance)
	}

	fn solve_part_two(&self, input: &str) -> SolverResult {
		let (first_path, second_path) = parse_paths(input);
		let intersections = find_intersections_with_distance(&first_path, &second_path);
		let closest_intersection_distance = intersections
			.iter()
			.filter(|(Point { x, y }, _)| *x != 0 && *y != 0)
			.fold(
				std::i64::MAX,
				|closest_intersection_distance, (_, distance)| {
					cmp::min(closest_intersection_distance, *distance)
				},
			);
		SolverResult::Num(closest_intersection_distance)
	}
}

#[derive(Copy, Clone, Debug)]
struct Point {
	x: i64,
	y: i64,
}

#[derive(Copy, Clone, Debug, PartialEq)]
enum Direction {
	Horizontal,
	Vertical,
}

#[derive(Copy, Clone, Debug)]
struct Line {
	direction: Direction,
	start: Point,
	end: Point,
}

fn parse_paths(input: &str) -> (Vec<Line>, Vec<Line>) {
	let mut lines = input.lines();
	let first_path: Vec<&str> = lines.next().unwrap().split(',').collect();
	let second_path: Vec<&str> = lines.next().unwrap().split(',').collect();

	(build_path(&first_path), build_path(&second_path))
}

fn build_path(path_instructions: &[&str]) -> Vec<Line> {
	let mut path = Vec::new();
	let mut current_location = Point { x: 0, y: 0 };
	for &instruction in path_instructions {
		let (direction, distance) = instruction.split_at(1);
		let direction = direction.as_bytes()[0];
		let distance: i64 = distance.parse().unwrap();
		let (next_location, direction) = match direction {
			b'U' => (
				Point {
					x: current_location.x,
					y: current_location.y + distance,
				},
				Direction::Vertical,
			),
			b'R' => (
				Point {
					x: current_location.x + distance,
					y: current_location.y,
				},
				Direction::Horizontal,
			),
			b'D' => (
				Point {
					x: current_location.x,
					y: current_location.y - distance,
				},
				Direction::Vertical,
			),
			b'L' => (
				Point {
					x: current_location.x - distance,
					y: current_location.y,
				},
				Direction::Horizontal,
			),
			_ => panic!("Invalid direction"),
		};
		path.push(Line {
			direction,
			start: current_location,
			end: next_location,
		});
		current_location = next_location;
	}
	path
}

fn find_intersections(first_path: &[Line], second_path: &[Line]) -> Vec<Point> {
	let mut intersections = Vec::new();
	for first_path_line in first_path {
		for second_path_line in second_path {
			if let Some(intersection_point) = find_intersection(first_path_line, second_path_line) {
				intersections.push(intersection_point);
			}
		}
	}
	intersections
}

fn find_intersections_with_distance(
	first_path: &[Line],
	second_path: &[Line],
) -> Vec<(Point, i64)> {
	let mut intersections = Vec::new();
	let mut first_path_distance = 0;
	for first_line in first_path {
		let mut second_path_distance = 0;
		for second_line in second_path {
			if let Some(intersection_point) = find_intersection(first_line, second_line) {
				// Total distance travelled by both lines so far plus distance to intersection
				// from the "ends" of those lines
				let total_distance = first_path_distance
					+ second_path_distance
					+ manhattan_distance(&first_line.start, &intersection_point)
					+ manhattan_distance(&second_line.start, &intersection_point);
				intersections.push((intersection_point, total_distance));
			}
			second_path_distance += manhattan_distance(&second_line.start, &second_line.end);
		}
		first_path_distance += manhattan_distance(&first_line.start, &first_line.end);
	}
	intersections
}

fn find_intersection(first_line: &Line, second_line: &Line) -> Option<Point> {
	if first_line.direction == second_line.direction {
		return None;
	}

	if first_line.direction == Direction::Horizontal {
		let min_x = cmp::min(first_line.start.x, first_line.end.x);
		let max_x = cmp::max(first_line.start.x, first_line.end.x);
		let min_y = cmp::min(second_line.start.y, second_line.end.y);
		let max_y = cmp::max(second_line.start.y, second_line.end.y);
		if (min_x..=max_x).contains(&second_line.start.x)
			&& (min_y..=max_y).contains(&first_line.start.y)
		{
			Some(Point {
				x: second_line.start.x,
				y: first_line.start.y,
			})
		} else {
			None
		}
	} else {
		let min_x = cmp::min(second_line.start.x, second_line.end.x);
		let max_x = cmp::max(second_line.start.x, second_line.end.x);
		let min_y = cmp::min(first_line.start.y, first_line.end.y);
		let max_y = cmp::max(first_line.start.y, first_line.end.y);
		if (min_x..=max_x).contains(&first_line.start.x)
			&& (min_y..=max_y).contains(&second_line.start.y)
		{
			Some(Point {
				x: first_line.start.x,
				y: second_line.start.y,
			})
		} else {
			None
		}
	}
}

fn manhattan_distance(first_point: &Point, second_point: &Point) -> i64 {
	(first_point.x - second_point.x).abs() + (first_point.y - second_point.y).abs()
}

fn manhattan_distance_from_origin(point: &Point) -> i64 {
	manhattan_distance(point, &Point { x: 0, y: 0 })
}

#[cfg(test)]
mod tests {
	use super::*;
	use test::Bencher;

	#[test]
	fn part_one_test_cases() {
		let input = include_str!("input.test1.txt");
		let input2 = include_str!("input.test2.txt");
		let input3 = include_str!("input.test3.txt");
		let solver = Day3Solver {};
		assert_eq!(solver.solve_part_one(input), SolverResult::Num(6));
		assert_eq!(solver.solve_part_one(input2), SolverResult::Num(159));
		assert_eq!(solver.solve_part_one(input3), SolverResult::Num(135));
	}

	#[test]
	fn part_two_test_cases() {
		let input = include_str!("input.test1.txt");
		let input2 = include_str!("input.test2.txt");
		let input3 = include_str!("input.test3.txt");
		let solver = Day3Solver {};
		assert_eq!(solver.solve_part_two(input), SolverResult::Num(30));
		assert_eq!(solver.solve_part_two(input2), SolverResult::Num(610));
		assert_eq!(solver.solve_part_two(input3), SolverResult::Num(410));
	}

	#[bench]
	fn bench_part_one(bencher: &mut Bencher) {
		let input = include_str!("input.txt");
		let solver = Day3Solver {};
		bencher.iter(|| solver.solve_part_one(input));
	}

	#[bench]
	fn bench_part_two(bencher: &mut Bencher) {
		let input = include_str!("input.txt");
		let solver = Day3Solver {};
		bencher.iter(|| solver.solve_part_two(input));
	}
}
