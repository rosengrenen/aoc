import gleam/list

pub fn sum(ints: List(Int)) {
  ints |> list.fold(0, fn(acc, x) { acc + x })
}

pub fn combinations(first: List(a), second: List(a)) {
  first
  |> list.map(fn(el_first) {
    second
    |> list.map(fn(el_second) { #(el_first, el_second) })
  })
  |> list.flatten
}
