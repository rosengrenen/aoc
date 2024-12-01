module Util

open Expecto
open System.IO

let lines (input: string) = input.Split('\n') |> Array.toSeq

let splitOnce (input: string) (separator: string) =
    let parts = input.Split(separator)

    if parts.Length <> 2 then
        None
    else
        Some(parts.[0], parts.[1])

let formatDay (day: int) : string = sprintf "day%02d" day

let partTestCase runner inputPath expected =
    testCase ""
    <| fun _ ->
        let input = File.ReadAllText inputPath
        let result = runner input
        Expect.equal result expected ""
