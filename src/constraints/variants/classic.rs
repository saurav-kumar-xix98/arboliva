use crate::constants::{REGION_SIZE, GRID_SIZE};
use crate::constraints::constraint::Constraint;
use crate::constraints::helpers;
use crate::grid::{CandidateCell, Grid, Position};

pub struct ClassicConstraint;

impl Constraint for ClassicConstraint {
    fn update(&self,
        grid: &mut Grid<CandidateCell>,
        active_positions: Grid<bool>
    ) -> Option<Grid<bool>> {
        
        println!("ClassicConstraint::update");

        helpers::update_grid_for_position(grid, active_positions, |pos| -> Vec<Position> {
            let mut positions_to_update = Vec::new();

            let row = pos.row();
            let col = pos.col();

            for r in 0..GRID_SIZE {
                let pos = Position::new(r, col);
                if r != row {
                    positions_to_update.push(pos);
                }
            }

            for c in 0..GRID_SIZE {
                let pos = Position::new(row, c);
                if c != col {
                    positions_to_update.push(pos);
                }
            }

            let region_row = (row / REGION_SIZE) * REGION_SIZE;
            let region_col = (col / REGION_SIZE) * REGION_SIZE;
            for r in region_row..region_row + REGION_SIZE {
                for c in region_col..region_col + REGION_SIZE {
                    let pos = Position::new(r, c);
                    if r != row && c != col {
                        positions_to_update.push(pos);
                    }
                }
            }
            
            positions_to_update
            
        })
    }
}
