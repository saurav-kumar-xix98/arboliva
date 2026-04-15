use crate::solver::Puzzle;
use std::error::Error;
use std::fs;
use crate::constraints::builder::to_puzzle;

mod constraints;
mod grid;
mod solver;

pub fn load_puzzle(path: &str) -> Result<Puzzle, Box<dyn Error>> {
    let yaml_str = fs::read_to_string(path)?;
    let yaml_value = serde_yaml::from_str(&yaml_str)?;

    let puzzle = to_puzzle(yaml_value)?;

    Ok(puzzle)
}

fn main() {
    let puzzle = match load_puzzle("puzzles/carnival.yaml") {
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
