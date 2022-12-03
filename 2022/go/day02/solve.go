package main

import (
	"fmt"
	"os"
	"strings"
)

type game struct {
	elf int
	me  int
}

func main() {
	content, _ := os.ReadFile("input.txt")
	games := make([]game, 0)

	for _, g := range strings.Split(string(content), "\n") {
		elf := int(g[0]) - int('A')
		me := int(g[2]) - int('X')

		games = append(games, game{elf: elf, me: me})
	}

	part1Sum := 0
	for _, game := range games {
		part1Sum += score(game.elf, game.me)
	}

	part2Sum := 0
	for _, game := range games {
		part2Sum += selectscore(game.elf, game.me)
	}

	fmt.Println("Part 1:", part1Sum)
	fmt.Println("Part 2:", part2Sum)
}

func score(elf, me int) int {
	return (me-elf+4)%3*3 + me + 1
}

func selectscore(elf, me int) int {
	return score(elf, (elf+me+2)%3)
}

// int main()
// {
//   auto input = read_to_string("input.txt");
//   std::vector<std::pair<int, int>> matches;
//   for (auto match : split(input, "\n"))
//   {
//     matches.push_back(std::make_pair(match[0] - 'A', match[2] - 'X'));
//   }

//   auto score = [](int elf, int me)
//   { return (me - elf + 4) % 3 * 3 + me + 1; };
//   auto selectscore = [&](int elf, int me)
//   { return score(elf, (elf + me + 2) % 3); };

//   int part1_sum = 0;
//   for (auto match : matches)
//   {
//     part1_sum += score(std::get<0>(match), std::get<1>(match));
//   }

//   int part2_sum = 0;
//   for (auto match : matches)
//   {
//     part2_sum += selectscore(std::get<0>(match), std::get<1>(match));
//   }

//   std::cout << "Part 1: " << part1_sum << '\n';
//   std::cout << "Part 2: " << part2_sum << '\n';
// }
