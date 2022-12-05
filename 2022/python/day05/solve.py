input = open("input.txt").read()
[stacks, instrs] = input.split('\n\n')
stack_lines = [[line[1 + i * 4] for i in range(9)]
               for line in reversed(stacks.split('\n')[:-1])]
stacks = [[] for _ in range(len(stack_lines[0]))]

for line in stack_lines:
    for i in range(len(line)):
        if line[i] != " ":
            stacks[i].append(line[i])

stacks_copy = [stack.copy() for stack in stacks]

for instr in instrs.splitlines():
    [_, amt, _, f, _, t] = instr.split(" ")
    for _ in range(int(amt)):
        stacks[int(t) - 1].append(stacks[int(f) - 1].pop())

part1 = "".join(str(stack.pop()) for stack in stacks)

stacks = stacks_copy

for instr in instrs.splitlines():
    [_, amt, _, f, _, t] = instr.split(" ")
    crates = []
    for _ in range(int(amt)):
        crates.append(stacks[int(f) - 1].pop())
    for crate in reversed(crates):
        stacks[int(t) - 1].append(crate)

part2 = "".join(str(stack.pop()) for stack in stacks)

print("Part 1:", part1)
print("Part 2:", part2)
