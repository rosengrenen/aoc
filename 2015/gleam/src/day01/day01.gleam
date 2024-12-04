import gleam/int
import gleam/list
import gleam/string

pub fn part1(input: String) -> String {
  input
  |> string.to_graphemes
  |> list.fold(0, fn(acc, char) {
    case char {
      "(" -> acc + 1
      ")" -> acc - 1
      _ -> panic
    }
  })
  |> int.to_string
}

pub fn part2(input: String) -> String {
  input
  |> string.to_graphemes
  |> list.fold_until(#(0, 0), fn(acc, char) {
    case acc.1 < 0 {
      True -> list.Stop(acc)
      False ->
        case char {
          "(" -> list.Continue(#(acc.0 + 1, acc.1 + 1))
          ")" -> list.Continue(#(acc.0 + 1, acc.1 - 1))
          _ -> panic
        }
    }
  })
  |> fn(acc) { acc.0 }
  |> int.to_string
}
