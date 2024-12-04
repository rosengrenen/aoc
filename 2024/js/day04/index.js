const fs = require("fs");
fs.readFile("input.txt", { encoding: "utf8" }, (err, content) => {
	const grid = content.split("\n").map((line) => line.split(""));
	function get(row, col) {
		if (grid[row]) {
			return grid[row][col];
		}

		return undefined;
	}

	function row(row, col) {
		if (
			get(row, col) == "X" &&
			get(row, col + 1) == "M" &&
			get(row, col + 2) == "A" &&
			get(row, col + 3) == "S"
		) {
			return 1;
		}

		if (
			get(row, col) == "S" &&
			get(row, col + 1) == "A" &&
			get(row, col + 2) == "M" &&
			get(row, col + 3) == "X"
		) {
			return 1;
		}

		return 0;
	}
	function col(row, col) {
		if (
			get(row, col) == "X" &&
			get(row + 1, col) == "M" &&
			get(row + 2, col) == "A" &&
			get(row + 3, col) == "S"
		) {
			return 1;
		}

		if (
			get(row, col) == "S" &&
			get(row + 1, col) == "A" &&
			get(row + 2, col) == "M" &&
			get(row + 3, col) == "X"
		) {
			return 1;
		}

		return 0;
	}

	function diag(row, col) {
		if (
			get(row, col) == "X" &&
			get(row + 1, col + 1) == "M" &&
			get(row + 2, col + 2) == "A" &&
			get(row + 3, col + 3) == "S"
		) {
			return 1;
		}

		if (
			get(row, col) == "S" &&
			get(row + 1, col + 1) == "A" &&
			get(row + 2, col + 2) == "M" &&
			get(row + 3, col + 3) == "X"
		) {
			return 1;
		}

		return 0;
	}
	function diag2(row, col) {
		if (
			get(row, col) == "X" &&
			get(row + 1, col - 1) == "M" &&
			get(row + 2, col - 2) == "A" &&
			get(row + 3, col - 3) == "S"
		) {
			return 1;
		}

		if (
			get(row, col) == "S" &&
			get(row + 1, col - 1) == "A" &&
			get(row + 2, col - 2) == "M" &&
			get(row + 3, col - 3) == "X"
		) {
			return 1;
		}

		return 0;
	}

	function diag3(row, col) {
		if (
			get(row, col) == "S" &&
			get(row + 1, col + 1) == "A" &&
			get(row + 2, col + 2) == "M"
		) {
			return true;
		}

		if (
			get(row, col) == "M" &&
			get(row + 1, col + 1) == "A" &&
			get(row + 2, col + 2) == "S"
		) {
			return true;
		}

		return false;
	}
	function diag4(row, col) {
		if (
			get(row, col) == "S" &&
			get(row + 1, col - 1) == "A" &&
			get(row + 2, col - 2) == "M"
		) {
			return true;
		}

		if (
			get(row, col) == "M" &&
			get(row + 1, col - 1) == "A" &&
			get(row + 2, col - 2) == "S"
		) {
			return true;
		}

		return false;
	}

	let sum = 0;
	for (let r = 0; r < grid.length; r++) {
		for (let c = 0; c < grid[r].length; c++) {
			sum += row(r, c) + col(r, c) + diag(r, c) + diag2(r, c);
		}
	}

	console.log(sum);

	let sum2 = 0;
	for (let r = 0; r < grid.length; r++) {
		for (let c = 0; c < grid[r].length; c++) {
			if (diag3(r - 1, c - 1) && diag4(r - 1, c + 1)) {
				sum2 += 1;
			}
		}
	}
	console.log(sum2);
});
