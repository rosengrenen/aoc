const fs = require("fs");
const input = fs.readFileSync("input.txt", { encoding: "utf8" });

const invs = input
	.split("\n\n")
	.map((inv) => inv.split("\n").reduce((sum, line) => sum + parseInt(line), 0))
	.sort((a, b) => b - a);

console.log("Part 1:", invs[0]);
console.log("Part 2:", invs[0] + invs[1] + invs[2]);
