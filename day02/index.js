const fs = require('fs');

const input = fs.readFileSync('input.txt', { encoding: 'utf-8' });
const commands = input.split(",").map(cmd => parseInt(cmd, 10));

function runMachine(input) {
  const cmds = input.slice(0, input.length)
  let running = true;
  let programPointer = 0;
  while (running) {
    switch (cmds[programPointer]) {
      case 1:
        cmds[cmds[programPointer + 3]] =
          cmds[cmds[programPointer + 1]] + cmds[cmds[programPointer + 2]];
        break;
      case 2:
        cmds[cmds[programPointer + 3]] =
          cmds[cmds[programPointer + 1]] * cmds[cmds[programPointer + 2]];
        break;
      case 99:
        running = false;
        break;
      default:
        console.log(cmds.slice(programPointer, programPointer + 4));
        throw new Error(
          "Invalid opcode at",
          programPointer,
          cmds[programPointer]
        );
    }

    programPointer += 4;
  }

  return { state: cmds, result: cmds[0] };
}

const partOneInput = commands.slice(0, commands.length);
partOneInput[1] = 12;
partOneInput[2] = 2;
console.log("Part one", runMachine(partOneInput).result);

let noun = 0;
let verb = 0;
while (true) {
  const partTwoInput = commands.slice(0, commands.length);
  partTwoInput[1] = noun;
  partTwoInput[2] = verb;
  const result = runMachine(partTwoInput);
  if (result.result === 19690720) {
    break;
  }
  noun++;
  if (noun > 99) {
    noun = 0
    verb++;
  }
  if (verb > 99) {
    console.log('No answer');
    break;
  }
}

console.log('Part two:', 100 * noun + verb);