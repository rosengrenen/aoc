import gleam/dict
import gleam/int
import gleam/list
import gleam/result
import gleam/string

pub fn part1(input: String) -> String {
  let #(left, right) = parse(input)
  let left = list.sort(left, int.compare)
  let right = list.sort(right, int.compare)
  list.zip(left, right)
  |> list.map(fn(entry) { int.absolute_value(entry.0 - entry.1) })
  |> list.fold(0, fn(a, b) { a + b })
  |> int.to_string()
}

pub fn part2(input: String) -> String {
  let #(left, right) = parse(input)
  let frequency =
    right
    |> list.fold(dict.new(), fn(freq, n) {
      let count = freq |> dict.get(n) |> result.unwrap(0)
      freq |> dict.insert(n, count + 1)
    })
  left
  |> list.map(fn(n) { n * result.unwrap(frequency |> dict.get(n), 0) })
  |> list.fold(0, fn(a, b) { a + b })
  |> int.to_string()
}

fn parse(input: String) {
  let entries =
    string.split(input, on: "\n")
    |> list.map(parse_line)
  let left_list = entries |> list.map(fn(entry) { entry.0 })
  let right_list = entries |> list.map(fn(entry) { entry.1 })
  #(left_list, right_list)
}

fn parse_line(line: String) {
  let assert Ok(#(left, right)) = string.split_once(line, on: "   ")
  let assert Ok(left) = int.parse(left)
  let assert Ok(right) = int.parse(right)
  #(left, right)
}
