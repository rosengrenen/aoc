use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

pub trait Solver {
	fn solve(&self, lines: &Vec<String>, part_two: bool) -> String;
}

pub fn read_lines<P>(filename: P) -> Vec<String>
where
	P: AsRef<Path>,
{
	let file = File::open(filename).expect("Failed to open file");
	io::BufReader::new(file)
		.lines()
		.filter_map(io::Result::ok)
		.collect()
}
