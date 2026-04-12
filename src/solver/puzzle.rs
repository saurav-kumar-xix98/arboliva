use crate::grid::Grid;
use crate::constraints::constraint_set::ConstraintSet;
use serde_yaml::Value;

pub struct Puzzle {
    pub grid: Grid<Option<usize>>,
    pub constraint_set: ConstraintSet,
}

impl Puzzle {
    pub fn from_yaml(value: &Value) -> Result<Self, String> {
        let grid_value = value.get("grid")
            .ok_or_else(|| "missing 'grid' field".to_string())?;
        let grid = Grid::grid_from_yaml(grid_value)?;

        let constraint_vec = value.get("constraint_set")
            .ok_or_else(|| "missing 'constraint_set' field".to_string())?.as_sequence()
            .ok_or_else(|| "'constraint_set' must be a sequence".to_string())?
            .clone();

        let constraint_set = ConstraintSet::from_yaml_values(constraint_vec)?;

        Ok(Puzzle { grid, constraint_set })
    }
}
