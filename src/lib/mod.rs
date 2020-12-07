pub trait Solver {
	fn solve(&self, input: &str, part_two: bool) -> i64;
}
