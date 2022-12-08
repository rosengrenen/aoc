import copy

input = open("input.txt").read()
grid = [[int(c) for c in line] for line in input.splitlines()]
visible = [[False for _ in row] for row in grid]

for row in range(len(grid)):
    left = -1
    right = -1
    for i in range(len(grid[row])):
        if grid[row][i] > left:
            visible[row][i] = True
            left = grid[row][i]
        if grid[row][-1-i] > right:
            visible[row][-1-i] = True
            right = grid[row][-1-i]

for col in range(len(grid[0])):
    top = -1
    bottom = -1
    for i in range(len(grid)):
        if grid[i][col] > top:
            visible[i][col] = True
            top = grid[i][col]
        if grid[-1-i][col] > bottom:
            visible[-1-i][col] = True
            bottom = grid[-1-i][col]

dists = copy.deepcopy(grid)
for row in range(len(grid)):
    for col in range(len(grid[row])):
        up = 0
        right = 0
        down = 0
        left = 0
        for c in range(col + 1, len(grid[row])):
            right += 1
            if grid[row][c] >= grid[row][col]:
                break

        for c in reversed(range(0, col)):
            left += 1
            if grid[row][c] >= grid[row][col]:
                break

        for r in range(row + 1, len(grid)):
            down += 1
            if grid[r][col] >= grid[row][col]:
                break

        for r in reversed(range(0, row)):
            up += 1
            if grid[r][col] >= grid[row][col]:
                break

        dists[row][col] = up * right * down * left

print("Part 1:", len([c for row in visible for c in filter(lambda c: c, row)]))
print("Part 2:", max([c for row in dists for c in row]))
