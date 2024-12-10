import gleam/int
import gleam/set

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
  let new_pos = step(dir, pos)
  let new_dir = rotate(dir)

  case x < 0 || y < 0 || x > max_x || y > max_y {
    True -> 0
    False -> {
      let new_visited = visited |> set.insert(#(dir, pos))
      let good_obstacles = case obstacles |> set.contains(new_pos) {
        False -> traverse2(new_visited, obstacles, dir, new_pos, max)
        True -> traverse2(new_visited, obstacles, new_dir, pos, max)
      }
      case
        obstacles |> set.contains(new_pos)
        || visited |> contains_pos(new_pos)
        || !loops(visited, obstacles |> set.insert(new_pos), dir, pos, max)
      {
        True -> 0 + good_obstacles
        False -> 1 + good_obstacles
      }
    }
  }
}

fn contains_pos(visited, pos) {
  visited |> set.contains(#(North, pos))
  || visited |> set.contains(#(East, pos))
  || visited |> set.contains(#(South, pos))
  || visited |> set.contains(#(West, pos))
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
          let #(new_dir, new_pos) = case
            obstacles |> set.contains(step(dir, pos))
          {
            True -> #(rotate(dir), pos)
            False -> #(dir, step(dir, pos))
          }

          loops(visited, obstacles, new_dir, new_pos, max)
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
  |> helpers.parse_grid(#(set.new(), #(0, 0), 0, 0), fn(grid, x, y, tile) {
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
}
