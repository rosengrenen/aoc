import gleam/int
import gleam/list
import gleam/string

pub fn part1(input: String) -> String {
  input
  |> parse
  |> list.filter_map(fn(e) { calc(e.0, e.1, e.2) })
  |> int.sum
  |> int.to_string
}

pub fn part2(input: String) -> String {
  input
  |> parse
  |> list.filter_map(fn(e) {
    calc(e.0, e.1, #(10_000_000_000_000 + e.2.0, 10_000_000_000_000 + e.2.1))
  })
  |> int.sum
  |> int.to_string
}

// p0 = a0x + b0y
// p1 = a1x + b1y
// p0 - b0y = a0x
// p1 - b1y = a1x
// a1(p0 - b0y) = a0(p1 - b1y)
// a1p0 - a1b0y = a0p1 - a0b1y
// a1p0 - a0p1 = a1b0y - a0b1y
// a1p0 - a0p1 = y(a1b0 - a0b1)
// y = (a1p0 - a0p1) / (a1b0 - a0b1)
// p0 = a0x + b0y
// p0 - b0y = a0x
// x = (p0 - b0y) / a0
fn calc(a: #(Int, Int), b: #(Int, Int), p: #(Int, Int)) {
  let y = { a.1 * p.0 - a.0 * p.1 } / { a.1 * b.0 - a.0 * b.1 }
  let x = { p.0 - b.0 * y } / a.0
  case a.0 * x + b.0 * y == p.0 && a.1 * x + b.1 * y == p.1 {
    True -> Ok(3 * x + y)
    False -> Error(Nil)
  }
}

fn parse(input) {
  input
  |> string.split("\n\n")
  |> list.map(fn(part) {
    part
    |> string.split("\n")
    |> list.map(fn(line) {
      let assert Ok(#(_, line)) = line |> string.split_once(":")
      let assert Ok(#(left, right)) =
        line |> string.trim |> string.split_once(",")
      let assert Ok(left) =
        left |> string.trim |> string.slice(2, 100) |> int.parse
      let assert Ok(right) =
        right |> string.trim |> string.slice(2, 100) |> int.parse
      #(left, right)
    })
  })
  |> list.map(fn(part) {
    case part {
      [a, b, p] -> #(a, b, p)
      _ -> panic
    }
  })
}
