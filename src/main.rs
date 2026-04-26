use crate::io::load_puzzle;

mod model;
mod solver;
mod io;

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
