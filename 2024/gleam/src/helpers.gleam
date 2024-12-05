import gleam/list
import gleam/result

pub fn sum(ints) {
  ints |> list.fold(0, fn(acc, x) { acc + x })
}

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
