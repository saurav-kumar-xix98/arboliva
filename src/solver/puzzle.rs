use crate::grid::Grid;
use crate::constraints::constraint_set::ConstraintSet;

pub struct Puzzle {
    pub grid: Grid<Option<usize>>,
    pub constraint_set: ConstraintSet,
}
