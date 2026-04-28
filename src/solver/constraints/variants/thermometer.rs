use crate::model::{CandidateGrid, Grid, Position, RegionShape};
use crate::solver::constraints::constraint::Constraint;

pub struct Thermometer {
    pub positions: Vec<Position>,
}

pub struct ThermometerConstraint {
    thermometers: Vec<Thermometer>,
    thermometer_indices: Grid<Vec<usize>>,
}

impl ThermometerConstraint {
    pub fn new(thermometers : Vec<Thermometer>, region_rows: u8, region_cols: u8) -> ThermometerConstraint {
        let mut thermometer_indices = Grid::from_default(RegionShape{ region_rows, region_cols }, vec![]);

        for i in 0..thermometers.len() {
            for pos in &thermometers[i].positions {
                thermometer_indices[*pos].push(i);
            }
        }

        Self { thermometers, thermometer_indices }
    }
}

impl Constraint for ThermometerConstraint {
    fn update(&self, grid: &mut CandidateGrid, active_positions: Grid<bool>) -> Option<Grid<bool>> {
        println!("ThermometerConstraint::update");

        let mut is_thermometer_active = vec![false; self.thermometers.len()];

        for row in 0..grid.size() {
            for col in 0..grid.size() {
                let pos = Position{row, col};
                if active_positions[pos] {
                    for thermometer_index in &self.thermometer_indices[pos] {
                        is_thermometer_active[*thermometer_index] = true;
                    }
                }
            }
        }

        let mut affected_positions = Grid::from_default(grid.region_shape(), false);

        for i in 0..is_thermometer_active.len() {
            if !is_thermometer_active[i] {
                continue;
            }

            let thermometer = &self.thermometers[i];
            let thermometer_size = thermometer.positions.len();
            let mut updated_candidates = vec![vec![false; grid.size() as usize]; thermometer_size];

            if !recursive_solve(grid, &thermometer.positions, &mut updated_candidates, 0, 1) {
                print!("Thermometer cannot be solved: ");
                for pos in &thermometer.positions {
                    print!("{} ", pos);
                }
                println!();
                return None;
            }

            for i in 0..thermometer_size {
                for val in 1..=grid.size() {
                    let pos = thermometer.positions[i];
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

fn recursive_solve(grid: &mut CandidateGrid,
                   thermometer_positions: &Vec<Position>,
                   updated_candidates: &mut Vec<Vec<bool>>,
                   index: usize,
                   lower_limit: u8
) -> bool {
    if index == thermometer_positions.len() {
        return true;
    }

    let mut is_possible = false;

    let pos = thermometer_positions[index];
    for val in lower_limit..=grid.size() {
        let i = val as usize - 1;
        if i + thermometer_positions.len() - index > grid.size() as usize {
            break;
        }
        if !grid[pos].contains(val) {
            continue;
        }

        if recursive_solve(grid, thermometer_positions, updated_candidates, index + 1, val + 1) {
            updated_candidates[index][i] = true;
            is_possible = true;
        }
    }

    is_possible
}
