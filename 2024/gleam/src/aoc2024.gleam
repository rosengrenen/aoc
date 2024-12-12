import day01/day01
import day02/day02
import day03/day03
import day04/day04
import day05/day05
import day06/day06
import day07/day07
import day08/day08
import day09/day09
import day10/day10
import day11/day11
import day12/day12
import gleam/int
import gleam/io
import gleam/string
import simplifile

fn day_string(day: Int) -> String {
  let day_string = case day < 10 {
    True -> "0" <> int.to_string(day)
    False -> int.to_string(day)
  }
  "day" <> day_string
}

fn day_runners(day: Int) {
  case day {
    1 -> #(day01.part1, day01.part2)
    2 -> #(day02.part1, day02.part2)
    3 -> #(day03.part1, day03.part2)
    4 -> #(day04.part1, day04.part2)
    5 -> #(day05.part1, day05.part2)
    6 -> #(day06.part1, day06.part2)
    7 -> #(day07.part1, day07.part2)
    8 -> #(day08.part1, day08.part2)
    9 -> #(day09.part1, day09.part2)
    10 -> #(day10.part1, day10.part2)
    11 -> #(day11.part1, day11.part2)
    12 -> #(day12.part1, day12.part2)
    _ -> panic as "unimplemented"
  }
}

fn run_day(day: Int) {
  let #(p1, p2) = day_runners(day)
  let assert Ok(input) =
    simplifile.read(from: "src/" <> day_string(day) <> "/input.txt")
  let input = input |> string.trim
  io.println("Day " <> int.to_string(day))
  io.println("Part 1: " <> p1(input))
  io.println("Part 2: " <> p2(input))
}

pub fn main() {
  run_day(12)
}
