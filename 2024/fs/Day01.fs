module Day01

let parseLine (line: string) =
    let parts = line.Split("   ")
    parts.[0] |> int, parts.[1] |> int

let parse (input: string) =
    let lines = input.Split('\n') |> Array.toList
    let lines = List.map (fun line -> parseLine line) lines
    let leftList = List.map (fun parts -> fst parts) lines
    let rightList = List.map (fun parts -> snd parts) lines
    List.sort leftList, List.sort rightList

let intToString (number: int) = number.ToString()

let part1 input =
    let leftList, rightList = parse input

    List.zip leftList rightList
    |> List.map (fun (left, right) -> abs (left - right))
    |> List.sum
    |> intToString

let part2 input =
    let leftList, rightList = parse input
    let frequency = Seq.countBy id rightList |> Map

    let getFrequency value =
        let found, value = frequency.TryGetValue value
        if found then value else 0

    List.map (fun value -> value * getFrequency value) leftList
    |> List.sum
    |> intToString
