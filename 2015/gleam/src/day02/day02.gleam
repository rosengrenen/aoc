import gleam/int
import gleam/list
import gleam/string

import helpers

pub fn part1(input: String) -> String {
  input
  |> parse
  |> list.map(fn(nums) { paper(nums.0, nums.1, nums.2) })
  |> helpers.sum
  |> int.to_string
}

pub fn part2(input: String) -> String {
  input
  |> parse
  |> list.map(fn(nums) { ribbon(nums.0, nums.1, nums.2) })
  |> helpers.sum
  |> int.to_string
}

fn paper(l, w, h) {
  2 * l * w + 2 * w * h + 2 * h * l + helpers.min([l * w, w * h, h * l])
}

fn ribbon(l, w, h) {
  [l, w, h]
  |> list.sort(int.compare)
  |> list.take(2)
  |> list.map(fn(v) { v * 2 })
  |> helpers.sum
  |> fn(around) { around + l * w * h }
}

fn parse(input) {
  input
  |> string.split("\n")
  |> list.map(fn(line) {
    line
    |> string.split("x")
    |> list.filter_map(int.parse)
  })
  |> list.map(fn(nums) {
    case nums {
      [l, w, h] -> #(l, w, h)
      _ -> panic
    }
  })
}
