import gleam/int
import gleam/list
import gleam/result
import gleam/string

import helpers

pub fn part1(input: String) -> String {
  input
  |> parse_muls
  |> helpers.sum
  |> int.to_string
}

pub fn part2(input: String) -> String {
  input
  |> string.split("do()")
  |> list.map(fn(part) {
    part
    |> string.split_once("don't()")
    |> result.map(fn(parts) { parts.0 })
    |> result.unwrap(part)
  })
  |> list.flat_map(parse_muls)
  |> helpers.sum
  |> int.to_string
}

fn parse_muls(input: String) {
  input
  |> string.split("mul(")
  |> list.filter_map(fn(part) { string.split_once(part, ")") })
  |> list.map(fn(parts) { parts.0 })
  |> list.filter_map(fn(args) { string.split_once(args, ",") })
  |> list.map(fn(args) {
    result.unwrap(int.parse(args.0), 0) * result.unwrap(int.parse(args.1), 0)
  })
}
