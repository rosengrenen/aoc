{-# LANGUAGE OverloadedStrings #-}

import Data.Text (pack, splitOn, unpack)

type Range = (Int, Int)

splitOnce :: String -> String -> (String, String)
splitOnce delim input = (a, b)
  where
    (a : b : _) = map unpack $ splitOn (pack delim) (pack input)

parseInput :: String -> [(Range, Range)]
parseInput input = map (parseElfPair . splitOnce ",") (lines input)

parseElfPair :: (String, String) -> (Range, Range)
parseElfPair (e0, e1) = (parseRange $ splitOnce "-" e0, parseRange $ splitOnce "-" e1)

parseRange :: (String, String) -> Range
parseRange (start, end) = (read start, read end)

rangeOverlap :: (Range, Range) -> Bool
rangeOverlap ((s0, e0), (s1, e1)) = e0 >= s1 && e1 >= s0

rangeContains :: Range -> Range -> Bool
rangeContains (s0, e0) (s1, e1) = s1 >= s0 && e1 <= e0

solvePart1 :: String -> Int
solvePart1 = length . filter (\(r0, r1) -> rangeContains r0 r1 || rangeContains r1 r0) . parseInput

solvePart2 :: String -> Int
solvePart2 = length . filter rangeOverlap . parseInput

main :: IO ()
main =
  do
    input <- readFile "input.txt"
    putStrLn $ "Part one: " ++ show (solvePart1 input)
    putStrLn $ "Part two: " ++ show (solvePart2 input)
