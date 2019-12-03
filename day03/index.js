const fs = require('fs');

const input = fs.readFileSync('input.txt', { encoding: 'utf-8' });
const paths = input.split("\n").map(path => path.split(","));
// const paths = ['R75,D30,R83,U83,L12,D49,R71,U7,L72', 'U62,R66,U55,R34,D71,R55,D58,R83'].map(path => path.split(","));
// const paths = ['R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51', 'U98,R91,D20,R16,D67,R40,U7,R15,U6,R7'].map(path => path.split(","));

const pathLocations = paths.map(path => {
  return path.reduce((previousLocations, instruction) => {
    const direction = instruction.charAt(0);
    const steps = parseInt(instruction.substring(1), 10);

    let previousLocation;
    if (previousLocations.length === 0) {
      previousLocation = { x: 0, y: 0, steps: 0 };
    } else {
      previousLocation = previousLocations[previousLocations.length - 1];
    }

    let newLocations = []
    for (let i = 1; i <= steps; ++i) {
      switch (direction) {
        case "D":
          newLocations.push({
            x: previousLocation.x,
            y: previousLocation.y - i,
            steps: previousLocations.length + i
          });
          break;
        case "U":
          newLocations.push({
            x: previousLocation.x,
            y: previousLocation.y + i,
            steps: previousLocations.length + i
          });
          break;
        case "L":
          newLocations.push({
            x: previousLocation.x - i,
            y: previousLocation.y,
            steps: previousLocations.length + i
          });
          break;
        case "R":
          newLocations.push({
            x: previousLocation.x + i,
            y: previousLocation.y,
            steps: previousLocations.length + i
          });
          break;
      }
    }

    return [...previousLocations, ...newLocations];
  }, []);
});

const [firstPath, secondPath] = pathLocations;

const intersections = [];
for (let i = 0; i < firstPath.length; ++i) {
  // console.log((i / firstPath.length * 100).toFixed(2), '%');
  for (let j = 0; j < secondPath.length; ++j) {
    if (firstPath[i].x === secondPath[j].x && firstPath[i].y === secondPath[j].y) {
      console.log("They meet at", firstPath[i].x, firstPath[i].y);
      intersections.push({
        ...firstPath[i],
        steps: firstPath[i].steps + secondPath[j].steps
      });
    }
  }
}

console.log(intersections.sort((left, right) => {
  return (Math.abs(left.x) + Math.abs(left.y)) - (Math.abs(right.x) + Math.abs(right.y));
}).map(intersection => ({...intersection, distance: Math.abs(intersection.x) + Math.abs(intersection.y)})))


console.log(
  intersections
    .sort((left, right) => {
      return left.steps - right.steps;
    })
    .map(intersection => ({
      ...intersection,
      distance: Math.abs(intersection.x) + Math.abs(intersection.y)
    }))
);