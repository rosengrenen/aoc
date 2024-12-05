import gleam/int
import gleam/list
import gleam/order
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

fn correctly_ordered(from, to, orders) {
  orders |> set.contains(#(from, to))
}

fn parse(input: String) {
  let assert Ok(#(order, updates)) = input |> string.split_once("\n\n")
  let order =
    order
    |> string.split("\n")
    |> list.fold(set.new(), fn(acc, line) {
      let assert Ok(#(from, to)) = string.split_once(line, "|")
      let assert Ok(from) = int.parse(from)
      let assert Ok(to) = int.parse(to)
      acc |> set.insert(#(from, to))
    })
  let updates =
    updates
    |> string.split("\n")
    |> list.map(fn(line) {
      line |> string.split(",") |> list.filter_map(int.parse)
    })
  #(order, updates)
}
