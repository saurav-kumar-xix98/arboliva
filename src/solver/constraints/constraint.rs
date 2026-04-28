use crate::model::{CandidateGrid, Grid};
use crate::solver::constraints::helpers;

pub trait Constraint {
    fn update(&self, grid: &mut CandidateGrid, active_positions: Grid<bool>) -> Option<Grid<bool>>;
    fn update_recursive(&self,
        grid: &mut CandidateGrid,
        mut active_positions: Grid<bool>
    ) -> Option<Grid<bool>> {

        let mut affected_positions  = Grid::from_default(grid.region_shape(), false);

        while helpers::is_any_true(&active_positions) {
            let newly_affected_positions = self.update(grid, active_positions)?;
            helpers::accumulate(&newly_affected_positions, &mut affected_positions);
            active_positions = newly_affected_positions;

            println!("{}", grid);
        }

        Some(affected_positions)
    }
}
