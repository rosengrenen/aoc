import gleam/dict
import gleam/int
import gleam/list
import gleam/result
import gleam/string

import helpers

pub fn part1(input: String) -> String {
  input
  |> parse_circuit
  |> eval(dict.new(), _, "a")
  |> helpers.first
  |> int.to_string
}

pub fn part2(input: String) -> String {
  let circuit = input |> parse_circuit
  let a_val =
    circuit
    |> eval(dict.new(), _, "a")
    |> helpers.first
    |> int.to_string
  circuit
  |> dict.insert("b", Value(a_val))
  |> eval(dict.new(), _, "a")
  |> helpers.first
  |> int.to_string
}

fn eval(cache, circuit, variable) {
  let assert Ok(op) = circuit |> dict.get(variable)
  case op {
    And(left, right) ->
      eval_bin_op(cache, circuit, int.bitwise_and, left, right)
    Or(left, right) -> eval_bin_op(cache, circuit, int.bitwise_or, left, right)
    RShift(left, right) ->
      eval_bin_op(cache, circuit, int.bitwise_shift_right, left, right)
    LShift(left, right) ->
      eval_bin_op(cache, circuit, int.bitwise_shift_left, left, right)
    Not(operand) -> {
      let #(val, cache) = eval_cache(cache, circuit, operand)
      #(int.bitwise_not(val), cache)
    }
    Value(value) -> eval_cache(cache, circuit, value)
  }
}

fn eval_cache(cache, circuit, var) {
  case var_as_int(var) {
    Ok(var) -> #(var, cache)
    Error(var) -> {
      case cache |> dict.get(var) {
        Ok(val) -> #(val, cache)
        Error(_) -> {
          let #(val, cache) = eval(cache, circuit, var)
          let cache = cache |> dict.insert(var, val)
          #(val, cache)
        }
      }
    }
  }
}

fn eval_bin_op(cache, circuit, f, left, right) {
  let #(left_val, cache) = eval_cache(cache, circuit, left)
  let #(right_val, cache) = eval_cache(cache, circuit, right)
  #(f(left_val, right_val), cache)
}

fn var_as_int(input) {
  input |> int.parse |> result.map_error(fn(_) { input })
}

type Op {
  And(left: String, right: String)
  Or(left: String, right: String)
  RShift(left: String, right: String)
  LShift(left: String, right: String)
  Not(operand: String)
  Value(value: String)
}

fn parse_circuit(input) {
  input
  |> string.split("\n")
  |> list.map(parse_connection)
  |> dict.from_list
}

fn parse_connection(line) {
  let assert Ok(#(operation, output)) = line |> string.split_once(" -> ")
  let operation = case operation |> string.split(" ") {
    [input] -> Value(input)
    [operator, operand] if operator == "NOT" -> Not(operand)
    [left, operator, right] ->
      case operator {
        "AND" -> And(left, right)
        "OR" -> Or(left, right)
        "RSHIFT" -> RShift(left, right)
        "LSHIFT" -> LShift(left, right)
        _ -> panic
      }
    _ -> panic
  }
  #(output, operation)
}
