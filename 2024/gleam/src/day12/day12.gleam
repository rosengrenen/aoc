import gleam/dict
import gleam/int
import gleam/list
import gleam/set

import helpers

pub fn part1(input: String) -> String {
  input
  |> parse
  |> groups
  |> list.map(fn(group) { area(group) * { perimeter(group) |> set.size } })
  |> int.sum
  |> int.to_string
}

pub fn part2(input: String) -> String {
  input
  |> parse
  |> groups
  |> list.map(fn(group) { area(group) * sides(group) })
  |> int.sum
  |> int.to_string
}

fn area(group) {
  group |> set.size
}

fn perimeter(group) {
  group
  |> set.fold(set.new(), fn(acc, k) {
    let #(x, y) = k
    helpers.adj4
    |> list.zip([0, 1, 2, 3])
    |> list.map(fn(adj_with_dir) {
      let #(#(dx, dy), dir) = adj_with_dir
      #(#(x + dx, y + dy), dir)
    })
    |> list.fold(acc, fn(acc, cur) {
      let #(pos, _) = cur
      case group |> set.contains(pos) {
        True -> acc
        False -> acc |> set.insert(cur)
      }
    })
  })
}

fn sides(group) {
  group
  |> perimeter
  |> fence_sides
}

fn fence_sides(group) {
  case group |> set.is_empty {
    True -> 0
    False -> {
      let assert Ok(pos) = group |> set.to_list |> list.first
      let group = fence_side(group, pos)
      1 + fence_sides(group)
    }
  }
}

fn fence_side(group, pos) {
  case group |> set.contains(pos) {
    False -> group
    True -> {
      let group = group |> set.delete(pos)
      let #(#(x, y), dir) = pos
      helpers.adj4
      |> list.map(fn(d) { #(#(x + d.0, y + d.1), dir) })
      |> list.fold(group, fn(acc, pos) {
        case group |> set.contains(pos) {
          True -> fence_side(acc, pos)
          _ -> acc
        }
      })
    }
  }
}

fn groups(grid) {
  case grid |> dict.is_empty {
    True -> []
    False -> {
      let assert Ok(pos) = grid |> dict.keys |> list.first
      let #(grid, g) = group(grid, set.new(), pos)
      [g, ..groups(grid)]
    }
  }
}

fn group(grid, area, pos) {
  case grid |> dict.get(pos) {
    Error(_) -> #(grid, area)
    Ok(v) -> {
      let grid = grid |> dict.delete(pos)
      let area = area |> set.insert(pos)
      let #(x, y) = pos
      helpers.adj4
      |> list.map(fn(d) { #(x + d.0, y + d.1) })
      |> list.fold(#(grid, area), fn(acc, pos) {
        case grid |> dict.get(pos) {
          Ok(nv) if nv == v -> {
            let #(grid, area) = acc
            group(grid, area, pos)
          }
          _ -> acc
        }
      })
    }
  }
}

fn parse(input) {
  input
  |> helpers.parse_grid(dict.new(), fn(grid, x, y, tile) {
    grid |> dict.insert(#(x, y), tile)
  })
}
