use crate::model::{CandidateGrid, Grid, Position};

pub fn is_any_true(grid: &Grid<bool>) -> bool {
    for row in 0..grid.size() {
        for col in 0..grid.size() {
            let pos = Position{row, col};
            if grid[pos] {
                return true;
            }
        }
    }
    false
}

pub fn accumulate(active_positions: &Grid<bool>, accumulated: &mut Grid<bool>) {
    for row in 0..active_positions.size() {
        for col in 0..active_positions.size() {
            let pos = Position{row, col};
            if active_positions[pos] {
                accumulated[pos] = true;
            }
        }
    }
}

pub fn update_grid_for_position<F>(
    grid: &mut CandidateGrid,
    active_positions: Grid<bool>,
    get_positions_to_update: F,
) -> Option<Grid<bool>>
where F: Fn(Position) -> Vec<Position> {

    let mut affected_positions = Grid::from_default(grid.region_shape(), false);

    for row in 0..grid.size() {
        for col in 0..grid.size() {
            let pos = Position{row, col};
            if !active_positions[pos] {
                continue;
            }
            let value = match grid[pos].fixed_value() {
                Some(val) => val,
                None => continue,
            };

            let positions_to_update = get_positions_to_update(pos);
            for position in positions_to_update {
                if !grid[position].contains(value) {
                    continue;
                }
                println!("Removing {} from {}", value, position);
                grid[position].remove(value);
                if grid[position].len() == 0 {
                    println!("Candidate Cell is empty now!");
                    return None;
                }
                affected_positions[position] = true;
            }
        }
    }

    Some(affected_positions)
}
