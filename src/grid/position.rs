use crate::constants::REGION_SIZE;
use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Position {
    pub(crate) row: usize,
    pub(crate) col: usize,
}

impl Position {
    pub fn region(self) -> usize {
        (self.row / REGION_SIZE) * REGION_SIZE + (self.col / REGION_SIZE)
    }
}

impl fmt::Display for Position {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {})", self.row + 1, self.col + 1)
    }
}
