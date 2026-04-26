use crate::model::{CandidateCell, Grid, Position, Puzzle};
use crate::solver::constraints::constraint_set::ConstraintSet;

fn to_candidate_grid(puzzle: Grid<Option<u8>>) -> Grid<CandidateCell> {
    puzzle.map(|cell| match cell {
        Some(value) => CandidateCell::with_value(*value),
        None => CandidateCell::with_count(puzzle.size())
    })
}

fn from_candidate_grid(candidate_grid: Grid<CandidateCell>) -> Grid<u8> {
    candidate_grid.map(|cell| match cell.fixed_value() {
        Some(value) => value,
        None => panic!("CandidateCell is not solved")
    })
}

fn find_best_candidate(grid: &Grid<CandidateCell>) -> Option<Position> {
    let mut best_position: Option<Position> = None;
    let mut best_count = grid.size() + 1;

    for row in 0..grid.size() {
        for col in 0..grid.size() {
            let pos = Position{row, col};

            let count = grid[pos].len();
            if (count < best_count) && (count > 1) {
                best_count = count;
                best_position = Some(pos);
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

    let position = match find_best_candidate(grid) {
        Some(pos) => pos,
        None => return true,
    };

    for value in 1..=grid.size() {
        if !grid[position].contains(value) {
            continue;
        }

        let mut new_grid = grid.clone();
        new_grid[position] = CandidateCell::with_value(value);
        let mut active_positions = grid.map(|_| false);
        active_positions[position] = true;

        println!("Guessing value {} at {}", value, position);

        if solve_recursive(&mut new_grid, constraint_set, active_positions) {
            *grid = new_grid;
            return true;
        }
    }

    false
}

pub fn solve(puzzle: Puzzle) -> Option<Grid<u8>> {
    let active_positions = puzzle.grid.map(|_| true);
    let mut candidate_grid = to_candidate_grid(puzzle.grid);

    println!("{}", candidate_grid);

    if solve_recursive(&mut candidate_grid, &puzzle.constraint_set, active_positions) {
        Some(from_candidate_grid(candidate_grid))
    } else {
        None
    }
}
