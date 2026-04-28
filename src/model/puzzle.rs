use std::collections::{HashMap, HashSet};
use crate::model::{Clue, ClueType, PuzzleGrid, Rule};

pub struct Puzzle {
    pub puzzle_grid: PuzzleGrid,
    pub rules: HashSet<Rule>,
    pub clues: HashMap<ClueType, Vec<Clue>>,
}
