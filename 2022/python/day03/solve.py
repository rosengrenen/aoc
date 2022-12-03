input = open("input.txt").read()
invs = input.splitlines()


def priority(item: chr):
    if ord('a') <= ord(item) <= ord('z'):
        return ord(item) - ord('a') + 1
    return ord(item) - ord('A') + 27


compartments = [[inv[0:int(len(inv)/2)], inv[int(len(inv)/2):]]
                for inv in invs]
common_items = [list(set(comp1).intersection(set(comp2)))[0]
                for [comp1, comp2] in compartments]
part1_sum = sum([priority(item) for item in common_items])


def chunks(lst, n):
    for i in range(0, len(lst), n):
        yield lst[i:i + n]


elf_groups = chunks(invs, 3)
common_items = [list(set(inv1).intersection(set(inv2)).intersection(set(inv3)))[0]
                for [inv1, inv2, inv3] in elf_groups]
part2_sum = sum([priority(item) for item in common_items])

print("Part 1:", part1_sum)
print("Part 2:", part2_sum)
