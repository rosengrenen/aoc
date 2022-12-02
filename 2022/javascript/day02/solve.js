const fs = require("fs");
const input = fs.readFileSync("input.txt", { encoding: "utf8" });

const matches = input
	.split("\n")
	.map((line) => [
		line.charCodeAt(0) - "A".charCodeAt(0),
		line.charCodeAt(2) - "X".charCodeAt(0),
	]);

const score = (elf, me) => ((me - elf + 4) % 3) * 3 + me + 1;
const selectscore = (elf, me) => score(elf, (elf + me + 2) % 3);

console.log(
	"Part 1:",
	matches.reduce((sum, [elf, me]) => sum + score(elf, me), 0)
);
console.log(
	"Part 2:",
	matches.reduce((sum, [elf, me]) => sum + selectscore(elf, me), 0)
);
