use crate::model::{Clue, ClueType, PuzzleGrid, Rule};
use std::collections::{HashMap, HashSet};

pub struct Puzzle {
    pub puzzle_grid: PuzzleGrid,
    pub rules: HashSet<Rule>,
    pub clues: HashMap<ClueType, Vec<Clue>>,
}
