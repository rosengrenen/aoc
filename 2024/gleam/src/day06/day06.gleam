import gleam/int
import gleam/list
import gleam/set
import gleam/string

import helpers

pub fn part1(input: String) -> String {
  let #(obstacles, pos, max_x, max_y) = input |> parse
  traverse(set.new(), obstacles, North, pos, #(max_x, max_y))
  |> set.size
  |> int.to_string
}

pub fn part2(input: String) -> String {
  let #(obstacles, pos, max_x, max_y) = input |> parse
  traverse2(set.new(), obstacles, North, pos, #(max_x, max_y))
  |> set.filter(fn(new_obstacle) {
    loops(set.new(), obstacles |> set.insert(new_obstacle), North, pos, #(
      max_x,
      max_y,
    ))
  })
  |> set.size
  |> int.to_string
}

fn traverse(visited, obstacles, dir, pos, max) {
  let #(max_x, max_y) = max
  let #(x, y) = pos
  case x < 0 || y < 0 || x > max_x || y > max_y {
    True -> visited
    False -> {
      let visited = visited |> set.insert(pos)
      case obstacles |> set.contains(step(dir, pos)) {
        False -> traverse(visited, obstacles, dir, step(dir, pos), max)
        True -> {
          traverse(visited, obstacles, rotate(dir), pos, max)
        }
      }
    }
  }
}

fn traverse2(visited, obstacles, dir, pos, max) {
  let #(max_x, max_y) = max
  let #(x, y) = pos
  case x < 0 || y < 0 || x > max_x || y > max_y {
    True -> visited
    False -> {
      let visited = case obstacles |> set.contains(step(dir, pos)) {
        False -> visited |> set.insert(step(dir, pos))
        True -> visited
      }
      case obstacles |> set.contains(step(dir, pos)) {
        False -> traverse(visited, obstacles, dir, step(dir, pos), max)
        True -> traverse(visited, obstacles, rotate(dir), pos, max)
      }
    }
  }
}

fn loops(visited, obstacles, dir, pos, max) {
  let #(max_x, max_y) = max
  let #(x, y) = pos
  case x < 0 || y < 0 || x > max_x || y > max_y {
    True -> False
    False -> {
      case visited |> set.contains(#(dir, pos)) {
        True -> True
        False -> {
          let visited = visited |> set.insert(#(dir, pos))
          case obstacles |> set.contains(step(dir, pos)) {
            True -> loops(visited, obstacles, rotate(dir), pos, max)
            False -> loops(visited, obstacles, dir, step(dir, pos), max)
          }
        }
      }
    }
  }
}

type Direction {
  North
  East
  South
  West
}

fn rotate(dir) {
  case dir {
    North -> East
    East -> South
    South -> West
    West -> North
  }
}

fn step(dir, pos) {
  let #(x, y) = pos
  case dir {
    North -> #(x, y - 1)
    East -> #(x + 1, y)
    South -> #(x, y + 1)
    West -> #(x - 1, y)
  }
}

fn parse(input) {
  input
  |> string.split("\n")
  |> helpers.enumerate
  |> list.fold(#(set.new(), #(0, 0), 0, 0), fn(grid, row) {
    let #(y, row) = row
    row
    |> string.to_graphemes
    |> helpers.enumerate
    |> list.fold(grid, fn(grid, tile) {
      let #(x, tile) = tile
      case tile {
        "^" -> #(grid.0, #(x, y), int.max(grid.2, x), int.max(grid.3, y))
        "#" -> #(
          grid.0 |> set.insert(#(x, y)),
          grid.1,
          int.max(grid.2, x),
          int.max(grid.3, y),
        )
        _ -> #(grid.0, grid.1, int.max(grid.2, x), int.max(grid.3, y))
      }
    })
  })
}
