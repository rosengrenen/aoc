module Day01

open Expecto

open Util

let parseLine (line: string) =
    let left, right = splitOnce line "   " |> Option.get
    left |> int, right |> int

let part1 input =
    let entries = lines input |> Seq.map parseLine
    let left = entries |> Seq.map fst |> Seq.sort
    let right = entries |> Seq.map snd |> Seq.sort

    Seq.zip left right
    |> Seq.map (fun (left, right) -> left - right |> abs)
    |> Seq.sum
    |> string

let part2 input =
    let entries = lines input |> Seq.map parseLine
    let frequency = entries |> Seq.map snd |> Seq.countBy id |> Map

    let getFrequency value =
        frequency |> Map.tryFind value |> Option.defaultValue 0

    entries
    |> Seq.map fst
    |> Seq.map (fun value -> value * getFrequency value)
    |> Seq.sum
    |> string

let part1Tests =
    testList "Day 1 part 1 tests" [ partTestCase part1 "day01/example1.txt" "11" ]

let part2Tests =
    testList "Day 1 part 2 tests" [ partTestCase part2 "day01/example1.txt" "31" ]
