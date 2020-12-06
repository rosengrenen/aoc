#![feature(test)]

extern crate test;

#[macro_use]
extern crate lazy_static;

use std::env;
use std::time::Instant;

mod day01;
use day01::Day1Solver;
mod day02;
use day02::Day2Solver;
mod day03;
use day03::Day3Solver;
mod day04;
use day04::Day4Solver;
mod day05;
use day05::Day5Solver;
mod day06;
use day06::Day6Solver;

mod lib;
use lib::{read_lines, Solver};

fn get_solver(day: u32) -> (Box<dyn Solver>, String) {
	let file = format!("src/day{:02}/input.txt", day);
	let solver: Box<dyn Solver> = match day {
		1 => Box::new(Day1Solver {}),
		2 => Box::new(Day2Solver {}),
		3 => Box::new(Day3Solver {}),
		4 => Box::new(Day4Solver {}),
		5 => Box::new(Day5Solver {}),
		6 => Box::new(Day6Solver {}),
		n => panic!("The solver for day {} has not been implemented", n),
	};
	(solver, file)
}

fn main() {
	let args: Vec<String> = env::args().collect();
	if args.len() <= 1 {
		panic!("Must input number as first argument");
	}
	let day: u32 = args[1].parse().expect("Not a number");
	let part = if args.len() >= 3 {
		args[2]
			.parse::<i64>()
			.map_or_else(|_| None, |part| Some(part))
	} else {
		None
	};
	let (solver, file) = get_solver(day);
	let mut total_time = 0.0;
	if part.is_none() || part.unwrap() == 1 {
		let part1_now = Instant::now();
		let part1_answer = solver.solve(&read_lines(&file), false);
		let part1_time = part1_now.elapsed().as_secs_f32();
		println!("Day {} (part 1) answer: {}", day, part1_answer);
		println!("Day {} (part 1) time: {}", day, part1_time);
		total_time += part1_time;
	}

	if part.is_none() || part.unwrap() == 2 {
		let part2_now = Instant::now();
		let part2_answer = solver.solve(&read_lines(&file), true);
		let part2_time = part2_now.elapsed().as_secs_f32();
		println!("Day {} (part 2) answer: {}", day, part2_answer);
		println!("Day {} (part 2) time: {}", day, part2_time);
		total_time += part2_time;
	}

	if part.is_none() {
		println!("Day {} total time: {}", day, total_time);
	}
}
