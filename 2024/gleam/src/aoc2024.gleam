import day01/day01
import day02/day02
import gleam/int
import gleam/io
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
    _ -> panic as "unimplemented"
  }
}

fn run_day(day: Int) {
  let #(p1, p2) = day_runners(day)
  let assert Ok(input) =
    simplifile.read(from: "src/" <> day_string(day) <> "/input.txt")
  io.println("Day " <> int.to_string(day))
  io.println("Part 1: " <> p1(input))
  io.println("Part 2: " <> p2(input))
}

pub fn main() {
  run_day(2)
}
