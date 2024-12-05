import gleam/bool
import gleam/dict
import gleam/int
import gleam/list
import gleam/set
import gleam/string

import helpers

pub fn part1(input: String) -> String {
  let bad_strings = ["ab", "cd", "pq", "xy"]
  input
  |> string.split("\n")
  |> list.filter(fn(line) {
    line |> string.to_graphemes |> list.filter(is_vowel) |> list.length >= 3
  })
  |> list.filter(fn(line) {
    bad_strings |> list.filter(string.contains(line, _)) |> list.is_empty
  })
  |> list.filter(contains_double_char)
  |> list.length
  |> int.to_string
}

pub fn part2(input: String) -> String {
  input
  |> string.split("\n")
  |> list.filter(fn(line) { line |> contains_duplicate_pair })
  |> list.filter(fn(line) {
    line
    |> string.to_graphemes
    |> list.window(3)
    |> list.filter(fn(triple) {
      case triple {
        [c0, _, c2] -> c0 == c2
        _ -> panic
      }
    })
    |> list.is_empty
    |> bool.negate
  })
  |> list.length
  |> int.to_string
}

fn is_vowel(char) {
  string.contains("aeiou", char)
}

fn contains_double_char(input) {
  input
  |> string.to_graphemes
  |> list.fold_until(#("", False), fn(acc, char) {
    case acc {
      #(prev_char, _) if prev_char == char -> #(char, True) |> list.Stop
      #(_, _) -> #(char, False) |> list.Continue
    }
  })
  |> helpers.second
}

fn contains_duplicate_pair(line) {
  line
  |> string.to_graphemes
  |> list.window_by_2
  |> list.map(fn(tuple) { tuple.0 <> tuple.1 })
  |> helpers.enumerate
  |> list.fold(dict.new(), fn(acc, cur) {
    let #(index, pair) = cur
    case acc |> dict.get(pair) {
      Ok(indices) -> acc |> dict.insert(pair, indices |> set.insert(index))
      Error(_) -> acc |> dict.insert(pair, set.new() |> set.insert(index))
    }
  })
  |> dict.to_list
  |> list.map(fn(tuple) { tuple.1 |> set.to_list })
  |> list.filter(fn(indices) {
    { indices |> helpers.max } - { indices |> helpers.min } >= 2
  })
  |> list.is_empty
  |> bool.negate
}
