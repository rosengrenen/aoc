input = open("input.txt").read()


def first_marker(l):
    return next(filter(lambda i: len(set(input[i:i+l])) == l, range(len(input) - l + 1))) + l


print("Part 1:", first_marker(4))
print("Part 2:", first_marker(14))
