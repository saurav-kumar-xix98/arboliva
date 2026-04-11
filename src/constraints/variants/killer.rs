use crate::constants::GRID_SIZE;
use crate::constraints::constraint::Constraint;
use crate::grid::{CandidateCell, Grid, Position};

pub struct Cage {
    pub sum: u8,
    pub positions: Vec<Position>,
}

pub struct KillerConstraint {
    cages: Vec<Cage>,
    cage_indices: Grid<Vec<usize>>,
}

impl KillerConstraint {
    pub fn new(cages: Vec<Cage>) -> KillerConstraint {
        let mut cage_indices = Grid::from_default(vec![]);

        for i in 0..cages.len() {
            for pos in &cages[i].positions {
                cage_indices[*pos].push(i);
            }
        }

        Self { cages, cage_indices }
    }
}

fn recursive_solve(grid: &mut Grid<CandidateCell>,
                   cage_positions: &Vec<Position>,
                   updated_candidates: &mut Vec<[bool; GRID_SIZE as usize]>,
                   values_used: &mut [bool; GRID_SIZE as usize],
                   index: usize,
                   target_sum: u8
) -> bool {
    if index == cage_positions.len() {
        return target_sum == 0;
    }

    let mut is_possible = false;

    let pos = cage_positions[index];
    for i in 0..GRID_SIZE as usize {
        let val = (i + 1) as u8;
        if val > target_sum {
            break;
        }
        if values_used[i] || !grid[pos].is_valid(val) {
            continue;
        }

        values_used[i] = true;
        if recursive_solve(grid, cage_positions, updated_candidates, values_used, index + 1, target_sum - val) {
            updated_candidates[index][i] = true;
            is_possible = true;
        }
        values_used[i] = false;
    }

    is_possible
}

impl Constraint for KillerConstraint {
    fn update(&self, grid: &mut Grid<CandidateCell>, active_positions: Grid<bool>) -> Option<Grid<bool>> {
        println!("KillerConstraint::update");

        let mut is_cage_active = vec![false; self.cages.len()];

        for row in 0..GRID_SIZE {
            for col in 0..GRID_SIZE {
                let pos = Position::new(row, col);
                if active_positions[pos] {
                    for cage_index in &self.cage_indices[pos] {
                        is_cage_active[*cage_index] = true;
                    }
                }
            }
        }

        let mut affected_positions = Grid::from_default(false);

        for i in 0..is_cage_active.len() {
            if !is_cage_active[i] {
                continue;
            }
            let cage = &self.cages[i];
            let cage_size = cage.positions.len();
            let mut values_used = [false; GRID_SIZE as usize];
            let mut updated_candidates = vec![[false; GRID_SIZE as usize]; cage_size];

            if !recursive_solve(grid, &cage.positions, &mut updated_candidates, &mut values_used, 0, cage.sum) {
                return None;
            }

            for i in 0..cage_size {
                for val in 1..=GRID_SIZE {
                    let pos = cage.positions[i];
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
