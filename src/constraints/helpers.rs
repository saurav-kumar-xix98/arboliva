use crate::constants::GRID_SIZE;
use crate::grid::{CandidateCell, Grid, Position};
use crate::grid::CandidateCell::Fixed;

pub fn is_any_true(grid: &Grid<bool>) -> bool {
    for row in 0..GRID_SIZE {
        for col in 0..GRID_SIZE {
            let pos = Position::new(row, col);
            if grid[pos] {
                return true;
            }
        }
    }
    false
}

pub fn increment_count(counter: &mut Grid<u8>, active_positions: &Grid<bool>) {
    for row in 0..GRID_SIZE {
        for col in 0..GRID_SIZE {
            let pos = Position::new(row, col);
            if active_positions[pos] {
                counter[pos] += 1;
            }
        }
    }
}

pub fn decrement_count(counter: &mut Grid<u8>, active_positions: &Grid<bool>) {
    for row in 0..GRID_SIZE {
        for col in 0..GRID_SIZE {
            let pos = Position::new(row, col);
            if active_positions[pos] {
                counter[pos] -= 1;
            }
        }
    }
}

pub fn aggregate(counter: &Grid<u8>) -> Grid<bool> {
    let mut result = Grid::from_default(false);
    for row in 0..GRID_SIZE {
        for col in 0..GRID_SIZE {
            let pos = Position::new(row, col);
            if counter[pos] > 0 {
                result[pos] = true;
            }
        }
    }
    result
}

pub fn accumulate(active_positions: &Grid<bool>, accumulated: &mut Grid<bool>) {
    for row in 0..GRID_SIZE {
        for col in 0..GRID_SIZE {
            let pos = Position::new(row, col);
            if active_positions[pos] {
                accumulated[pos] = true;
            }
        }
    }
}

pub fn update_grid_for_position<F>(
    grid: &mut Grid<CandidateCell>,
    active_positions: Grid<bool>,
    get_positions_to_update: F,
) -> Option<Grid<bool>>
where F: Fn(Position) -> Vec<Position> {

    let mut affected_positions = Grid::from_default(false);
    
    for row in 0..GRID_SIZE {
        for col in 0..GRID_SIZE {
            let pos = Position::new(row, col);
            if !active_positions[pos] {
                continue;
            }
            let value = match grid[pos] { 
                Fixed(val) => val,
                _ => continue,
            };
            
            let positions_to_update = get_positions_to_update(pos);
            for position in positions_to_update {
                if !grid[position].is_valid(value) {
                    continue;
                }
                if grid[position].is_fixed() {
                    println!("Cannot remove {} from {}. exiting", value, position);
                    return None;
                }
                println!("Removing {} from {}", value, position);
                grid[position].remove_candidate(value);
                affected_positions[position] = true;
            }
        }
    }

    Some(affected_positions)
}
