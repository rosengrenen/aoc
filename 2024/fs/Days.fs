module Days

let dayRunners day =
    match day with
    | 1 -> (Day01.part1, Day01.part2)
    | 2 -> (Day02.part1, Day02.part2)
    | 3 -> (Day03.part1, Day03.part2)
    | _ -> System.NotImplementedException "" |> raise

let dayTests day =
    match day with
    | 1 -> (Day01.part1Tests, Day01.part2Tests)
    | 2 -> (Day02.part1Tests, Day02.part2Tests)
    | 3 -> (Day03.part1Tests, Day03.part2Tests)
    | _ -> System.NotImplementedException "" |> raise

let MAX_DAYS = 1
