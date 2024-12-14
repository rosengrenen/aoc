import gleam/dict
import gleam/int
import gleam/list
import gleam/option
import gleam/result
import gleam/set
import gleam/string
import gleam/yielder
import helpers

const max_x = 101

const max_y = 103

pub fn part1(input: String) -> String {
  let robots = input |> parse
  let positions =
    robots
    |> list.fold(dict.new(), fn(acc, robot) {
      let #(#(px, py), #(vx, vy)) = robot
      let end_x = mod(px + vx * 100, max_x)
      let end_y = mod(py + vy * 100, max_y)
      acc
      |> dict.upsert(#(end_x, end_y), fn(c) { { c |> option.unwrap(0) } + 1 })
    })
  let low_x = yielder.range(0, max_x / 2 - 1)
  let high_x = yielder.range(max_x / 2 + 1, max_x - 1)
  let low_y = yielder.range(0, max_y / 2 - 1)
  let high_y = yielder.range(max_y / 2 + 1, max_y - 1)
  [#(low_x, low_y), #(low_x, high_y), #(high_x, low_y), #(high_x, high_y)]
  |> list.map(fn(ranges) {
    let #(xs, ys) = ranges
    xs
    |> yielder.fold(0, fn(acc, x) {
      ys
      |> yielder.fold(acc, fn(acc, y) {
        acc + { positions |> dict.get(#(x, y)) |> result.unwrap(0) }
      })
    })
  })
  |> int.product
  |> int.to_string
}

pub fn part2(input: String) -> String {
  let robots = input |> parse
  let samples = 100
  let var_sample =
    yielder.range(1, samples)
    |> yielder.map(fn(seconds) {
      let positions = positions_after_seconds(robots, seconds)
      let var_x = positions |> list.map(helpers.tuple_first) |> variance
      let var_y = positions |> list.map(helpers.tuple_second) |> variance
      #(var_x, var_y)
    })
    |> yielder.to_list
  let avg_var_x =
    { var_sample |> list.map(helpers.tuple_first) |> int.sum } / samples
  let avg_var_y =
    { var_sample |> list.map(helpers.tuple_second) |> int.sum } / samples
  let assert Ok(#(seconds, _)) =
    yielder.range(1, 100_000_000)
    |> yielder.map(fn(seconds) {
      #(seconds, positions_after_seconds(robots, seconds))
    })
    |> yielder.find(fn(e) {
      let #(_, positions) = e
      let var_x = positions |> list.map(helpers.tuple_first) |> variance
      let var_y = positions |> list.map(helpers.tuple_second) |> variance
      var_x < avg_var_x / 2 && var_y < avg_var_y / 2
    })
  seconds |> int.to_string
}

fn mod(x, n) {
  let r = x % n
  case r < 0 {
    True -> r + n
    False -> r
  }
}

fn positions_after_seconds(robots, seconds) {
  robots
  |> list.fold(set.new(), fn(acc, robot) {
    let #(#(px, py), #(vx, vy)) = robot
    let end_x = mod(px + vx * seconds, max_x)
    let end_y = mod(py + vy * seconds, max_y)
    acc
    |> set.insert(#(end_x, end_y))
  })
  |> set.to_list
}

fn variance(values) {
  let avg = { values |> int.sum } / { values |> list.length }
  {
    values
    |> list.map(fn(value) {
      let diff = value - avg
      diff * diff
    })
    |> int.sum
  }
  / { values |> list.length }
}

fn parse(input) {
  input
  |> string.split("\n")
  |> list.map(fn(line) {
    let assert Ok(#(pos, vel)) = line |> string.split_once(" ")
    let assert Ok(#(pos_x, pos_y)) =
      pos |> string.drop_start(2) |> string.split_once(",")
    let assert Ok(#(vel_x, vel_y)) =
      vel |> string.drop_start(2) |> string.split_once(",")
    let assert Ok(pos_x) = pos_x |> int.parse
    let assert Ok(pos_y) = pos_y |> int.parse
    let assert Ok(vel_x) = vel_x |> int.parse
    let assert Ok(vel_y) = vel_y |> int.parse
    #(#(pos_x, pos_y), #(vel_x, vel_y))
  })
}
