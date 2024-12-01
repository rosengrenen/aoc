open Argu
open Expecto
open System.IO

open Days
open Util

let runDayInner day part =
    let part1Runner, part2Runner = dayRunners day
    let input = File.ReadAllText $"{formatDay day}/input.txt"

    let runPartOne () = printfn $"Part 1: {part1Runner input}"
    let runPartTwo () = printfn $"Part 2: {part2Runner input}"

    printfn $"Day {day}"

    match part with
    | Some 1 -> runPartOne ()
    | Some 2 -> runPartTwo ()
    | _ ->
        runPartOne ()
        runPartTwo ()

    printfn ""

let runDay day part =
    match day with
    | Some day -> runDayInner day part
    | None ->
        for day = 1 to MAX_DAYS do
            runDayInner day part

let runDayTestsInner day part =
    let part1Tests, part2Tests = dayTests day

    let runPartOne () =
        runTestsWithCLIArgs [] [||] part1Tests |> ignore

    let runPartTwo () =
        runTestsWithCLIArgs [] [||] part2Tests |> ignore

    match part with
    | Some 1 -> runPartOne ()
    | Some 2 -> runPartTwo ()
    | _ ->
        runPartOne ()
        runPartTwo ()

let runDayTests day part =
    match day with
    | Some day -> runDayTestsInner day part
    | None ->
        for day = 1 to MAX_DAYS do
            runDayTestsInner day part

type Argument =
    | [<AltCommandLine("-d")>] Day of day: int
    | [<AltCommandLine("-p")>] Part of part: int
    | [<AltCommandLine("-t")>] Test

    interface IArgParserTemplate with
        member s.Usage =
            match s with
            | Day _ -> "day"
            | Part _ -> "part"
            | Test -> "test"

[<EntryPoint>]
let main argv =
    let parser = ArgumentParser.Create<Argument>(programName = "aoc2024")
    let result = parser.Parse(argv)
    let day = result.TryGetResult Day
    let part = result.TryGetResult Part
    let test = result.Contains Test

    if test then runDayTests day part else runDay day part
    0
