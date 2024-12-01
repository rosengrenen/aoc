module Day01

open Util

let parseLine (line: string) =
    let left, right = splitOnce line "   " |> Option.get
    left |> int, right |> int

let parse (input: string) =
    let lines = lines input |> Seq.map parseLine
    let leftList = Seq.map (fun parts -> fst parts) lines |> Seq.toList
    let rightList = Seq.map (fun parts -> snd parts) lines |> Seq.toList
    List.sort leftList, List.sort rightList

let part1 input =
    let left, right = parse input

    List.zip left right
    |> List.map (fun (left, right) -> left - right |> abs)
    |> List.sum
    |> string

let part2 input =
    let left, right = parse input
    let frequency = List.countBy id right |> Map

    let getFrequency value =
        frequency |> Map.tryFind value |> Option.defaultValue 0

    left |> List.map (fun value -> value * getFrequency value) |> List.sum |> string
