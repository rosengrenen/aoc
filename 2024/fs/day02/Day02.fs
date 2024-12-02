module Day02

open Expecto

open Util

let parse_report line = line |> words |> Seq.map int

let parse input = lines input |> Seq.map parse_report

let diffs report =
    report |> Seq.windowed 2 |> Seq.map (fun a -> Array.item 1 a - Array.item 0 a)

let increasing n = n > 0 && n <= 3
let decreasing n = n < 0 && n >= -3

let part1 input =
    parse input
    |> Seq.map diffs
    |> Seq.filter (fun ds -> (Seq.forall increasing) ds || (Seq.forall decreasing) ds)
    |> Seq.length
    |> string

let skipIndex indexToSkip list =
    list
    |> Seq.mapi (fun i x -> if i = indexToSkip then None else Some x)
    |> Seq.choose id

let safe_descent report =
    let indices = seq { 0 .. Seq.length report - 1 }

    indices
    |> Seq.exists (fun index ->
        report
        |> skipIndex index
        |> diffs
        |> fun ds -> Seq.forall increasing ds || Seq.forall decreasing ds)

let part2 input =
    parse input |> Seq.filter safe_descent |> Seq.length |> string

let part1Tests =
    testList "Day 2 part 1 tests" [ partTestCase part1 "day02/example1.txt" "2" ]

let part2Tests =
    testList "Day 2 part 2 tests" [ partTestCase part2 "day02/example1.txt" "4" ]
