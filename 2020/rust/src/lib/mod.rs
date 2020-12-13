use reqwest::header::COOKIE;
use std::fs::{read_to_string, write};

pub trait Solver {
	fn solve_part_one(&self, input: &str) -> i64;
	fn solve_part_two(&self, input: &str) -> i64;
}

fn input_from_file(day: i64) -> Option<String> {
	// let input = read_file(&file);
	if let Ok(input) = read_to_string(format!("src/day{:02}/input.txt", day)) {
		Some(input)
	} else {
		None
	}
}

fn input_from_online(day: i64) -> String {
	let endpoint = format!("https://adventofcode.com/2020/day/{}/input", day);
	let session = include_str!("../../aoc.session");
	let client = reqwest::blocking::Client::new();
	match client
		.get(&endpoint)
		.header(COOKIE, format!("session={}", session))
		.send()
	{
		Ok(response) => {
			let status = response.status();
			if status == 200 {
				if let Ok(body) = response.text() {
					body
				} else {
					panic!("Could not parse response body");
				}
			} else {
				panic!("Could not fetch input, either the year or day is invalid, or the cookie string provided does not correspond to a valid session");
			}
		}
		Err(_) => {
			panic!("Could not complete request");
		}
	}
}

pub fn fetch_input(day: i64) -> String {
	if let Some(input) = input_from_file(day) {
		input
	} else {
		let input = input_from_online(day);

		write(format!("src/day{:02}/input.txt", day), &input.trim()).unwrap();

		input
	}
}
