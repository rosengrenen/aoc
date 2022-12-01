{-# LANGUAGE OverloadedStrings #-}

import Data.List (sort)
import qualified Data.Text as T

splitOn :: String -> String -> [String]
splitOn delim input = map T.unpack $ T.splitOn (T.pack delim) (T.pack input)

parseInput :: String -> [Int]
parseInput input = map (sum . map read . lines) (splitOn "\n\n" input)

solvePart1 :: String -> Int
solvePart1 = maximum . parseInput

solvePart2 :: String -> Int
solvePart2 = sum . take 3 . reverse . sort . parseInput

main :: IO ()
main =
  do
    input <- readFile "input.txt"
    putStrLn $ "Part one: " ++ show (solvePart1 input)
    putStrLn $ "Part two: " ++ show (solvePart2 input)
