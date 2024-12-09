import gleam/int
import gleam/list
import gleam/result
import gleam/string
import gleam/yielder

import helpers

pub fn part1(input: String) -> String {
  input
  |> parse_nums
  |> parse_fragments(0)
  |> move_fragments
  |> list.filter_map(fn(fragment) {
    case fragment {
      FFill(fill) -> Ok(fill)
      _ -> Error(Nil)
    }
  })
  |> helpers.enumerate
  |> list.map(fn(entry) {
    let #(index, id) = entry
    index * id
  })
  |> helpers.sum
  |> int.to_string
}

fn move_fragments(disk) {
  case disk {
    [] -> []
    [FEmpty, ..rest] -> {
      let #(rest, block) = remove_last_block(rest)
      case block {
        FEmpty -> move_fragments([FEmpty, ..rest])
        _ -> [block, ..move_fragments(rest)]
      }
    }
    [head, ..rest] -> {
      [head, ..move_fragments(rest)]
    }
  }
}

type Fragment {
  FFill(n: Int)
  FEmpty
}

fn parse_fragments(nums, i) {
  case nums {
    [fill] -> list.repeat(FFill(i), fill)
    [fill, empty, ..rest] ->
      [
        list.repeat(FFill(i), fill),
        list.repeat(FEmpty, empty),
        parse_fragments(rest, i + 1),
      ]
      |> list.flatten
    _ -> panic
  }
}

pub fn part2(input: String) -> String {
  let blocks = input |> parse_nums |> parse_blocks(0)
  let max_id = { blocks |> list.length } / 2
  blocks
  |> move_block_descending(max_id)
  |> list.map(fn(block) {
    case block {
      BFill(id, n) -> list.repeat(id, n)
      BEmpty(n) -> list.repeat(0, n)
    }
  })
  |> list.flatten
  |> helpers.enumerate
  |> list.map(fn(entry) {
    let #(index, id) = entry
    index * id
  })
  |> helpers.sum
  |> int.to_string
}

fn remove_last_block(disk) {
  case disk {
    [] -> panic
    [last] -> #([], last)
    [head, ..rest] -> {
      let #(rest, last) = remove_last_block(rest)
      #([head, ..rest], last)
    }
  }
}

fn move_block_descending(disk, max_id) {
  yielder.range(max_id, 0)
  |> yielder.fold(disk, fn(disk, id) {
    let #(updated_disk, fill) = remove_block_id(id, disk)
    case move_block(id, fill, updated_disk) {
      Ok(disk) -> disk
      Error(_) -> disk
    }
  })
}

fn move_block(id, fill, disk) {
  case disk {
    [] -> Error(Nil)
    [BEmpty(empty), ..rest] if empty >= fill -> {
      Ok([BFill(id, fill), BEmpty(empty - fill), ..rest])
    }
    [head, ..rest] ->
      move_block(id, fill, rest) |> result.map(fn(rest) { [head, ..rest] })
  }
}

fn remove_block_id(id, disk) {
  case disk {
    [] -> panic
    [BFill(i, count), ..rest] if i == id -> #(
      [BEmpty(count), ..rest] |> merge_empty,
      count,
    )
    [head, ..rest] -> {
      let #(rest, last) = remove_block_id(id, rest)
      #([head, ..rest], last)
    }
  }
}

fn merge_empty(disk) {
  case disk {
    [] -> []
    [BEmpty(e0), BEmpty(e1), ..rest] -> [BEmpty(e0 + e1), ..rest]
    disk -> disk
  }
}

type Block {
  BFill(i: Int, n: Int)
  BEmpty(n: Int)
}

fn parse_blocks(nums, i) {
  case nums {
    [fill] -> [BFill(i, fill)]
    [fill, empty, ..rest] -> [
      BFill(i, fill),
      BEmpty(empty),
      ..parse_blocks(rest, i + 1)
    ]
    _ -> panic
  }
}

fn parse_nums(input) {
  input
  |> string.to_graphemes
  |> list.filter_map(int.parse)
}
