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

// booooring
let safe_descent report =
    seq { 0 .. Seq.length report - 1 }
    |> Seq.exists (fun index ->
        report
        |> skipIndex index
        |> diffs
        |> fun ds -> Seq.forall increasing ds || Seq.forall decreasing ds)

// ahh yess!!
let rec safe_descent2' report skipped =
    match report with
    | [] -> true
    | [ a ] -> if skipped then increasing a else true
    | a :: b :: rest ->
        match skipped, increasing a, increasing b with
        | (_, true, true) -> safe_descent2' (b :: rest) skipped
        | (false, true, _) -> safe_descent2' (b :: rest) skipped || safe_descent2' (a + b :: rest) true
        | (false, _, _) when a + b |> increasing -> safe_descent2' (a + b :: rest) true
        | _ -> false

let safe_descent2 report =
    let diffs = report |> diffs |> Seq.toList

    let safe_descent2_inner diffs =
        match diffs with
        | [] -> true
        | [ a ] -> increasing a
        | a :: b :: rest ->
            match increasing a, a + b |> increasing with
            | (true, _) -> safe_descent2' diffs false
            | (_, true) -> safe_descent2' (a + b :: rest) true
            | (_, false) -> safe_descent2' (b :: rest) true

    safe_descent2_inner diffs || safe_descent2_inner (diffs |> List.map (~-))

let part2 input =
    parse input |> Seq.filter safe_descent2 |> Seq.length |> string

let part1Tests =
    testList "Day 2 part 1 tests" [ partTestCase part1 "day02/example1.txt" "2" ]

let part2Tests =
    testList "Day 2 part 2 tests" [ partTestCase part2 "day02/example1.txt" "4" ]
