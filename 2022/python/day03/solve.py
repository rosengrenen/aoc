input = open("input.txt").read()
invs = [[int(item) for item in inv.splitlines()]
        for inv in input.split("\n\n")]
invs = [int(item) for item in input.splitlines()]


def score(elf, me):
    return (me - elf + 1) % 3 * 3 + me + 1


def selectscore(elf, result):
    return score(elf, (elf + result) % 3)


print("Part 1:", sum([score(elf, me) for (elf, me) in invs]))
print("Part 2:", sum([selectscore(elf, me - 1) for (elf, me) in invs]))
