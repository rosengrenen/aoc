const fs = require('fs');

function calculateFuelCost(weight) {
  return Math.floor(weight / 3) - 2;
}

const input = fs.readFileSync('input.txt', { encoding: 'utf-8' });

const weights = input.split('\n').map(line => parseInt(line, 10));

const partOne = weights
  .map(weight => calculateFuelCost(weight))
  .reduce((previous, current) => previous + current, 0);

function calculateFuelCostComplete(weight) {
  const fuelCost = calculateFuelCost(weight)
  if (fuelCost <= 0) {
    return 0;
  } else {
    return fuelCost + calculateFuelCost(fuelCost);
  }
}

const partTwo = weights
  .map(weight => calculateFuelCostComplete(weight))
  .reduce((previous, current) => previous + current, 0);
