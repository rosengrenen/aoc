input = open("input.txt").read()

pairs = [[range(int(elf.split('-')[0]), int(elf.split('-')[1]) + 1)
         for elf in pair.split(',')] for pair in input.split('\n')]


def ri(r1, r2):
    return range(max(r1[0], r2[0]), min(r1[-1], r2[-1]) + 1)


part1 = len(list(filter(lambda p:
                        ri(p[0], p[1]) == p[0] or ri(p[0], p[1]) == p[1], pairs)))

part2 = len(list(filter(lambda p:
                        len(ri(p[0], p[1])) > 0, pairs)))

print("Part 1:", part1)
print("Part 2:", part2)
