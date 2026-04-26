use crate::model::{CandidateCell, Grid, Position};
use crate::solver::constraints::constraint::Constraint;
use crate::solver::constraints::helpers;

pub struct ClassicConstraint;

impl Constraint for ClassicConstraint {
    fn update(&self,
        grid: &mut Grid<CandidateCell>,
        active_positions: Grid<bool>
    ) -> Option<Grid<bool>> {

        println!("ClassicConstraint::update");

        let grid_size = grid.size();
        let region_rows = grid.region_rows();
        let region_cols = grid.region_cols();

        helpers::update_grid_for_position(grid, active_positions, |pos| -> Vec<Position> {
            let mut positions_to_update = Vec::new();

            let active_row = pos.row;
            let active_col = pos.col;

            for row in 0..grid_size {
                let pos = Position{row, col: active_col };
                if row != active_row {
                    positions_to_update.push(pos);
                }
            }

            for col in 0..grid_size {
                let pos = Position{ row: active_row, col };
                if col != active_col {
                    positions_to_update.push(pos);
                }
            }

            let region_row = (active_row / region_rows) * region_rows;
            let region_col = (active_col / region_cols) * region_cols;
            for row in region_row..region_row + region_rows {
                for col in region_col..region_col + region_cols {
                    let pos = Position{row, col };
                    if row != active_row && col != active_col {
                        positions_to_update.push(pos);
                    }
                }
            }

            positions_to_update

        })
    }
}
