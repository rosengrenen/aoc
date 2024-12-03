import gleam/list

pub fn sum(ints: List(Int)) {
  ints |> list.fold(0, fn(acc, x) { acc + x })
}
