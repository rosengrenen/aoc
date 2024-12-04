import gleam/int
import gleam/list
import gleam/result

pub fn sum(nums) {
  nums
  |> list.reduce(fn(acc, x) { acc + x })
  |> result.unwrap(0)
}

pub fn min(nums) {
  nums
  |> list.reduce(fn(acc, x) { int.min(acc, x) })
  |> result.unwrap(0)
}

pub fn enumerate(input) {
  input |> list.index_map(fn(v, i) { #(i, v) })
}

pub fn index_filter(input, pred) {
  input
  |> enumerate
  |> list.filter(fn(e) { pred(e.0, e.1) })
  |> list.map(second)
}

pub fn first(input: #(a, b)) {
  input.0
}

pub fn second(input: #(a, b)) {
  input.1
}
