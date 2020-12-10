pub enum SolverResult {
	Num(i64),
	// Text(String),
}

pub trait Solver {
	fn solve_part_one(&self, input: &str) -> SolverResult;
	fn solve_part_two(&self, input: &str) -> SolverResult;
}
