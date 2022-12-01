use aoc_util::{Solver, SolverOutput};

#[derive(Default)]
pub struct Day13;

impl Solver for Day13 {
  fn part_one(&self, input: &str) -> SolverOutput {
    let (dots, folds) = parse_(input);

    let x_max = dots.iter().map(|&(x, _)| x).max().unwrap();
    let y_max = dots.iter().map(|&(_, y)| y).max().unwrap();

    let mut x_dim = x_max + 1;
    let mut y_dim = y_max + 1;
    let mut grid = vec![vec![false; x_max + 1]; y_max + 1];
    for &(x, y) in &dots {
      grid[y][x] = true;
    }

    let (axis, at) = folds[0];
    match axis {
      "x" => {
        for y in 0..y_dim {
          for x in 0..at {
            grid[y][x] = grid[y][x] || grid[y][2 * at - x];
          }
        }

        x_dim /= 2;
      }
      "y" => {
        for y in 0..at {
          for x in 0..x_dim {
            grid[y][x] = grid[y][x] || grid[2 * at - y][x];
          }
        }

        y_dim /= 2;
      }
      _ => panic!("invalid axis"),
    }

    let mut num_dots = 0;
    for y in 0..y_dim {
      for x in 0..x_dim {
        if grid[y][x] {
          num_dots += 1;
        }
      }
    }

    SolverOutput::Num(num_dots)
  }

  fn part_two(&self, input: &str) -> SolverOutput {
    let (dots, folds) = parse_(input);

    let x_max = dots.iter().map(|&(x, _)| x).max().unwrap();
    let y_max = dots.iter().map(|&(_, y)| y).max().unwrap();

    let mut x_dim = x_max + 1;
    let mut y_dim = y_max + 1;
    let mut grid = vec![vec![false; x_max + 1]; y_max + 1];
    for &(x, y) in &dots {
      grid[y][x] = true;
    }

    for (axis, at) in folds {
      match axis {
        "x" => {
          for y in 0..y_dim {
            for x in 0..at {
              grid[y][x] = grid[y][x] || grid[y][2 * at - x];
            }
          }

          x_dim /= 2;
        }
        "y" => {
          for y in 0..at {
            for x in 0..x_dim {
              grid[y][x] = grid[y][x] || grid[2 * at - y][x];
            }
          }

          y_dim /= 2;
        }
        _ => panic!("invalid axis"),
      }
    }

    print_grid(&grid, x_dim, y_dim);

    SolverOutput::Num(0)
  }
}

fn print_grid(grid: &Vec<Vec<bool>>, x_dim: usize, y_dim: usize) {
  for row in grid.iter().take(y_dim) {
    for &dot in row.iter().take(x_dim) {
      if dot {
        print!("#");
      } else {
        print!(".");
      }
    }
    println!("");
  }
}

fn parse_<'a>(input: &'a str) -> (Vec<(usize, usize)>, Vec<(&'a str, usize)>) {
  let (dots, folds) = input.split_once("\n\n").unwrap();
  let dots = dots
    .lines()
    .map(|line| line.split_once(",").unwrap())
    .map(|(x, y)| (x.parse().unwrap(), y.parse().unwrap()))
    .collect();
  let folds = folds
    .lines()
    .map(|line| {
      line
        .strip_prefix("fold along ")
        .unwrap()
        .split_once("=")
        .unwrap()
    })
    .map(|(axis, num)| (axis, num.parse().unwrap()))
    .collect();

  (dots, folds)
}

#[cfg(test)]
mod benches {
  use super::*;
  use aoc_util::get_input;
  use test::{black_box, Bencher};

  #[bench]
  fn parse(bencher: &mut Bencher) {
    let input = get_input(2021, 13).unwrap();
    bencher.iter(|| parse_(black_box(&input)));
  }
}
