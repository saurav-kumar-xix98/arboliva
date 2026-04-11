use crate::solver::Puzzle;
use std::error::Error;
use std::fs;

mod constants;
mod constraints;
mod grid;
mod solver;

pub fn load_puzzle(path: &str) -> Result<Puzzle, Box<dyn Error>> {
    let yaml_str = fs::read_to_string(path)?;
    let yaml_value = serde_yaml::from_str(&yaml_str)?;

    let puzzle = Puzzle::from_yaml(&yaml_value)?;

    Ok(puzzle)
}

fn main() {
    let puzzle = match load_puzzle("puzzles/thermometer.yaml") {
        Ok(p) => p,
        Err(e) => {
            eprintln!("Failed to load puzzle: {}", e);
            return;
        }
    };

    match solver::solve(puzzle) {
        Some(solution) => println!("{}", solution),
        None => println!("No solution found"),
    }
}
