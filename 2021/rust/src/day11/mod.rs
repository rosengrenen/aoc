use aoc_util::{Solver, SolverOutput};

#[derive(Default)]
pub struct Day11;

impl Solver for Day11 {
  fn part_one(&self, input: &str) -> SolverOutput {
    let mut grid = parse_(input);

    let mut flashes = 0;
    for _ in 0..100 {
      let mut flashed = [[false; 10]; 10];

      let mut tiles_to_check = Vec::with_capacity(100);
      for y in 0..10 {
        for x in 0..10 {
          grid[y][x] += 1;
          if grid[y][x] > 9 {
            tiles_to_check.push((x, y));
          }
        }
      }

      while let Some((x, y)) = tiles_to_check.pop() {
        if flashed[y][x] {
          continue;
        }

        flashed[y][x] = true;
        for dy in -1..=1 {
          for dx in -1..=1 {
            if dx == 0 && dy == 0 {
              continue;
            }

            let cx = x as isize + dx;
            let cy = y as isize + dy;

            if cx < 0 || cx >= 10 || cy < 0 || cy >= 10 {
              continue;
            }

            let cx = cx as usize;
            let cy = cy as usize;

            grid[cy][cx] += 1;
            if grid[cy][cx] > 9 {
              tiles_to_check.push((cx, cy));
            }
          }
        }
      }

      for y in 0..10 {
        for x in 0..10 {
          if flashed[y][x] {
            grid[y][x] = 0;
            flashes += 1;
          }
        }
      }
    }

    for y in 0..10 {
      for x in 0..10 {
        print!("{}", grid[y][x]);
      }
      println!("");
    }

    SolverOutput::Num(flashes)
  }

  fn part_two(&self, input: &str) -> SolverOutput {
    let mut grid = parse_(input);

    let mut flashes = 0;
    let mut iteration = 0;
    loop {
      iteration += 1;
      let mut flashed = [[false; 10]; 10];

      let mut tiles_to_check = Vec::with_capacity(100);
      for y in 0..10 {
        for x in 0..10 {
          grid[y][x] += 1;
          if grid[y][x] > 9 {
            tiles_to_check.push((x, y));
          }
        }
      }

      while let Some((x, y)) = tiles_to_check.pop() {
        if flashed[y][x] {
          continue;
        }

        flashed[y][x] = true;
        for dy in -1..=1 {
          for dx in -1..=1 {
            if dx == 0 && dy == 0 {
              continue;
            }

            let cx = x as isize + dx;
            let cy = y as isize + dy;

            if cx < 0 || cx >= 10 || cy < 0 || cy >= 10 {
              continue;
            }

            let cx = cx as usize;
            let cy = cy as usize;

            grid[cy][cx] += 1;
            if grid[cy][cx] > 9 {
              tiles_to_check.push((cx, cy));
            }
          }
        }
      }

      let mut all_flash = true;
      'outer: for y in 0..10 {
        for x in 0..10 {
          if flashed[y][x] {
            grid[y][x] = 0;
          } else {
            all_flash = false;
          }
        }
      }

      if all_flash {
        break;
      }
    }

    for y in 0..10 {
      for x in 0..10 {
        print!("{}", grid[y][x]);
      }
      println!("");
    }

    SolverOutput::Num(iteration)
  }
}

fn parse_(input: &str) -> [[u8; 10]; 10] {
  let mut grid = [[0; 10]; 10];
  for (y, line) in input.lines().enumerate().take(10) {
    for (x, b) in line.bytes().enumerate().take(10) {
      grid[y][x] = b - b'0';
    }
  }

  grid
}

#[cfg(test)]
mod benches {
  use super::*;
  use aoc_util::get_input;
  use test::{black_box, Bencher};

  #[bench]
  fn parse(bencher: &mut Bencher) {
    let input = get_input(2021, 11).unwrap();
    bencher.iter(|| parse_(black_box(&input)));
  }
}
