use aoc_util::{Solver, SolverOutput};

#[derive(Default)]
pub struct Day2;

impl Solver for Day2 {
  fn part_one(&self, input: &str) -> SolverOutput {
    SolverOutput::Num(
      parse_games(input)
        .enumerate()
        .filter(|(_, game)| {
          game
            .sets
            .iter()
            .all(|set| set.red <= 12 && set.green <= 13 && set.blue <= 14)
        })
        .map(|(id, _)| (id + 1) as i64)
        .sum::<i64>(),
    )
  }

  fn part_two(&self, input: &str) -> SolverOutput {
    SolverOutput::Num(
      parse_games(input)
        .map(|game| {
          game.sets.iter().fold(Cubes::default(), |cubes, set| Cubes {
            red: cubes.red.max(set.red),
            green: cubes.green.max(set.green),
            blue: cubes.blue.max(set.blue),
          })
        })
        .map(|game| (game.red * game.green * game.blue) as i64)
        .sum::<i64>(),
    )
  }
}

#[derive(Debug)]
struct Game {
  sets: Vec<Cubes>,
}

#[derive(Debug, Default)]
struct Cubes {
  red: u64,
  green: u64,
  blue: u64,
}

fn parse_games<'a>(input: &'a str) -> impl Iterator<Item = Game> + 'a {
  input.lines().map(|line| {
    let (_, sets) = line.split_once(':').unwrap();
    let sets = sets
      .trim()
      .split(';')
      .map(|set| {
        let mut cubes = Cubes::default();
        for cube in set.trim().split(',') {
          let (amount, color) = cube.trim().split_once(' ').unwrap();
          match color {
            "red" => cubes.red = amount.parse().unwrap(),
            "green" => cubes.green = amount.parse().unwrap(),
            "blue" => cubes.blue = amount.parse().unwrap(),
            _ => unreachable!(),
          }
        }

        cubes
      })
      .collect::<Vec<_>>();
    Game { sets }
  })
}
