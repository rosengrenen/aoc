module Util

let lines (input: string) = input.Split('\n') |> Array.toSeq

let splitOnce (input: string) (separator: string) =
    let parts = input.Split(separator)

    if parts.Length <> 2 then
        None
    else
        Some(parts.[0], parts.[1])
