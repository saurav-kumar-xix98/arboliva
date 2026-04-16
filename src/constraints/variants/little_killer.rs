use crate::constraints::constraint::Constraint;
use crate::grid::{CandidateCell, Grid, Position};
use crate::grid::grid::RegionShape;

pub enum Direction {
    DownRight, DownLeft, UpRight, UpLeft,
}

pub struct Diagonal {
    pub sum: u16,
    pub positions: Vec<Position>,
}

pub struct LittleKillerConstraint {
    diagonals: Vec<Diagonal>,
    diagonal_indices: Grid<Vec<usize>>,
}

impl Diagonal {
    pub fn new (sum: u16, direction: Direction, first_position: Position, grid_size: u8) -> Diagonal {
        let mut row = first_position.row;
        let mut col = first_position.col;
        match direction {
            Direction::DownRight => {
                assert!(row == 0 || col == 0, "");
            }
            Direction::DownLeft => {
                assert!(row == 0 || col == grid_size - 1, "");
            }
            Direction::UpRight => {
                assert!(row == grid_size - 1 || col == 0, "");
            }
            Direction::UpLeft => {
                assert!(row == grid_size - 1 || col == grid_size - 1, "");
            }
        }
        let mut positions = Vec::new();

        loop {
            positions.push(Position{row, col});
            match direction {
                Direction::DownRight | Direction::DownLeft => {
                    if row == grid_size - 1 {
                        break;
                    }
                    row += 1;
                }
                Direction::UpRight | Direction::UpLeft => {
                    if row == 0 {
                        break;
                    }
                    row -= 1;
                }
            }
            match direction {
                Direction::DownRight | Direction::UpRight => {
                    if col == grid_size - 1 {
                        break;
                    }
                    col += 1;
                }
                Direction::DownLeft | Direction::UpLeft => {
                    if col == 0 {
                        break;
                    }
                    col -= 1;
                }
            }
        }

        Self {sum, positions}
    }
}

impl LittleKillerConstraint {
    pub fn new(diagonal: Vec<Diagonal>, region_rows: u8, region_cols: u8) -> Self {
        let mut diagonal_indices = Grid::from_default(RegionShape{ region_rows, region_cols }, vec![]);

        for i in 0..diagonal.len() {
            for pos in &diagonal[i].positions {
                diagonal_indices[*pos].push(i);
            }
        }

        Self { diagonals: diagonal, diagonal_indices }
    }
}

impl Constraint for LittleKillerConstraint {
    fn update(&self, grid: &mut Grid<CandidateCell>, active_positions: Grid<bool>) -> Option<Grid<bool>> {
        println!("LittleKillerConstraint::update");

        let mut is_diagonal_active = vec![false; self.diagonals.len()];

        for row in 0..grid.size() {
            for col in 0..grid.size() {
                let pos = Position{row, col};
                if active_positions[pos] {
                    for diagonal_index in &self.diagonal_indices[pos] {
                        is_diagonal_active[*diagonal_index] = true;
                    }
                }
            }
        }

        let mut affected_positions = grid.map(|_| false);

        for i in 0..is_diagonal_active.len() {
            if !is_diagonal_active[i] {
                continue;
            }
            let diagonal = &self.diagonals[i];
            let diagonal_size = diagonal.positions.len();
            let grid_size = grid.size() as usize;
            let mut values_used_in_box = vec![vec![false; grid_size]; grid_size];
            let mut updated_candidates = vec![vec![false; grid_size]; diagonal_size];

            if !recursive_solve(grid, &diagonal.positions, &mut updated_candidates, &mut values_used_in_box, 0, diagonal.sum) {
                return None;
            }

            for i in 0..diagonal_size {
                for val in 1..=grid.size() {
                    let pos = diagonal.positions[i];
                    if !updated_candidates[i][val as usize - 1] && grid[pos].contains(val) {
                        println!("Removing {} from {}", val, pos);
                        grid[pos].remove(val);
                        affected_positions[pos] = true;
                    }
                }
            }
        }

        Some(affected_positions)
    }
}

fn recursive_solve(grid: &mut Grid<CandidateCell>,
                   diagonal_positions: &Vec<Position>,
                   updated_candidates: &mut Vec<Vec<bool>>,
                   values_used_in_box: &mut Vec<Vec<bool>>,
                   index: usize,
                   target_sum: u16
) -> bool {
    if index == diagonal_positions.len() {
        return target_sum == 0;
    }

    let mut is_possible = false;

    let pos = diagonal_positions[index];
    let region = ((pos.row / grid.region_rows()) * grid.region_rows() + pos.col / grid.region_cols()) as usize;
    for i in 0..grid.size() as usize {
        let val = i as u8 + 1;
        if val as u16 > target_sum {
            break;
        }
        if values_used_in_box[region][i] || !grid[pos].contains(val) {
            continue;
        }

        values_used_in_box[region][i] = true;
        if recursive_solve(grid, diagonal_positions, updated_candidates, values_used_in_box, index + 1, target_sum - val as u16) {
            updated_candidates[index][i] = true;
            is_possible = true;
        }
        values_used_in_box[region][i] = false;
    }

    is_possible
}
