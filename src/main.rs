use std::env;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
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

mod lib;
use lib::Solver;

fn read_lines<P>(filename: P) -> Vec<String>
where
    P: AsRef<Path>,
{
    let file = File::open(filename).expect("Failed to open file");
    io::BufReader::new(file)
        .lines()
        .filter_map(io::Result::ok)
        .collect()
}

fn get_solver(day: u32) -> (Box<dyn Solver>, String) {
    let file = format!("src/day{:02}/input.txt", day);
    let solver: Box<dyn Solver> = match day {
        1 => Box::new(Day1Solver {}),
        2 => Box::new(Day2Solver {}),
        3 => Box::new(Day3Solver {}),
        4 => Box::new(Day4Solver {}),
        5 => Box::new(Day5Solver {}),
        6 => Box::new(Day6Solver {}),
        7 => Box::new(Day7Solver {}),
        8 => Box::new(Day8Solver {}),
        9 => Box::new(Day9Solver {}),
        10 => Box::new(Day10Solver {}),
        11 => Box::new(Day11Solver {}),
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
    let (solver, file) = get_solver(day);
    let now = Instant::now();
    let part1_answer = solver.solve(read_lines(&file), false);
    println!("Day {} (part 1): {}", day, part1_answer);
    let part2_answer = solver.solve(read_lines(&file), true);
    println!("Day {} (part 2): {}", day, part2_answer);
    println!("Time: {}", now.elapsed().as_secs_f32());
}
