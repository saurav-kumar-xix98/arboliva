use crate::model::{CandidateCell, CandidateGrid, Clue, ClueType, Grid, Position, Puzzle, PuzzleGrid, Rule, SolutionGrid};
use crate::solver::constraints::constraint::Constraint;
use crate::solver::constraints::constraint_set::ConstraintSet;
use crate::solver::constraints::variants::{killer, little_killer, thermometer, AntiKnightConstraint, ClassicConstraint, KillerConstraint, LittleKillerConstraint, ThermometerConstraint};

fn to_candidate_grid(puzzle_grid: &PuzzleGrid) -> CandidateGrid {
    let candidate_grid = puzzle_grid.map(|cell| match cell {
        Some(value) => CandidateCell::with_value(*value),
        None => CandidateCell::with_count(puzzle_grid.size())
    });
    candidate_grid
}

fn from_candidate_grid(candidate_grid: CandidateGrid) -> SolutionGrid {
    let solution_grid = candidate_grid.map(|cell| match cell.fixed_value() {
        Some(value) => value,
        None => panic!("CandidateCell is not solved")
    });
    solution_grid
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

pub fn solve(puzzle: &Puzzle) -> Option<SolutionGrid> {
    let mut candidate_grid = to_candidate_grid(&puzzle.puzzle_grid);
    let constraint_set = to_constraint_set(puzzle);
    let active_positions = Grid::from_default(puzzle.puzzle_grid.region_shape(), true);

    println!("{}", candidate_grid);

    if solve_recursive(&mut candidate_grid, &constraint_set, active_positions) {
        Some(from_candidate_grid(candidate_grid))
    } else {
        None
    }
}
