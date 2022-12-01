input = open("input.txt").read()
invs = sorted([sum([int(item) for item in inv.splitlines()])
               for inv in input.split('\n\n')], reverse=True)

print("Part 1:", invs[0])
print("Part 2:", invs[0] + invs[1] + invs[2])
