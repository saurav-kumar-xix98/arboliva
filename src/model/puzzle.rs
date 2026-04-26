use crate::model::Grid;
use crate::solver::constraints::constraint_set::ConstraintSet;

pub struct Puzzle {
    pub grid: Grid<Option<u8>>,
    pub constraint_set: ConstraintSet,
}
