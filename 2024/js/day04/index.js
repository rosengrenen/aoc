const fs = require("fs");
fs.readFile("input.txt", { encoding: "utf8" }, (err, content) => {
  const grid = content.split("\n").map((line) => line.split(""));
  const get = (x, y) => (grid[y] ? grid[y][x] : undefined);
  const arr = (length, mapper) => Array.from({ length }, (_, i) => mapper(i));
  const row = (x, y, l) => arr(l, (i) => [x + i, y]);
  const col = (x, y, l) => arr(l, (i) => [x, y + i]);
  const diag1 = (x, y, l) => arr(l, (i) => [x + i, y + i]);
  const diag2 = (x, y, l) => arr(l, (i) => [x + i, y - i]);
  const match = (coords, string) => coords.every(([x, y], i) => get(x, y) === string[i]);
  const matchMany = (coords, strings) => strings.some((string) => match(coords, string));

  const allCoords = arr(grid.length, (y) => arr(grid[y].length, (x) => [x, y])).flat();

  const seqs = (x, y, l) => [row(x, y, l), col(x, y, l), diag1(x, y, l), diag2(x, y, l)];
  const matchSum = (seqs, string) => seqs.reduce((acc, coords) => acc + (match(coords, string) ? 1 : 0), 0);
  const p1 = allCoords.reduce(
    (acc, [x, y]) => acc + matchSum(seqs(x, y, 4), "XMAS") + matchSum(seqs(x, y, 4), "SAMX"),
    0
  );
  console.log("Part 1", p1);

  const crossSeqs = (x, y) => [diag1(x - 1, y - 1, 3), diag2(x - 1, y + 1, 3)];
  const matchAll = (seqs, strings) => (seqs.every((coords) => matchMany(coords, strings)) ? 1 : 0);
  const p2 = allCoords.reduce((acc, [x, y]) => acc + matchAll(crossSeqs(x, y), ["SAM", "MAS"]), 0);
  console.log("Part 2", p2);
});
