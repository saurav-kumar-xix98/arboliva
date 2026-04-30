use crate::model::SolverState;

pub enum SolverResult {
    NoSolution,
    Solution(SolverState),
    MultipleSolution,
}
