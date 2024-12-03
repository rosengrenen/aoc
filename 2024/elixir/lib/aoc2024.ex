defmodule Aoc2024 do
  def day_runners(day) do
    case day do
      1 -> {&Day01.part1/1, &Day01.part2/1}
      _ -> raise "not implmented"
    end
  end

  def run_day(day) do
    {p1, p2} = day_runners(day)

    with {:ok, input} <- File.read("lib/day01/input.txt") do
      IO.puts("Day #{day}")
      IO.puts("Part 1: #{p1.(input)}")
      IO.puts("Part 2: #{p2.(input)}")
    end
  end
end

Aoc2024.run_day(1)
