import gleam/int
import gleam/list
import gleam/string

import helpers

pub fn part1(input: String) -> String {
  input
  |> parse
  |> list.filter(run(_, combinations))
  |> list.map(helpers.tuple_first)
  |> helpers.sum
  |> int.to_string
}

pub fn part2(input: String) -> String {
  input
  |> parse
  |> list.filter(run(_, combinations2))
  |> list.map(helpers.tuple_first)
  |> helpers.sum
  |> int.to_string
}

fn run(entry: #(Int, List(Int)), f) {
  case entry.1 {
    [] -> False
    [acc, ..rest] -> f(entry.0, acc, rest)
  }
}

fn combinations(answer, acc, inputs) {
  case inputs {
    [] -> acc == answer
    [next, ..rest] ->
      acc <= answer
      && {
        combinations(answer, acc * next, rest)
        || combinations(answer, acc + next, rest)
      }
  }
}

fn combinations2(answer, acc, inputs) {
  case inputs {
    [] -> acc == answer
    [next, ..rest] ->
      acc <= answer
      && {
        combinations2(answer, acc * next, rest)
        || combinations2(answer, acc + next, rest)
        || combinations2(answer, concat_ints(acc, next), rest)
      }
  }
}

fn concat_ints(left, right) {
  let string = left |> int.to_string <> right |> int.to_string
  let assert Ok(joined) = string |> int.parse
  joined
}

pub fn parse(input) {
  input
  |> string.split("\n")
  |> list.map(fn(line) {
    let assert Ok(#(answer, inputs)) = line |> string.split_once(":")
    let assert Ok(answer) = answer |> string.trim |> int.parse
    #(
      answer,
      inputs |> string.trim |> string.split(" ") |> list.filter_map(int.parse),
    )
  })
}
