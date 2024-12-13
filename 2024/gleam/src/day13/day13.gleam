import gleam/dict
import gleam/int
import gleam/list
import gleam/option
import gleam/order
import gleam/result
import gleam/set
import gleam/string
import gleam/yielder
import pprint.{debug}

import helpers

pub fn part1(input: String) -> String {
  // input
  // |> parse
  // |> list.map(fn(e) { calc(e.0, e.1, e.2) |> debug })
  // |> int.sum
  // |> int.to_string
  ""
}

pub fn part2(input: String) -> String {
  input
  |> parse
  |> list.map(fn(e) {
    calc(e.0, e.1, #(10_000_000_000_000 + e.2.0, 10_000_000_000_000 + e.2.1))
    // |> debug
  })
  |> int.sum
  |> int.to_string
}

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

fn gcd(r0, r1) {
  euclid_inner(int.max(r0, r1), int.min(r0, r1), 1, 0, 0, 1).0
}

// crt

// Steg 1: 
// Button A: X+94, Y+34
// Button B: X+22, Y+67
// Prize: X=8400, Y=5400
fn calc(a: #(Int, Int), b: #(Int, Int), p: #(Int, Int)) {
  #(a, b, p) |> debug
  let c = gcd(gcd(a.0, a.1), gcd(b.0, b.1)) |> debug
  let a = #(a.0 / c, a.1 / c)
  let b = #(b.0 / c, b.1 / c)
  let p = #(p.0 / c, p.1 / c)
  #(a, b, p) |> debug
  let xxa =
    yielder.range(0, 10_000_000_000_000)
    |> yielder.filter_map(fn(xa) {
      let rem = p.0 - xa * a.0
      case int.compare(rem, 0) {
        order.Lt -> Error(Nil)
        _ ->
          case rem % b.0 == 0 {
            True -> Ok(#(xa, rem / b.0))
            False -> Error(Nil)
          }
      }
    })
    // |> yielder.map(fn(x) { #(x.0 * a.0 + x.1 * b.0) })
    |> yielder.to_list
    |> set.from_list
  // |> debug
  let xxb =
    yielder.range(0, 10_000_000_000_000)
    |> yielder.filter_map(fn(xb) {
      let rem = p.1 - xb * a.1
      case int.compare(rem, 0) {
        order.Lt -> Error(Nil)
        _ ->
          case rem % b.1 == 0 {
            True -> Ok(#(xb, rem / b.1))
            False -> Error(Nil)
          }
      }
    })
    // |> yielder.map(fn(x) { #(x.0 * a.1 + x.1 * b.1) })
    |> yielder.to_list
    |> set.from_list
  // |> debug
  xxa
  |> set.intersection(xxb)
  // |> debug
  // |> set.map(fn(i) {
  //   let x = i.0 * a.0 + i.1 * b.0
  //   let y = i.0 * a.1 + i.1 * b.1
  //   #(x, y)
  // })
  // |> debug
  |> set.to_list
  |> list.map(fn(a) { a.0 * 3 + a.1 })
  |> list.sort(int.compare)
  |> list.first
  |> result.unwrap(0)
  // panic
  // let c = #(ca * a.0 + cb * b.0, ca * a.1 + cb * b.1)
  // #(ca, cb) |> debug
  // c |> debug
  // case int.compare(c.0, p.0), int.compare(c.1, p.1) {
  //   order.Eq, order.Eq -> Ok(ca * 3 + cb + 1)
  //   order.Gt, _ | _, order.Gt -> Error(Nil)
  //   _, _ -> {
  //     case calc(a, b, p, ca + 1, cb) {
  //       Ok(r1) ->
  //         case calc(a, b, p, ca, cb + 1) {
  //           Ok(r2) -> Ok(int.min(r1, r2))
  //           _ -> Ok(r1)
  //         }
  //       _ -> calc(a, b, p, ca, cb + 1)
  //     }
  //   }
  // }
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
