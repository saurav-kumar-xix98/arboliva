use crate::constants::GRID_SIZE;
use crate::constraints::constraint_set::ConstraintSet;
use crate::grid::{CandidateCell, Grid, Position};
use crate::solver::puzzle::Puzzle;

fn to_candidate_grid(puzzle: Grid<Option<usize>>) -> Grid<CandidateCell> {
    let mut candidate_grid = Grid::from_default(CandidateCell::new());

    for row in 0..GRID_SIZE {
        for col in 0..GRID_SIZE {
            let pos = Position{row, col};
            candidate_grid[pos] = match puzzle[pos] {
                Some(value) => CandidateCell::FixedValue(value),
                None => CandidateCell::new()
            }
        }
    }

    candidate_grid
}

fn from_candidate_grid(candidate_grid: Grid<CandidateCell>) -> Grid<usize> {
    let mut solution = Grid::from_default(0);

    for row in 0..GRID_SIZE {
        for col in 0..GRID_SIZE {
            let pos = Position{row, col};
            solution[pos] = match candidate_grid[pos] {
                CandidateCell::FixedValue(value) => value,
                _ => panic!("CandidateCell is not fixed")
            }
        }
    }

    solution
}

fn find_best_candidate(grid: &Grid<CandidateCell>) -> Option<Position> {
    let mut best_position: Option<Position> = None;
    let mut best_count = GRID_SIZE + 1;

    println!("Best count: {}", best_count);

    for row in 0..GRID_SIZE {
        for col in 0..GRID_SIZE {
            let pos = Position{row, col};
            let cell = grid[pos];

            let count = cell.len();

            println!("Position {} Count {}", pos, count);
            if (count < best_count) && (count > 1) {
                best_count = count;
                best_position = Some(pos);
                println!("Found new best");
            }
        }
    }

    best_position
}

fn solve_recursive(grid: &mut Grid<CandidateCell>,
    constraint_set: &ConstraintSet,
    active_positions: Grid<bool>
) -> bool {

    if !constraint_set.update(grid, active_positions) {
        return false;
    }

    println!("Finding best candidate");

    let position = match find_best_candidate(grid) {
        Some(pos) => pos,
        None => return true,
    };

    println!("Best candidate: {}", position);

    for value in 1..=GRID_SIZE {
        if !grid[position].contains(value) {
            continue;
        }

        let mut new_grid = grid.clone();
        new_grid[position] = CandidateCell::FixedValue(value);
        let mut active_positions = Grid::from_default(false);
        active_positions[position] = true;

        println!("Guessing value {} at {}", value, position);

        if solve_recursive(&mut new_grid, constraint_set, active_positions) {
            *grid = new_grid;
            return true;
        }
    }

    false
}

pub fn solve(puzzle: Puzzle) -> Option<Grid<usize>> {
    let mut candidate_grid = to_candidate_grid(puzzle.grid);
    let active_positions = Grid::from_default(true);

    println!("{}", candidate_grid);

    if solve_recursive(&mut candidate_grid, &puzzle.constraint_set, active_positions) {
        Some(from_candidate_grid(candidate_grid))
    } else {
        None
    }
}
