import gleam/dict
import gleam/int
import gleam/list
import gleam/order
import gleam/result
import gleam/set
import gleam/string

import helpers

pub fn part1(input: String) -> String {
  let #(orders, updates) = input |> parse
  updates
  |> list.filter(update_correctly_ordered(orders, _))
  |> list.map(fn(update) {
    update |> helpers.list_nth(list.length(update) / 2, 0)
  })
  |> helpers.sum
  |> int.to_string
}

pub fn part2(input: String) -> String {
  let #(orders, updates) = input |> parse
  updates
  |> list.filter(fn(update) { !update_correctly_ordered(orders, update) })
  |> list.map(fn(update) {
    list.sort(update, fn(l, r) {
      case correctly_ordered(l, r, orders) {
        True -> order.Lt
        False -> order.Gt
      }
    })
  })
  |> list.map(fn(update) {
    update |> helpers.list_nth(list.length(update) / 2, 0)
  })
  |> helpers.sum
  |> int.to_string
}

fn update_correctly_ordered(orders, update) {
  update
  |> list.window_by_2
  |> list.all(fn(e) { correctly_ordered(e.0, e.1, orders) })
}

fn correctly_ordered(left, right, orders) {
  orders |> dict.get(left) |> result.unwrap(set.new()) |> set.contains(right)
}

fn parse(input: String) {
  let assert Ok(#(order, updates)) = input |> string.split_once("\n\n")
  let order =
    order
    |> string.split("\n")
    |> list.fold(dict.new(), fn(acc, line) {
      let assert Ok(#(before, after)) = string.split_once(line, "|")
      let assert Ok(before) = int.parse(before)
      let assert Ok(after) = int.parse(after)
      case acc |> dict.get(before) {
        Ok(afters) -> acc |> dict.insert(before, afters |> set.insert(after))
        Error(_) -> acc |> dict.insert(before, set.new() |> set.insert(after))
      }
    })
  let updates =
    updates
    |> string.split("\n")
    |> list.map(fn(line) {
      line |> string.split(",") |> list.filter_map(int.parse)
    })
  #(order, updates)
}
