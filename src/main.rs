use crate::io::load_puzzle;

mod model;
mod solver;
mod io;

fn main() {
    let puzzle = match load_puzzle("puzzles/thermometer.yaml") {
        Ok(p) => p,
        Err(e) => {
            eprintln!("Failed to load puzzle: {}", e);
            return;
        }
    };

    match solver::solve(&puzzle) {
        Some(solution_grid) => println!("{}", solution_grid),
        None => println!("No solution found"),
    }
}
