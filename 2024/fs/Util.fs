module Util

open Expecto
open System.IO
open System.Text.RegularExpressions

let lines (input: string) = input.Split('\n') |> Array.toSeq

let words (input: string) =
    input.Split([| ' '; '\t'; '\n'; '\r' |], System.StringSplitOptions.RemoveEmptyEntries)
    |> Array.toSeq

let ints (input: string) =
    seq {
        for m in Regex.Matches(input, "-?\d+") do
            m.Value |> int
    }

let splitOnce (input: string) (separator: string) =
    let parts = input.Split(separator) |> Array.toList

    if parts.Length <> 2 then
        None
    else
        Some(parts |> List.item 0, parts |> List.item 1)

let formatDay (day: int) : string = sprintf "day%02d" day

let partTestCase runner inputPath expected =
    testCase inputPath
    <| fun _ ->
        let input = File.ReadAllText inputPath
        let result = runner input
        Expect.equal result expected ""

let extraTestCase name result expected =
    testCase name <| fun _ -> Expect.equal result expected ""
