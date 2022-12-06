{-# LANGUAGE OverloadedStrings #-}

import Data.Set (fromList)

firstMarker :: Int -> [Char] -> Int
firstMarker l input = head [i | i <- [0 ..], (length . fromList . take l . drop i $ input) == l] + l

solvePart1 :: String -> Int
solvePart1 = firstMarker 4

solvePart2 :: String -> Int
solvePart2 = firstMarker 14

main :: IO ()
main =
  do
    input <- readFile "input.txt"
    print input
    putStrLn $ "Part one: " ++ show (solvePart1 input)
    putStrLn $ "Part two: " ++ show (solvePart2 input)
