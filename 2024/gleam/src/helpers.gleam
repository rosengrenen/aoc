import gleam/list
import gleam/result
import gleam/string

pub fn combinations(first, second) {
  first
  |> list.map(fn(el_first) {
    second
    |> list.map(fn(el_second) { #(el_first, el_second) })
  })
  |> list.flatten
}

pub fn enumerate(input) {
  input |> list.index_map(fn(v, i) { #(i, v) })
}

pub fn list_nth(input, index, default) {
  input |> enumerate |> list.key_find(index) |> result.unwrap(default)
}

pub fn tuple_first(input: #(a, b)) {
  input.0
}

pub fn tuple_second(input: #(a, b)) {
  input.1
}

pub fn parse_grid(input, acc, f) {
  input
  |> string.split("\n")
  |> enumerate
  |> list.fold(acc, fn(acc, line) {
    let #(y, line) = line
    line
    |> string.to_graphemes
    |> enumerate
    |> list.fold(acc, fn(acc, tile) {
      let #(x, tile) = tile
      f(acc, x, y, tile)
    })
  })
}
