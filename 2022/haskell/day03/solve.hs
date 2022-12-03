{-# LANGUAGE OverloadedStrings #-}

import Data.Char (ord)
import Data.List (intersect, sort)
import Data.List.Split (chunksOf)

parseInput :: String -> [[Int]]
parseInput input = [[priority c | c <- line] | line <- lines input]

priority :: Char -> Int
priority c
  | c >= 'a' = ord c - ord 'a' + 1
  | otherwise = ord c - ord 'A' + 27

halve :: [a] -> ([a], [a])
halve xs = splitAt (div (length xs) 2) xs

intersect' :: Eq a => ([a], [a]) -> [a]
intersect' (a, b) = a `intersect` b

solvePart1 :: String -> Int
solvePart1 = sum . map (head . intersect' . halve) . parseInput

solvePart2 :: String -> Int
solvePart2 = sum . map intersectGroup . chunksOf 3 . parseInput

intersectGroup :: [[Int]] -> Int
intersectGroup (inv1 : inv2 : inv3 : _) = head $ intersect (inv1 `intersect` inv2) inv3
intersectGroup _ = error "uh oh"

main :: IO ()
main =
  do
    input <- readFile "input.txt"
    putStrLn $ "Part one: " ++ show (solvePart1 input)
    putStrLn $ "Part two: " ++ show (solvePart2 input)
