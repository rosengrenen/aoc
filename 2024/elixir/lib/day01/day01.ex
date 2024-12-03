defmodule Day01 do
  def part1(input) do
    {left, right} = parse(input)

    left = left |> Enum.sort()
    right = right |> Enum.sort()

    Enum.zip(left, right)
    |> Enum.map(fn {l, r} -> l - r end)
    |> Enum.map(&abs/1)
    |> Enum.sum()
    |> Integer.to_string()
  end

  def part2(input) do
    {left, right} = parse(input)
    freq = Enum.frequencies(right)
    left |> Enum.map(fn n -> n * Map.get(freq, n, 0) end) |> Enum.sum() |> Integer.to_string()
  end

  def parse(input) do
    entries =
      input
      |> String.split("\n", trim: true)
      |> Enum.map(fn line ->
        line |> String.split(" ", trim: true) |> Enum.map(&String.to_integer/1)
      end)

    left = entries |> Enum.map(fn [x, _] -> x end)
    right = entries |> Enum.map(fn [_, x] -> x end)
    {left, right}
  end
end
