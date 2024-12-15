import gleam/dict
import gleam/int
import gleam/list
import gleam/result
import gleam/string
import helpers

pub fn part1(input: String) -> String {
  let #(map, robot, movement) = input |> parse
  let map =
    movement
    |> list.fold(#(map, robot), fn(acc, movement) {
      let #(map, robot) = acc
      case movement {
        "^" -> move(map, robot, #(0, -1))
        ">" -> move(map, robot, #(1, 0))
        "v" -> move(map, robot, #(0, 1))
        "<" -> move(map, robot, #(-1, 0))
        _ -> panic
      }
    })
    |> helpers.tuple_first
  map
  |> dict.to_list
  |> list.filter(fn(e) { e.1 == "O" })
  |> list.map(fn(e) { e.0 })
  |> list.map(fn(e) { e.1 * 100 + e.0 })
  |> int.sum
  |> int.to_string
}

pub fn part2(input: String) -> String {
  let #(map, robot, movement) = input |> parse
  let map = map |> widen
  let robot = #(robot.0 * 2, robot.1)
  let map =
    movement
    |> list.fold(#(map, robot), fn(acc, movement) {
      let #(map, robot) = acc
      case movement {
        "^" -> move_wide(map, robot, #(0, -1))
        ">" -> move_wide(map, robot, #(1, 0))
        "v" -> move_wide(map, robot, #(0, 1))
        "<" -> move_wide(map, robot, #(-1, 0))
        _ -> panic
      }
    })
    |> helpers.tuple_first
  map
  |> dict.to_list
  |> list.filter(fn(e) { e.1 == "[" })
  |> list.map(fn(e) { e.0 })
  |> list.map(fn(e) { e.1 * 100 + e.0 })
  |> int.sum
  |> int.to_string
}

fn widen(map: dict.Dict(#(Int, Int), String)) {
  map
  |> dict.fold(dict.new(), fn(acc, pos, tile) {
    case tile {
      "@" ->
        acc
        |> dict.insert(#(pos.0 * 2, pos.1), tile)
        |> dict.insert(#(pos.0 * 2 + 1, pos.1), ".")
      "O" ->
        acc
        |> dict.insert(#(pos.0 * 2, pos.1), "[")
        |> dict.insert(#(pos.0 * 2 + 1, pos.1), "]")
      _ ->
        acc
        |> dict.insert(#(pos.0 * 2, pos.1), tile)
        |> dict.insert(#(pos.0 * 2 + 1, pos.1), tile)
    }
  })
}

fn move_wide(map, robot: #(Int, Int), dir: #(Int, Int)) {
  let next_pos = #(robot.0 + dir.0, robot.1 + dir.1)
  let next_tile = map |> dict.get(next_pos) |> result.unwrap("")
  case next_tile {
    "." | "@" -> #(
      map |> dict.insert(robot, ".") |> dict.insert(next_pos, "@"),
      next_pos,
    )
    "#" -> #(map, robot)
    "[" | "]" ->
      case move_boxes_wide(map, next_pos, dir) {
        Ok(map) -> #(
          map |> dict.insert(robot, ".") |> dict.insert(next_pos, "@"),
          next_pos,
        )
        _ -> #(map, robot)
      }
    _ -> panic
  }
}

fn move_boxes_wide(map, pos: #(Int, Int), dir: #(Int, Int)) {
  let next_pos = #(pos.0 + dir.0, pos.1 + dir.1)
  case map |> dict.get(pos) |> result.unwrap("") {
    "." | "@" ->
      Ok(
        map
        |> dict.insert(pos, map |> dict.get(pos) |> result.unwrap("_")),
      )
    "#" -> Error(Nil)
    "[" -> {
      case dir.0 {
        0 -> {
          move_boxes_wide(map, next_pos, dir)
          |> result.map(fn(map) {
            move_boxes_wide(map, #(next_pos.0 + 1, next_pos.1), dir)
          })
          |> result.flatten
          |> result.map(fn(map) {
            map
            |> dict.insert(pos, ".")
            |> dict.insert(#(pos.0 + 1, pos.1), ".")
            |> dict.insert(#(next_pos.0, next_pos.1), "[")
            |> dict.insert(#(next_pos.0 + 1, next_pos.1), "]")
          })
        }
        _ ->
          move_boxes_wide(map, next_pos, dir)
          |> result.map(fn(map) { map |> dict.insert(next_pos, "[") })
      }
    }
    "]" ->
      case dir.0 {
        0 -> move_boxes_wide(map, #(pos.0 - 1, pos.1), dir)
        _ ->
          move_boxes_wide(map, next_pos, dir)
          |> result.map(fn(map) { map |> dict.insert(next_pos, "]") })
      }
    _ -> panic
  }
}

fn move(map, robot: #(Int, Int), dir: #(Int, Int)) {
  let next_pos = #(robot.0 + dir.0, robot.1 + dir.1)
  case map |> dict.get(next_pos) |> result.unwrap("") {
    "." | "@" -> #(
      map |> dict.insert(robot, ".") |> dict.insert(next_pos, "@"),
      next_pos,
    )
    "#" -> #(map, robot)
    "O" ->
      case move_boxes(map, next_pos, dir) {
        Ok(map) -> #(
          map |> dict.insert(robot, ".") |> dict.insert(next_pos, "@"),
          next_pos,
        )
        _ -> #(map, robot)
      }
    _ -> panic
  }
}

fn move_boxes(map, pos: #(Int, Int), dir: #(Int, Int)) {
  let next_pos = #(pos.0 + dir.0, pos.1 + dir.1)
  case map |> dict.get(next_pos) |> result.unwrap("") {
    "." | "@" -> Ok(map |> dict.insert(next_pos, "O"))
    "#" -> Error(Nil)
    "O" -> move_boxes(map, next_pos, dir)
    _ -> panic
  }
}

fn parse(input) {
  let assert Ok(#(map, movement)) = input |> string.split_once("\n\n")
  let map =
    map
    |> helpers.parse_grid(dict.new(), fn(acc, x, y, tile) {
      acc |> dict.insert(#(x, y), tile)
    })
  let assert Ok(robot) = map |> dict.to_list |> list.find(fn(e) { e.1 == "@" })
  let robot = robot |> helpers.tuple_first
  let movement = movement |> string.replace("\n", "") |> string.to_graphemes
  #(map, robot, movement)
}
