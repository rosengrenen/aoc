module Day02

open Expecto

open Util

let parse input =
    lines input |> Seq.map words |> Seq.map (Seq.map int) |> Seq.map Seq.toList

let rec diffs nums =
    match nums with
    | []
    | [ _ ] -> []
    | a :: b :: rest -> b - a :: diffs (b :: rest)

let increasing n = n > 0 && n <= 3
let decreasing n = n < 0 && n >= -3

let part1 input =
    parse input
    |> Seq.map diffs
    |> Seq.filter (fun ds -> (List.forall increasing) ds || (List.forall decreasing) ds)
    |> Seq.length
    |> string

let rec safe_decreasing (diffs: int list) (p: bool) =
    match diffs with
    | [] -> true
    | [ a ] -> decreasing a || p
    | a :: b :: rest ->
        match decreasing a, decreasing b with
        | (true, true) -> safe_decreasing (b :: rest) p
        | (true, false)
        | (false, true) -> safe_decreasing (a + b :: rest) false
        | (false, false) -> false

let rec safe_increasing (diffs: int list) (p: bool) =
    match diffs with
    | [] -> true
    | [ a ] -> increasing a || p
    | a :: b :: rest ->
        match increasing a, increasing b with
        | (true, true) -> safe_increasing (b :: rest) p
        | (true, false)
        | (false, true) -> safe_increasing (a + b :: rest) false
        | (false, false) -> false

let part2 input =
    parse input
    |> Seq.map diffs
    |> Seq.filter (fun ds -> safe_increasing ds true || safe_decreasing ds true)
    |> Seq.length
    |> string

let part1Tests =
    testList "Day 2 part 1 tests" [ partTestCase part1 "day02/example1.txt" "2" ]

let part2Tests =
    testList "Day 2 part 2 tests" [ partTestCase part2 "day02/example1.txt" "4" ]
