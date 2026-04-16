use crate::constraints::constraint::Constraint;
use crate::grid::{CandidateCell, Grid, Position};
use crate::grid::grid::RegionShape;

pub struct Cage {
    pub sum: u16,
    pub positions: Vec<Position>,
}

pub struct KillerConstraint {
    cages: Vec<Cage>,
    cage_indices: Grid<Vec<usize>>,
}

impl KillerConstraint {
    pub fn new(cages: Vec<Cage>, region_rows: u8, region_cols: u8) -> KillerConstraint {
        let mut cage_indices = Grid::from_default(RegionShape{ region_rows, region_cols }, vec![]);

        for i in 0..cages.len() {
            for pos in &cages[i].positions {
                cage_indices[*pos].push(i);
            }
        }

        Self { cages, cage_indices }
    }
}

impl Constraint for KillerConstraint {
    fn update(&self, grid: &mut Grid<CandidateCell>, active_positions: Grid<bool>) -> Option<Grid<bool>> {
        println!("KillerConstraint::update");

        let mut is_cage_active = vec![false; self.cages.len()];

        for row in 0..grid.size() {
            for col in 0..grid.size() {
                let pos = Position{row, col};
                if active_positions[pos] {
                    for cage_index in &self.cage_indices[pos] {
                        is_cage_active[*cage_index] = true;
                    }
                }
            }
        }

        let mut affected_positions = grid.map(|_| false);

        for i in 0..is_cage_active.len() {
            if !is_cage_active[i] {
                continue;
            }
            let cage = &self.cages[i];
            let cage_size = cage.positions.len();
            let mut values_used = vec![false; grid.size() as usize];
            let mut updated_candidates = vec![vec![false; grid.size() as usize]; cage_size];

            if !recursive_solve(grid, &cage.positions, &mut updated_candidates, &mut values_used, 0, cage.sum) {
                return None;
            }

            for i in 0..cage_size {
                for val in 1..=grid.size() {
                    let pos = cage.positions[i];
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
                   cage_positions: &Vec<Position>,
                   updated_candidates: &mut Vec<Vec<bool>>,
                   values_used: &mut Vec<bool>,
                   index: usize,
                   target_sum: u16
) -> bool {
    if index == cage_positions.len() {
        return target_sum == 0;
    }

    let mut is_possible = false;

    let pos = cage_positions[index];
    for i in 0..grid.size() as usize {
        let val = i as u8 + 1;
        if val as u16 > target_sum {
            break;
        }
        if values_used[i] || !grid[pos].contains(val) {
            continue;
        }

        values_used[i] = true;
        if recursive_solve(grid, cage_positions, updated_candidates, values_used, index + 1, target_sum - val as u16) {
            updated_candidates[index][i] = true;
            is_possible = true;
        }
        values_used[i] = false;
    }

    is_possible
}
