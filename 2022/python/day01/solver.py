def solve01_p1(input):
    invs = sorted([sum([int(item) for item in inv.splitlines()])
                   for inv in input.split('\n\n')], reverse=True)

    return invs[0] + invs[1] + invs[2]


def solve01_p2(input):
    invs = sorted([sum([int(item) for item in inv.splitlines()])
                   for inv in input.split('\n\n')], reverse=True)

    return invs[0]
