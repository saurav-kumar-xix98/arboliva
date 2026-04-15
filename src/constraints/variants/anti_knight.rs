use crate::constraints::constraint::Constraint;
use crate::constraints::helpers;
use crate::grid::{CandidateCell, Grid, Position};

pub struct AntiKnightConstraint;

impl Constraint for AntiKnightConstraint {
    fn update(&self, grid: &mut Grid<CandidateCell>, active_positions: Grid<bool>) -> Option<Grid<bool>> {
        println!("AntiKnightConstraint::update");

        let grid_size = grid.grid_size();

        helpers::update_grid_for_position(grid, active_positions, |pos| -> Vec<Position> {
            let mut positions_to_update = Vec::new();

            const OFFSETS: [(isize, isize); 8] = [
                (-2, -1), (-2, 1), (-1, -2), (-1, 2), (1, -2), (1, 2), (2, -1), (2, 1),
            ];

            for (dr, dc) in OFFSETS {
                let row = pos.row as isize + dr;
                let col = pos.col as isize + dc;

                if row < 0 || col < 0 {
                    continue;
                }
                let row = row as usize;
                let col = col as usize;
                if row >= grid_size || col >= grid_size {
                    continue;
                }
                positions_to_update.push(Position{row, col});
            }

            positions_to_update
        })

    }
}
