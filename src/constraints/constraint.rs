use crate::constraints::helpers;
use crate::grid::{CandidateCell, Grid};

pub trait Constraint {
    fn update(&self, grid: &mut Grid<CandidateCell>, active_positions: Grid<bool>) -> Option<Grid<bool>>;
    fn update_recursive(&self,
        grid: &mut Grid<CandidateCell>,
        mut active_positions: Grid<bool>
    ) -> Option<Grid<bool>> {
        
        let mut affected_positions  = grid.map(|_| false);

        while helpers::is_any_true(&active_positions) {
            let newly_affected_positions = self.update(grid, active_positions)?;
            helpers::accumulate(&newly_affected_positions, &mut affected_positions);
            active_positions = newly_affected_positions;

            println!("{}", grid);
        }

        Some(affected_positions)
    }
}
