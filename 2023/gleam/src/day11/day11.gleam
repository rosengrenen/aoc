import gleam/bool
import gleam/dict
import gleam/int
import gleam/list
import gleam/result
import gleam/set
import gleam/yielder

import helpers

pub fn part1(input: String) -> String {
  calc(input, 2)
}

pub fn part2(input: String) -> String {
  calc(input, 1_000_000)
}

fn calc(input, scale) {
  let #(galaxies, #(max_x, max_y)) = input |> parse
  let empty_rows =
    yielder.range(0, max_y)
    |> yielder.filter(fn(y) {
      yielder.range(0, max_x)
      |> yielder.all(fn(x) { galaxies |> set.contains(#(x, y)) |> bool.negate })
    })
    |> yielder.fold(set.new(), fn(acc, c) { acc |> set.insert(c) })
  let empty_cols =
    yielder.range(0, max_x)
    |> yielder.filter(fn(x) {
      yielder.range(0, max_y)
      |> yielder.all(fn(y) { galaxies |> set.contains(#(x, y)) |> bool.negate })
    })
    |> yielder.fold(set.new(), fn(acc, c) { acc |> set.insert(c) })
  let rows_cumulative = cumulative(empty_rows, max_y, scale)
  let cols_cumulative = cumulative(empty_cols, max_x, scale)
  galaxies
  |> set.map(fn(pos) {
    let #(x, y) = pos
    let assert Ok(shift_x) = cols_cumulative |> dict.get(x)
    let assert Ok(shift_y) = rows_cumulative |> dict.get(y)
    #(x + shift_x, y + shift_y)
  })
  |> set.to_list
  |> list.combination_pairs
  |> list.map(fn(p) { distance(p.0, p.1) })
  |> int.sum
  |> int.to_string
}

fn cumulative(empty, max, scale) {
  yielder.range(0, max)
  |> yielder.fold(dict.new(), fn(acc, y) {
    let prev = acc |> dict.get(y - 1) |> result.unwrap(0)
    case empty |> set.contains(y) {
      True -> acc |> dict.insert(y, prev + scale - 1)
      False -> acc |> dict.insert(y, prev)
    }
  })
}

fn distance(left: #(Int, Int), right: #(Int, Int)) {
  let dx = { left.0 - right.0 } |> int.absolute_value
  let dy = { left.1 - right.1 } |> int.absolute_value
  dx + dy
}

fn parse(input) {
  input
  |> helpers.parse_grid(set.new(), fn(acc, x, y, tile) {
    case tile {
      "#" -> acc |> set.insert(#(x, y))
      _ -> acc
    }
  })
}
