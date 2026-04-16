use crate::grid::Grid;
use crate::constraints::constraint_set::ConstraintSet;

pub struct Puzzle {
    pub grid: Grid<Option<u8>>,
    pub constraint_set: ConstraintSet,
}
