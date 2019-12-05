use std::fs::File;
use std::io::{self, prelude::*};

fn solve(lines: Vec<String>, part_two: bool) -> String {
    let program_raw = &lines[0];
    let program_orig: Vec<i32> = program_raw
        .split(',')
        .map(|s| s.parse::<i32>().unwrap())
        .collect();
    let mut program = program_orig.clone();
    run_program(&mut program);
    "abc".to_string()
}

fn run_program(program: &mut Vec<i32>) {
    let mut pointer = 0;
    loop {
        println!("----- NEW RUN -----");
        println!("Pointer: {}", pointer);
        let instruction_options = program[pointer as usize];
        println!("Instruction options: {}", instruction_options);
        let op_code = instruction_options % 100;
        if op_code == 99 {
            break;
        }
        let a_mode = (instruction_options / 100) % 10;
        let b_mode = (instruction_options / 1000) % 10;
        let a_val = program[pointer + 1];
        let b_val = program[pointer + 2];
        let c = program[pointer + 3];

        match op_code {
            1 => {
                let a = if a_mode == 0 {
                    program[a_val as usize]
                } else if a_mode == 1 {
                    a_val
                } else {
                    panic!("Invalid mode: {}", a_mode);
                };
                let b = if b_mode == 0 {
                    program[b_val as usize]
                } else if b_mode == 1 {
                    b_val
                } else {
                    panic!("Invalid mode: {}", a_mode);
                };
                program[c as usize] = a + b;
                pointer += 4;
                println!("Put {} at {}", a + b, c);
            }
            2 => {
                let a = if a_mode == 0 {
                    program[a_val as usize]
                } else if a_mode == 1 {
                    a_val
                } else {
                    panic!("Invalid mode: {}", a_mode);
                };
                let b = if b_mode == 0 {
                    program[b_val as usize]
                } else if b_mode == 1 {
                    b_val
                } else {
                    panic!("Invalid mode: {}", a_mode);
                };
                program[c as usize] = a * b;
                pointer += 4;
                println!("Put {} at {}", a * b, c);
            }
            3 => {
                print!("Input: ");
                io::stdout().flush();
                let mut input = String::new();
                io::stdin().read_line(&mut input).expect("Invalid input");
                let input: i32 = input.trim().parse().expect("Not a number");

                // let modify_index = program[a as usize] as usize;
                program[a_val as usize] = input;
                pointer += 2;
                println!("Put {} at {}", input, a_val);
            }
            4 => {
                println!("Output: {}", program[a_val as usize]);
                pointer += 2;
            }
            5 => {
                let a = if a_mode == 0 {
                    program[a_val as usize]
                } else if a_mode == 1 {
                    a_val
                } else {
                    panic!("Invalid mode: {}", a_mode);
                };
                let b = if b_mode == 0 {
                    program[b_val as usize]
                } else if b_mode == 1 {
                    b_val
                } else {
                    panic!("Invalid mode: {}", a_mode);
                };
                if (a != 0) {
                    pointer = b as usize;
                } else {
                    pointer += 3;
                }
            }
            6 => {
                let a = if a_mode == 0 {
                    program[a_val as usize]
                } else if a_mode == 1 {
                    a_val
                } else {
                    panic!("Invalid mode: {}", a_mode);
                };
                let b = if b_mode == 0 {
                    program[b_val as usize]
                } else if b_mode == 1 {
                    b_val
                } else {
                    panic!("Invalid mode: {}", a_mode);
                };
                if (a == 0) {
                    pointer = b as usize;
                } else {
                    pointer += 3;
                }
            }
            7 => {
                let a = if a_mode == 0 {
                    program[a_val as usize]
                } else if a_mode == 1 {
                    a_val
                } else {
                    panic!("Invalid mode: {}", a_mode);
                };
                let b = if b_mode == 0 {
                    program[b_val as usize]
                } else if b_mode == 1 {
                    b_val
                } else {
                    panic!("Invalid mode: {}", a_mode);
                };
                if (a < b) {
                    program[c as usize] = 1;
                } else {
                    program[c as usize] = 0;
                }
                pointer += 4;
            }
            8 => {
                let a = if a_mode == 0 {
                    program[a_val as usize]
                } else if a_mode == 1 {
                    a_val
                } else {
                    panic!("Invalid mode: {}", a_mode);
                };
                let b = if b_mode == 0 {
                    program[b_val as usize]
                } else if b_mode == 1 {
                    b_val
                } else {
                    panic!("Invalid mode: {}", a_mode);
                };
                if (a == b) {
                    program[c as usize] = 1;
                } else {
                    program[c as usize] = 0;
                }
                pointer += 4;
            }
            _ => panic!("Invalid op code: {} at {}", op_code, pointer),
        };
    }
}

fn main() {
    let file = File::open("input.txt").expect("Failed to open file");
    let lines = io::BufReader::new(file)
        .lines()
        .filter_map(io::Result::ok)
        .collect();

    println!("{}", solve(lines, false));
}
