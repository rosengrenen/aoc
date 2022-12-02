{-# LANGUAGE OverloadedStrings #-}

import Data.Char (ord)
import Data.List (sort)

parseInput :: String -> [(Int, Int)]
parseInput input = [(ord elf - ord 'A', ord me - ord 'X') | (elf : _ : me : _) <- lines input]

solvePart1 :: String -> Int
solvePart1 = sum . map score . parseInput

solvePart2 :: String -> Int
solvePart2 = sum . map selectscore . parseInput

score :: (Int, Int) -> Int
score (elf, me) = (me - elf + 1) `mod` 3 * 3 + me + 1

selectscore :: (Int, Int) -> Int
selectscore (elf, me) = score (elf, (elf + me - 1) `mod` 3)

main :: IO ()
main =
  do
    input <- readFile "input.txt"
    putStrLn $ "Part one: " ++ show (solvePart1 input)
    putStrLn $ "Part two: " ++ show (solvePart2 input)
