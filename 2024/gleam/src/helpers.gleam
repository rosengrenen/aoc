import gleam/int
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

pub const adj4 = [#(0, -1), #(1, 0), #(0, 1), #(-1, 0)]

pub const adj8 = [
  #(0, -1), #(1, -1), #(1, 0), #(1, 1), #(0, 1), #(-1, 1), #(-1, 0), #(-1, -1),
]

fn euclid_inner(r0, r1, s0, s1, t0, t1) {
  let q = r0 / r1
  let rem = r0 - q * r1
  let s2 = s0 - q * s1
  let t2 = t0 - q * t1
  case rem {
    0 -> #(q, s0 - q * s1, t0 - q * t1)
    _ -> euclid_inner(r1, rem, s1, s2, t1, t2)
  }
}

pub fn euclid(r0, r1) {
  euclid_inner(int.max(r0, r1), int.min(r0, r1), 1, 0, 0, 1)
}

pub fn gcd(r0, r1) {
  euclid(int.max(r0, r1), int.min(r0, r1)).0
}

pub fn lcm(a, b) {
  a * b / gcd(a, b)
}
