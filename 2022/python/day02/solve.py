input = open("input.txt").read()
invs = [(ord(item[0]) - ord('A'), ord(item[2]) - ord('X'))
        for item in input.splitlines()]


def score(elf, me):
    return (me - elf + 1) % 3 * 3 + me + 1


def selectscore(elf, result):
    return score(elf, (elf + result) % 3)


print("Part 1:", sum([score(elf, me) for (elf, me) in invs]))
print("Part 2:", sum([selectscore(elf, me - 1) for (elf, me) in invs]))
