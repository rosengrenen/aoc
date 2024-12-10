import gleam/bool
import gleam/dict
import gleam/int
import gleam/list
import gleam/set
import gleam/string

import helpers

pub fn part1(input: String) -> String {
  let grid = input |> parse
  grid
  |> dict.map_values(fn(k, v) {
    case v {
      0 -> num_trails(grid, k, set.new()) |> set.from_list |> set.size
      _ -> 0
    }
  })
  |> dict.values
  |> int.sum
  |> int.to_string
}

pub fn part2(input: String) -> String {
  let grid = input |> parse
  grid
  |> dict.map_values(fn(k, v) {
    case v {
      0 -> num_trails(grid, k, set.new()) |> list.length
      _ -> 0
    }
  })
  |> dict.values
  |> int.sum
  |> int.to_string
}

fn num_trails(grid, start, visited) {
  let #(x, y) = start
  let assert Ok(v) = grid |> dict.get(start)
  case v {
    9 -> [start]
    _ -> {
      [#(0, -1), #(1, 0), #(0, 1), #(-1, 0)]
      |> list.map(fn(d) {
        let #(dx, dy) = d
        #(x + dx, y + dy)
      })
      |> list.filter(fn(pos) { visited |> set.contains(pos) |> bool.negate })
      |> list.filter(fn(pos) {
        case grid |> dict.get(pos) {
          Ok(v1) if v1 == v + 1 -> True
          _ -> False
        }
      })
      |> list.fold([], fn(acc, pos) {
        acc |> list.append(num_trails(grid, pos, visited |> set.insert(pos)))
      })
    }
  }
}

fn parse(input) {
  input
  |> string.split("\n")
  |> helpers.enumerate
  |> list.fold(dict.new(), fn(acc, line) {
    let #(y, line) = line
    line
    |> string.to_graphemes
    |> helpers.enumerate
    |> list.fold(acc, fn(acc, tile) {
      let #(x, tile) = tile
      let assert Ok(tile) = tile |> int.parse
      acc |> dict.insert(#(x, y), tile)
    })
  })
}
