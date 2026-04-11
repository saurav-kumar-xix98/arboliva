    use crate::constants::GRID_SIZE;
    use crate::constraints::constraint::Constraint;
    use crate::constraints::helpers;
    use crate::grid::{CandidateCell, Grid, Position};

    pub struct AntiKnightConstraint;

    impl Constraint for AntiKnightConstraint {
        fn update(&self, grid: &mut Grid<CandidateCell>, active_positions: Grid<bool>) -> Option<Grid<bool>> {

            println!("AntiKnightConstraint::update");

            helpers::update_grid_for_position(grid, active_positions, |pos| -> Vec<Position> {
                let mut positions_to_update = Vec::new();

                let row = pos.row();
                let col = pos.col();

                const OFFSETS: [(i8, i8); 8] = [
                    (-2, -1), (-2, 1),
                    (-1, -2), (-1, 2),
                    (1, -2),  (1, 2),
                    (2, -1),  (2, 1),
                ];

                for (dr, dc) in OFFSETS {
                    let new_row = row as i8 + dr;
                    let new_col = col as i8 + dc;

                    if new_row < 0 || new_col < 0 {
                        continue;
                    }
                    let new_row = new_row as u8;
                    let new_col = new_col as u8;
                    if new_row >= GRID_SIZE || new_col >= GRID_SIZE {
                        continue;
                    }
                    positions_to_update.push(Position::new(new_row, new_col));
                }

                positions_to_update
            })

        }
    }
