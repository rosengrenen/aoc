import gleam/dict
import gleam/int
import gleam/list
import gleam/option
import gleam/set
import gleam/yielder

import helpers

pub fn part1(input: String) -> String {
  num_antinodes(input, fn(x0, y0, x1, y1, dx, dy) {
    set.new()
    |> set.insert(#(x0 + dx, y0 + dy))
    |> set.insert(#(x1 - dx, y1 - dy))
  })
}

pub fn part2(input: String) -> String {
  num_antinodes(input, fn(x0, y0, x1, y1, dx, dy) {
    yielder.range(0, 100)
    |> yielder.fold(set.new(), fn(acc, n) {
      acc
      |> set.insert(#(x0 + n * dx, y0 + n * dy))
      |> set.insert(#(x1 - n * dx, y1 - n * dy))
    })
  })
}

fn num_antinodes(input, f) {
  let #(antennas, mx, my) = input |> parse
  antennas
  |> dict.to_list
  |> list.fold(set.new(), fn(acc, antenna) {
    let #(_, positions) = antenna
    positions
    |> set.to_list
    |> list.combination_pairs
    |> list.fold(acc, fn(acc, b) {
      let #(#(x0, y0), #(x1, y1)) = b
      let dx = x0 - x1
      let dy = y0 - y1
      acc |> set.union(f(x0, y0, x1, y1, dx, dy))
    })
  })
  |> set.filter(in_bounds(_, mx, my))
  |> set.size
  |> int.to_string
}

fn in_bounds(pos: #(Int, Int), mx, my) {
  pos.0 >= 0 && pos.1 >= 0 && pos.0 <= mx && pos.1 <= my
}

fn parse(input) {
  input
  |> helpers.parse_grid(#(dict.new(), 0, 0), fn(acc, x, y, tile) {
    let #(antennas, mx, my) = acc
    case tile {
      "." -> #(antennas, int.max(x, mx), int.max(y, my))
      name -> {
        let antennas =
          antennas
          |> dict.upsert(name, fn(positions) {
            positions |> option.unwrap(set.new()) |> set.insert(#(x, y))
          })
        #(antennas, int.max(x, mx), int.max(y, my))
      }
    }
  })
}
