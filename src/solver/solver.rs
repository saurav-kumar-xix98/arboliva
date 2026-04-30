use crate::model::{CandidateCell, CandidateGrid, Clue, ClueType, Grid, Position, Puzzle, Rule, SolutionGrid, SolverResult, SolverState};
use crate::solver::constraints::constraint::Constraint;
use crate::solver::constraints::constraint_set::ConstraintSet;
use crate::solver::constraints::variants::{killer, little_killer, thermometer, AntiKnightConstraint, ClassicConstraint, KillerConstraint, LittleKillerConstraint, ThermometerConstraint};

fn to_solver_state(puzzle: &Puzzle, constraint_set: &ConstraintSet) -> Option<SolverState> {
    let size = puzzle.puzzle_grid.size();
    let mut candidate_grid = puzzle.puzzle_grid.map(|cell| match cell {
        Some(value) => CandidateCell::with_value(*value),
        None => CandidateCell::with_count(size)
    });

    let active_positions = Grid::from_default(puzzle.puzzle_grid.region_shape(), true);

    if !constraint_set.update(&mut candidate_grid, active_positions) {
        return None;
    }

    Some(SolverState{candidate_grid})
}

fn to_solution_grid(solver_state: &SolverState) -> SolutionGrid {
    let solution_grid = solver_state.candidate_grid.map(|cell| match cell.fixed_value() {
        Some(value) => value,
        None => panic!("CandidateCell is not solved")
    });
    solution_grid
}

fn find_best_candidate(grid: &CandidateGrid) -> Option<Position> {
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

fn to_constraint_set(puzzle: &Puzzle) -> ConstraintSet {
    let mut constraints: Vec<Box<dyn Constraint>> = vec![Box::new(ClassicConstraint)];
    let region_shape = puzzle.puzzle_grid.region_shape();
    let grid_size = puzzle.puzzle_grid.size();
    for rule in puzzle.rules.iter() {
        match rule {
            Rule::AntiKnight => constraints.push(Box::new(AntiKnightConstraint)),
            Rule::Killer => {
                let cages = match puzzle.clues.get(&ClueType::KillerCage) {
                    Some(killer_cages) => killer_cages.iter().map(|clue| match clue {
                        Clue::KillerCage(killer_cage) => killer::Cage{sum: killer_cage.target_sum, positions: killer_cage.cage_cells.clone()},
                        _ => panic!("Expected KillerCage clue")
                    }).collect(),
                    None => panic!("Rules include Killer but Clues does not have KillerCage")
                };
                let killer_constraint = KillerConstraint::new(cages, region_shape.region_rows, region_shape.region_cols);
                constraints.push(Box::new(killer_constraint));
            },
            Rule::LittleKiller => {
                let diagonals = match puzzle.clues.get(&ClueType::LittleKillerArrow) {
                    Some(little_killer_arrows) => little_killer_arrows.iter().map(|clue| match clue {
                        Clue::LittleKillerArrow(little_killer_arrow) => little_killer::Diagonal::new(little_killer_arrow.target_sum, &little_killer_arrow.direction, little_killer_arrow.first_cell, grid_size),
                        _ => panic!("Expected LittleKillerArrow clue")
                    }).collect(),
                    None => panic!("Rules include LittleKiller but Clues does not have LittleKillerArrow")
                };
                let little_killer_constraint = LittleKillerConstraint::new(diagonals, region_shape.region_rows, region_shape.region_cols);
                constraints.push(Box::new(little_killer_constraint));
            },
            Rule::Thermometer => {
                let thermometers = match puzzle.clues.get(&ClueType::Thermometer) {
                    Some(thermometers) => thermometers.iter().map(|clue| match clue {
                        Clue::Thermometer(thermometer) => thermometer::Thermometer{positions: thermometer.thermometer_cells.clone()},
                        _ => panic!("Expected Thermometer clue")
                    }).collect(),
                    None => panic!("Rules include Thermometer but Clues does not have Thermometer")
                };
                let thermometer_constraint = ThermometerConstraint::new(thermometers, region_shape.region_rows, region_shape.region_cols);
                constraints.push(Box::new(thermometer_constraint));
            }
        }
    }
    ConstraintSet::new(constraints)
}

fn solve_recursive(solver_state: &mut SolverState, constraint_set: &ConstraintSet) -> SolverResult {
    let position = match find_best_candidate(&solver_state.candidate_grid) {
        Some(pos) => pos,
        None => return SolverResult::Solution(solver_state.clone()),
    };

    let mut solved_state = None;
    for value in 1..=solver_state.candidate_grid.size() {
        if !solver_state.candidate_grid[position].contains(value) {
            continue;
        }

        println!("Guessing value {} at {}", value, position);

        let mut new_solver_state = solver_state.clone();
        new_solver_state.candidate_grid[position] = CandidateCell::with_value(value);
        let mut active_positions = Grid::from_default(solver_state.candidate_grid.region_shape(), false);
        active_positions[position] = true;

        if !constraint_set.update(&mut new_solver_state.candidate_grid, active_positions) {
            continue;
        }

        match solve_recursive(&mut new_solver_state, constraint_set) {
            SolverResult::NoSolution => continue,
            SolverResult::MultipleSolution => return SolverResult::MultipleSolution,
            SolverResult::Solution(new_solver_state) => {
                match solved_state {
                    Some(_) => return SolverResult::MultipleSolution,
                    None => solved_state = Some(new_solver_state)
                };
            },
        }
    }

    match solved_state { 
        Some(solver_state) => SolverResult::Solution(solver_state),
        None => SolverResult::NoSolution,
    }
}

pub fn solve(puzzle: &Puzzle) -> Result<SolutionGrid, String> {
    let constraint_set = to_constraint_set(puzzle);
    let mut solver_state = match to_solver_state(puzzle, &constraint_set) {
        Some(candidate_grid) => candidate_grid,
        None => return Err("Contradiction found".to_string()),
    };

    println!("{}", solver_state.candidate_grid);

    match solve_recursive(&mut solver_state, &constraint_set) {
        SolverResult::NoSolution => Err("No solution found".to_string()),
        SolverResult::MultipleSolution => Err("Multiple solution found".to_string()),
        SolverResult::Solution(new_solver_state) => Ok(to_solution_grid(&new_solver_state)),
    }
}
