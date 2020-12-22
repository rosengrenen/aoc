#![feature(destructuring_assignment)]
#![feature(str_split_once)]
#![feature(test)]

extern crate test;

use num_format::{Locale, ToFormattedString};
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
mod day07;
use day07::Day7Solver;
mod day08;
use day08::Day8Solver;
mod day09;
use day09::Day9Solver;
mod day10;
use day10::Day10Solver;
mod day11;
use day11::Day11Solver;
mod day12;
use day12::Day12Solver;
mod day13;
use day13::Day13Solver;
mod day14;
use day14::Day14Solver;
mod day15;
use day15::Day15Solver;
mod day16;
use day16::Day16Solver;
mod day17;
use day17::Day17Solver;
mod day18;
use day18::Day18Solver;
mod day19;
use day19::Day19Solver;
mod day20;
use day20::Day20Solver;
mod day21;
use day21::Day21Solver;
mod day22;
use day22::Day22Solver;

mod lib;
use lib::{fetch_input, Solver};

fn get_solver(day: i64) -> Option<Box<dyn Solver>> {
	match day {
		1 => Some(Box::new(Day1Solver {})),
		2 => Some(Box::new(Day2Solver {})),
		3 => Some(Box::new(Day3Solver {})),
		4 => Some(Box::new(Day4Solver {})),
		5 => Some(Box::new(Day5Solver {})),
		6 => Some(Box::new(Day6Solver {})),
		7 => Some(Box::new(Day7Solver {})),
		8 => Some(Box::new(Day8Solver {})),
		9 => Some(Box::new(Day9Solver {})),
		10 => Some(Box::new(Day10Solver {})),
		11 => Some(Box::new(Day11Solver {})),
		12 => Some(Box::new(Day12Solver {})),
		13 => Some(Box::new(Day13Solver {})),
		14 => Some(Box::new(Day14Solver {})),
		15 => Some(Box::new(Day15Solver {})),
		16 => Some(Box::new(Day16Solver {})),
		17 => Some(Box::new(Day17Solver {})),
		18 => Some(Box::new(Day18Solver {})),
		19 => Some(Box::new(Day19Solver {})),
		20 => Some(Box::new(Day20Solver {})),
		21 => Some(Box::new(Day21Solver {})),
		22 => Some(Box::new(Day22Solver {})),
		_ => None,
	}
}

fn solve_day(day: i64, part: Option<i64>) {
	let solver = if let Some(solver) = get_solver(day) {
		solver
	} else {
		return;
	};
	print_separator();
	let input = fetch_input(day);
	if part.is_none() || part.unwrap() == 1 {
		let part1_now = Instant::now();
		let part1_answer = solver.solve_part_one(&input);
		let part1_time = part1_now.elapsed().as_nanos() as i64;
		print_answer(day, 1, part1_time, part1_answer)
	}

	if part.is_none() || part.unwrap() == 2 {
		let part2_now = Instant::now();
		let part2_answer = solver.solve_part_two(&input);
		let part2_time = part2_now.elapsed().as_nanos() as i64;
		print_answer(day, 2, part2_time, part2_answer)
	}
}

fn print_header() {
	print_top_border();
	println!("| Task          | {:<26} | {:<20} |", "Time (ns)", "Answer");
}

fn print_separator() {
	println!(">{:-<15}+{:-<28}+{:-<22}<", "", "", "");
}

fn print_top_border() {
	println!("/{:-<15}v{:-<28}v{:-<22}\\", "", "", "");
}

fn print_bottom_border() {
	println!("\\{:-<15}^{:-<28}^{:-<22}/", "", "", "");
}

fn print_answer(day: i64, part: i64, time: i64, answer: i64) {
	println!(
		"| Day {:02} part {} | {:<26} | {:<20} |",
		day,
		part,
		time.to_formatted_string(&Locale::en),
		answer
	);
}

fn main() {
	let args: Vec<String> = env::args().collect();
	if args.len() >= 2 {
		let day: i64 = args[1].parse().expect("Not a number");
		let part = if args.len() >= 3 {
			args[2].parse::<i64>().map_or_else(|_| None, Some)
		} else {
			None
		};
		if part.is_some() && part.unwrap() != 1 && part.unwrap() != 2 {
			panic!("Part can only be 1 or 2");
		}
		print_header();
		solve_day(day, part);
		print_bottom_border();
	} else {
		print_header();
		for day in 0..=25 {
			solve_day(day, None);
		}
		print_bottom_border();
	}
}
