module Day01

open Expecto

open Util

let parseLine (line: string) =
    let left, right = splitOnce line "   " |> Option.get
    left |> int, right |> int

let parse (input: string) =
    let lines = lines input |> Seq.map parseLine
    let left = lines |> Seq.map fst
    let right = lines |> Seq.map snd
    Seq.sort left, Seq.sort right

let part1 input =
    let left, right = parse input

    Seq.zip left right
    |> Seq.map (fun (left, right) -> left - right |> abs)
    |> Seq.sum
    |> string

let part2 input =
    let left, right = parse input
    let frequency = right |> Seq.countBy id |> Map

    let getFrequency value =
        frequency |> Map.tryFind value |> Option.defaultValue 0

    left |> Seq.map (fun value -> value * getFrequency value) |> Seq.sum |> string

let part1Tests =
    testList "Day 1 part 1 tests" [ partTestCase part1 "day01/example1.txt" "11" ]

let part2Tests =
    testList "Day 1 part 2 tests" [ partTestCase part2 "day01/example1.txt" "31" ]
