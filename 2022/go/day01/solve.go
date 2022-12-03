package main

import (
	"fmt"
	"os"
	"sort"
	"strconv"
	"strings"
)

func main() {
	content, _ := os.ReadFile("input.txt")
	invs := make([]int, 0)

	for _, inv := range strings.Split(string(content), "\n\n") {
		items := strings.Split(inv, "\n")
		totalCals := 0
		for _, item := range items {
			cals, _ := strconv.Atoi(item)
			totalCals += cals
		}

		invs = append(invs, totalCals)

	}

	sort.Slice(invs, func(i, j int) bool {
		return invs[i] > invs[j]
	})

	fmt.Println("Part 1:", invs[0])
	fmt.Println("Part 2:", invs[0]+invs[1]+invs[2])
}
