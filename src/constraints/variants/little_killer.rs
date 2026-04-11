use crate::constants::GRID_SIZE;
use crate::constraints::constraint::Constraint;
use crate::grid::{CandidateCell, Grid, Position};

pub enum Direction {
    DownRight, DownLeft, UpRight, UpLeft,
}

pub struct Diagonal {
    pub sum: u8,
    pub positions: Vec<Position>,
}

pub struct LittleKillerConstraint {
    diagonals: Vec<Diagonal>,
    diagonal_indices: Grid<Vec<usize>>,
}

impl Diagonal {
    pub fn new (sum: u8, direction: Direction, first_position: Position) -> Diagonal {
        let mut row = first_position.row();
        let mut col = first_position.col();
        match direction {
            Direction::DownRight => {
                assert!(row == 0 || col == 0, "");
            }
            Direction::DownLeft => {
                assert!(row == 0 || col == GRID_SIZE - 1, "");
            }
            Direction::UpRight => {
                assert!(row == GRID_SIZE - 1 || col == 0, "");
            }
            Direction::UpLeft => {
                assert!(row == GRID_SIZE - 1 || col == GRID_SIZE - 1, "");
            }
        }
        let mut positions = Vec::new();

        loop {
            positions.push(Position::new(row, col));
            match direction {
                Direction::DownRight | Direction::DownLeft => {
                    if row == GRID_SIZE - 1 {
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
                    if col == GRID_SIZE - 1 {
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
    pub fn new(diagonal: Vec<Diagonal>) -> Self {
        let mut diagonal_indices = Grid::from_default(vec![]);

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

        for row in 0..GRID_SIZE {
            for col in 0..GRID_SIZE {
                let pos = Position::new(row, col);
                if active_positions[pos] {
                    for diagonal_index in &self.diagonal_indices[pos] {
                        is_diagonal_active[*diagonal_index] = true;
                    }
                }
            }
        }

        let mut affected_positions = Grid::from_default(false);

        for i in 0..is_diagonal_active.len() {
            if !is_diagonal_active[i] {
                continue;
            }
            let diagonal = &self.diagonals[i];
            let diagonal_size = diagonal.positions.len();
            let mut values_used = [[false; GRID_SIZE as usize]; GRID_SIZE as usize];
            let mut updated_candidates = vec![[false; GRID_SIZE as usize]; diagonal_size];

            if !recursive_solve(grid, &diagonal.positions, &mut updated_candidates, &mut values_used, 0, diagonal.sum) {
                return None;
            }

            for i in 0..diagonal_size {
                for val in 1..=GRID_SIZE {
                    let pos = diagonal.positions[i];
                    if !updated_candidates[i][(val - 1) as usize] && grid[pos].is_valid(val) {
                        grid[pos].remove_candidate(val);
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
                   updated_candidates: &mut Vec<[bool; GRID_SIZE as usize]>,
                   values_used: &mut [[bool; GRID_SIZE as usize]; GRID_SIZE as usize],
                   index: usize,
                   target_sum: u8
) -> bool {
    if index == diagonal_positions.len() {
        return target_sum == 0;
    }

    let mut is_possible = false;

    let pos = diagonal_positions[index];
    let region = pos.region() as usize;
    for i in 0..GRID_SIZE as usize {
        let val = (i + 1) as u8;
        if val > target_sum {
            break;
        }
        if values_used[region][i] || !grid[pos].is_valid(val) {
            continue;
        }

        values_used[region][i] = true;
        if recursive_solve(grid, diagonal_positions, updated_candidates, values_used, index + 1, target_sum - val) {
            updated_candidates[index][i] = true;
            is_possible = true;
        }
        values_used[region][i] = false;
    }

    is_possible
}
