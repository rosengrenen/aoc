import gleam/int
import gleam/list
import gleam/set
import gleam/string

import helpers

pub fn part1(input: String) -> String {
  input
  |> string.to_graphemes
  |> deliver(set.new())
  |> set.size
  |> int.to_string
}

pub fn part2(input: String) -> String {
  let instructions = input |> string.to_graphemes
  let santa = instructions |> helpers.index_filter(fn(i, _) { i % 2 == 0 })
  let robot = instructions |> helpers.index_filter(fn(i, _) { i % 2 == 1 })
  set.new()
  |> deliver(santa, _)
  |> deliver(robot, _)
  |> set.size
  |> int.to_string
}

fn deliver(instructions, visited) {
  instructions
  |> list.fold(#(visited |> set.insert(#(0, 0)), #(0, 0)), fn(acc, dir) {
    let #(visited, #(x, y)) = acc
    let new_pos = case dir {
      "^" -> #(x, y + 1)
      ">" -> #(x + 1, y)
      "v" -> #(x, y - 1)
      "<" -> #(x - 1, y)
      _ -> panic
    }
    #(visited |> set.insert(new_pos), new_pos)
  })
  |> fn(acc) { acc.0 }
}
