import gleam/int
import gleam/list
import gleam/string

pub fn part1(input: String) -> String {
  input
  |> parse
  |> list.map(diffs)
  |> list.count(fn(report) { increasing(report) || decreasing(report) })
  |> int.to_string
}

pub fn part2(input: String) -> String {
  input
  |> parse
  |> list.count(fn(report) {
    report
    |> list.combinations(list.length(report) - 1)
    |> list.map(diffs)
    |> list.any(fn(report) { increasing(report) || decreasing(report) })
  })
  |> int.to_string
}

fn increasing(report: List(Int)) {
  report |> list.all(fn(level) { level > 0 && level <= 3 })
}

fn decreasing(report: List(Int)) {
  report |> list.all(fn(level) { level < 0 && level >= -3 })
}

fn diffs(report: List(Int)) {
  report
  |> list.window_by_2
  |> list.map(fn(entry) { entry.1 - entry.0 })
}

fn parse(input: String) {
  string.split(input, on: "\n")
  |> list.map(fn(line) {
    line
    |> string.split(on: " ")
    |> list.filter_map(int.parse)
  })
}
