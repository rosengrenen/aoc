import gleam/dict
import gleam/int
import gleam/list
import gleam/option.{Some}
import gleam/regexp
import gleam/result
import gleam/set
import gleam/string
import gleam/yielder

import helpers

pub fn part1(input: String) -> String {
  input
  |> string.split("\n")
  |> list.map(parse)
  |> list.fold(set.new(), fn(grid, instruction) {
    let #(command, #(start_x, end_x), #(start_y, end_y)) = instruction
    yielder.range(start_x, end_x)
    |> yielder.fold(grid, fn(grid, x) {
      yielder.range(start_y, end_y)
      |> yielder.fold(grid, fn(grid, y) {
        let coord = #(x, y)
        case command {
          "turn on" -> grid |> set.insert(coord)
          "turn off" -> grid |> set.delete(coord)
          "toggle" ->
            case grid |> set.contains(coord) {
              True -> grid |> set.delete(coord)
              False -> grid |> set.insert(coord)
            }
          _ -> panic
        }
      })
    })
  })
  |> set.size
  |> int.to_string
}

pub fn part2(input: String) -> String {
  input
  |> string.split("\n")
  |> list.map(parse)
  |> list.fold(dict.new(), fn(grid, instruction) {
    let #(command, #(start_x, end_x), #(start_y, end_y)) = instruction
    let brightness_change = case command {
      "turn on" -> 1
      "turn off" -> -1
      "toggle" -> 2
      _ -> panic
    }
    yielder.range(start_x, end_x)
    |> yielder.fold(grid, fn(grid, x) {
      yielder.range(start_y, end_y)
      |> yielder.fold(grid, fn(grid, y) {
        let coord = #(x, y)
        dict.upsert(grid, coord, fn(brightness) {
          int.max(0, { brightness |> option.unwrap(0) } + brightness_change)
        })
      })
    })
  })
  |> dict.values
  |> helpers.sum
  |> int.to_string
}

fn parse(line) {
  let assert Ok(re) =
    regexp.from_string(
      "(turn on|turn off|toggle) (\\d+),(\\d+) through (\\d+),(\\d+)",
    )
  let assert Ok(regexp.Match(_, matches)) = regexp.scan(re, line) |> list.first
  case matches {
    [Some(command), Some(start_x), Some(start_y), Some(end_x), Some(end_y)] -> #(
      command,
      #(
        start_x |> int.parse |> result.unwrap(0),
        end_x |> int.parse |> result.unwrap(0),
      ),
      #(
        start_y |> int.parse |> result.unwrap(0),
        end_y |> int.parse |> result.unwrap(0),
      ),
    )
    _ -> panic
  }
}
