import gleam/bit_array
import gleam/crypto
import gleam/int
import gleam/result
import gleam/string
import gleam/yielder

import helpers

pub fn part1(input: String) -> String {
  input
  |> find_index_of_first_hash("00000")
  |> int.to_string
}

pub fn part2(input: String) -> String {
  input
  |> find_index_of_first_hash("000000")
  |> int.to_string
}

fn find_index_of_first_hash(input, target) {
  yielder.range(1, 1_000_000_000)
  |> yielder.map(int.to_string)
  |> yielder.map(fn(n) { input <> n })
  |> yielder.map(bit_array.from_string)
  |> yielder.map(crypto.hash(crypto.Md5, _))
  |> yielder.map(bit_array.base16_encode)
  |> yielder.index
  |> yielder.find(fn(e) { string.starts_with(e.0, target) })
  |> result.map(helpers.second)
  |> result.map(fn(i) { i + 1 })
  |> result.unwrap(0)
}
