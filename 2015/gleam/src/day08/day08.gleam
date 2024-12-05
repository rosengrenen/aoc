import gleam/int
import gleam/list
import gleam/string

import helpers

pub fn part1(input: String) -> String {
  input
  |> string.split("\n")
  |> list.map(fn(line) {
    { line |> string.length } - decoded_len(line |> string.to_graphemes)
  })
  |> helpers.sum
  |> int.to_string
}

pub fn part2(input: String) -> String {
  input
  |> string.split("\n")
  |> list.map(fn(line) {
    encoded_len(line |> string.to_graphemes) - { line |> string.length }
  })
  |> helpers.sum
  |> int.to_string
}

fn decoded_len(line) {
  case line {
    [] -> 0
    ["\""] -> 0
    ["\"", ..rest] -> 0 + decoded_len(rest)
    ["\\", "x", _, _, ..rest] -> 1 + decoded_len(rest)
    ["\\", _, ..rest] -> 1 + decoded_len(rest)
    [_, ..rest] -> 1 + decoded_len(rest)
  }
}

fn encoded_len(line) {
  case line {
    [] -> 2
    ["\\", ..rest] -> 2 + encoded_len(rest)
    ["\"", ..rest] -> 2 + encoded_len(rest)
    [_, ..rest] -> 1 + encoded_len(rest)
  }
}
