import gleam/dict
import gleam/int
import gleam/list
import gleam/string

import helpers

pub fn part1(input: String) -> String {
  input
  |> parse
  |> blinks(25)
  |> int.to_string
}

pub fn part2(input: String) -> String {
  input
  |> parse
  |> blinks(75)
  |> int.to_string
}

fn blinks(stones, blinks) {
  stones
  |> list.fold(#(dict.new(), 0), fn(acc, stone) {
    let #(cache, sum) = acc
    let #(cache, n) = blink(cache, stone, blinks)
    #(cache, sum + n)
  })
  |> helpers.tuple_second
}

fn blink(cache, stone, blinks) {
  case cache |> dict.get(#(blinks, stone)) {
    Ok(num_stones) -> #(cache, num_stones)
    _ -> {
      case blinks {
        0 -> #(cache, 1)
        _ -> {
          let #(cache, num_stones) = case stone {
            0 -> blink(cache, 1, blinks - 1)
            stone -> {
              case stone_is_even(stone) {
                True -> {
                  let #(stone1, stone2) = split_stone(stone)
                  let #(cache, num_stones1) = blink(cache, stone1, blinks - 1)
                  let #(cache, num_stones2) = blink(cache, stone2, blinks - 1)
                  let num_stones = num_stones1 + num_stones2
                  #(cache, num_stones)
                }
                False -> blink(cache, stone * 2024, blinks - 1)
              }
            }
          }
          #(cache |> dict.insert(#(blinks, stone), num_stones), num_stones)
        }
      }
    }
  }
}

fn stone_is_even(stone) {
  { stone |> int.to_string |> string.length } % 2 == 0
}

fn split_stone(stone) {
  let stone_string = stone |> int.to_string
  let half_len = { stone_string |> string.length } / 2
  let stone1_string = stone_string |> string.slice(0, half_len)
  let stone2_string = stone_string |> string.slice(half_len, half_len)
  let assert Ok(stone1) = stone1_string |> int.parse
  let assert Ok(stone2) = stone2_string |> int.parse
  #(stone1, stone2)
}

fn parse(input) {
  input
  |> string.split(" ")
  |> list.filter_map(int.parse)
}
