import gleam/dict
import gleam/int
import gleam/list
import gleam/option.{None, Some}
import gleam/result
import gleam/set
import gleam/yielder

import helpers.{adj4, east, north, south, west}

pub fn part1(input: String) -> String {
  let pipes = input |> parse
  let assert Ok(start) =
    pipes
    |> dict.filter(fn(_, v) { v == Start })
    |> dict.to_list
    |> list.first
    |> result.map(helpers.tuple_first)
  pipes
  |> pipe_loop(start, set.new())
  |> loop_distance_from_start(start, dict.new() |> dict.insert(start, 0))
  |> dict.values
  |> list.fold(0, fn(acc, cur) { int.max(acc, cur) })
  |> int.to_string
}

pub fn part2(input: String) -> String {
  let pipes = input |> parse
  let assert Ok(start) =
    pipes
    |> dict.filter(fn(_, v) { v == Start })
    |> dict.to_list
    |> list.first
    |> result.map(helpers.tuple_first)
  let loop_connections = pipes |> pipe_loop(start, set.new())
  let loop_pipes = loop_connections |> set.map(helpers.tuple_first)
  let pipes =
    case
      adj4
      |> list.map(fn(dir) { #(start.0 + dir.0, start.1 + dir.1) })
      |> list.map(fn(neighbour_pos) {
        loop_connections |> set.contains(#(start, neighbour_pos))
      })
    {
      [True, _, True, _] -> NorthSouth
      [True, True, _, _] -> NorthEast
      [True, _, _, True] -> NorthWest
      [_, True, _, True] -> EastWest
      [_, True, True, _] -> SouthEast
      [_, _, True, True] -> SouthWest
      _ -> panic
    }
    |> dict.insert(pipes, start, _)
  pipes
  |> dict.filter(fn(pos, _) { loop_pipes |> set.contains(pos) })
  |> num_enclosed
  |> int.to_string
}

fn num_enclosed(loop) {
  yielder.range(0, 1000)
  |> yielder.fold(0, fn(acc, y) {
    yielder.range(0, 1000)
    |> yielder.fold(#(False, acc, None), fn(acc, x) {
      let #(inside, acc, prev) = acc
      case inside, loop |> dict.get(#(x, y)) {
        True, Error(_) -> {
          #(True, acc + 1, None)
        }
        inside, Ok(pipe) -> {
          case pipe, prev {
            NorthSouth, _ -> #(!inside, acc, None)
            NorthWest, Some(SouthEast) | SouthWest, Some(NorthEast) -> #(
              !inside,
              acc,
              None,
            )
            NorthWest, Some(NorthEast) | SouthWest, Some(SouthEast) -> #(
              inside,
              acc,
              None,
            )
            NorthEast, None | SouthEast, None -> #(inside, acc, Some(pipe))
            _, _ -> #(inside, acc, prev)
          }
        }
        inside, _ -> #(inside, acc, prev)
      }
    })
    |> fn(t) { t.1 }
  })
}

fn loop_distance_from_start(loop, pos: #(Int, Int), distances) {
  let assert Ok(distance) = distances |> dict.get(pos)
  adj4
  |> list.map(fn(dir) { #(pos.0 + dir.0, pos.1 + dir.1) })
  |> list.filter(fn(neighbour_pos) {
    loop |> set.contains(#(pos, neighbour_pos))
  })
  |> list.fold(distances, fn(distances, neighbour_pos) {
    case distances |> dict.get(neighbour_pos) {
      Ok(old_distance) if old_distance > distance + 1 -> {
        loop
        |> loop_distance_from_start(
          neighbour_pos,
          distances |> dict.insert(neighbour_pos, distance + 1),
        )
      }
      Error(_) ->
        loop
        |> loop_distance_from_start(
          neighbour_pos,
          distances |> dict.insert(neighbour_pos, distance + 1),
        )
      _ -> distances
    }
  })
}

fn pipe_loop(pipes, pos: #(Int, Int), connections) {
  let assert Ok(pipe) = pipes |> dict.get(pos)
  pipe_connections(pipe, pos)
  |> set.filter(fn(neighbour_pos) {
    case pipes |> dict.get(neighbour_pos) {
      Ok(neighbour_pipe) -> {
        let already_is_connected =
          connections |> set.contains(#(pos, neighbour_pos))
        let can_connect =
          pipe_connections(neighbour_pipe, neighbour_pos) |> set.contains(pos)
        !already_is_connected && can_connect
      }
      _ -> False
    }
  })
  |> set.fold(connections, fn(connections, neighbour_pos) {
    case connections |> set.contains(#(pos, neighbour_pos)) {
      True -> connections
      False -> {
        let connections = connections |> set.insert(#(pos, neighbour_pos))
        let connections = connections |> set.insert(#(neighbour_pos, pos))
        pipe_loop(pipes, neighbour_pos, connections)
      }
    }
  })
}

fn pipe_connections(pipe, pos: #(Int, Int)) {
  let dirs = case pipe {
    NorthSouth -> [north, south]
    EastWest -> [east, west]
    NorthEast -> [north, east]
    NorthWest -> [north, west]
    SouthEast -> [south, east]
    SouthWest -> [south, west]
    Start -> adj4
  }
  dirs |> list.map(fn(dir) { #(pos.0 + dir.0, pos.1 + dir.1) }) |> set.from_list
}

type Tile {
  NorthSouth
  EastWest
  NorthEast
  NorthWest
  SouthEast
  SouthWest
  Start
}

fn parse(input) {
  input
  |> helpers.parse_grid(dict.new(), fn(acc, x, y, tile) {
    let pos = #(x, y)
    case tile {
      "|" -> acc |> dict.insert(pos, NorthSouth)
      "-" -> acc |> dict.insert(pos, EastWest)
      "L" -> acc |> dict.insert(pos, NorthEast)
      "J" -> acc |> dict.insert(pos, NorthWest)
      "7" -> acc |> dict.insert(pos, SouthWest)
      "F" -> acc |> dict.insert(pos, SouthEast)
      "S" -> acc |> dict.insert(pos, Start)
      _ -> acc
    }
  })
}
