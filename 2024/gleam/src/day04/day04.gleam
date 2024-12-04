import gleam/dict
import gleam/int
import gleam/list
import gleam/result
import gleam/string

import helpers

const directions = [-1, 0, 1]

pub fn part1(input: String) -> String {
  let grid = input |> parse
  grid
  |> dict.to_list
  |> list.map(fn(e) { e.0 })
  |> list.fold(0, fn(acc, coord) {
    let #(x, y) = coord
    helpers.combinations(directions, directions)
    |> list.fold(acc, fn(acc, delta) {
      let #(dx, dy) = delta
      case
        [0, 1, 2, 3]
        |> list.map(fn(i) { grid_get(grid, #(x + dx * i, y + dy * i)) })
        |> string.join("")
        == "XMAS"
      {
        True -> acc + 1
        _ -> acc
      }
    })
  })
  |> int.to_string
}

pub fn part2(input: String) -> String {
  let grid = input |> parse
  grid
  |> dict.to_list
  |> list.map(fn(e) { e.0 })
  |> list.filter(fn(c) { grid_get(grid, c) == "A" })
  |> list.fold(0, fn(acc, coord) {
    let is_mas_cross = fn(coord1, coord2) {
      [grid_get(grid, coord1), grid_get(grid, coord2)]
      |> list.sort(string.compare)
      == ["M", "S"]
    }
    let #(x, y) = coord
    case
      is_mas_cross(#(x - 1, y - 1), #(x + 1, y + 1))
      && is_mas_cross(#(x - 1, y + 1), #(x + 1, y - 1))
    {
      True -> acc + 1
      _ -> acc
    }
  })
  |> int.to_string
}

fn parse(input: String) {
  input
  |> string.split("\n")
  |> list.index_fold(dict.new(), fn(grid, line, y) {
    line
    |> string.split("")
    |> list.index_fold(grid, fn(grid, char, x) {
      grid |> dict.insert(#(x, y), char)
    })
  })
}

fn grid_get(grid: dict.Dict(k, String), key: k) {
  grid |> dict.get(key) |> result.unwrap("")
}
