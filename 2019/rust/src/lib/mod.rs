pub trait Solver {
  fn solve(&self, lines: Vec<String>, part_two: bool) -> String;
}
