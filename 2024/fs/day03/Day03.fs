module Day03

open Expecto
open System.Text.RegularExpressions

open Util

let part1 input =
    Regex.Matches(input, "mul\((\d+),(\d+)\)")
    |> Seq.sumBy (fun m -> m.Value |> ints |> Seq.reduce (*))
    |> string

let part2 input =
    Regex.Matches(input, "mul\((\d+),(\d+)\)|do(n't)?\(\)")
    |> Seq.fold
        (fun (sum, enabled) m ->
            match m.Value, enabled with
            | "do()", _ -> sum, true
            | "don't()", _ -> sum, false
            | mul, true -> sum + (mul |> ints |> Seq.reduce (*)), enabled
            | _, false -> sum, false)
        (0, true)
    |> fst
    |> string

let part1Tests =
    testList "Day 3 part 1 tests" [ partTestCase part1 "day03/example1.txt" "161" ]

let part2Tests =
    testList "Day 3 part 2 tests" [ partTestCase part2 "day03/example2.txt" "48" ]
