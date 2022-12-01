import requests
from dotenv import load_dotenv
import os
import sys

from day01.solver import solve01_p1, solve01_p2

load_dotenv()


def get_input(day: int) -> str:
    input_path = "day{0:0>2}/input.txt".format(day)
    try:
        return open(input_path).read()
    except:
        pass

    aoc_session = os.getenv("AOC_SESSION")
    res = requests.get("https://adventofcode.com/2022/day/{}/input".format(day),
                       cookies={'session': aoc_session})
    open(input_path, 'w').write(res.text)
    return res.text


def print_res(res, day: int, p1: bool):
    if p1:
        print("Day {} part 1: {}".format(day, res))
    else:
        print("Day {} part 2: {}".format(day, res))


def solve_day(day: int, p1: bool):
    match (day, p1):
        case (1, True):
            print_res(solve01_p1(get_input(day)), day, p1)
        case (1, False):
            print_res(solve01_p2(get_input(day)), day, p1)


def main():
    if len(sys.argv) == 1:
        for day in range(1, 26):
            solve_day(day, True)
            solve_day(day, False)
    elif len(sys.argv) == 2:
        day = int(sys.argv[1])
        solve_day(day, True)
        solve_day(day, False)
    elif len(sys.argv) == 3:
        day = int(sys.argv[1])
        p1 = sys.argv[2] == "1"
        solve_day(day, p1)
    else:
        print("Usage: python main.py day [part]")


if __name__ == "__main__":
    main()
