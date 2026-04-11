use crate::constants::{GRID_SIZE, REGION_SIZE};
use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Position {
    index: u8,
}

impl Position {
    pub fn new(row: u8, col: u8) -> Self {
        assert!(row < GRID_SIZE && col < GRID_SIZE, "row or col out of bounds");
        Self {
            index: row * GRID_SIZE + col,
        }
    }

    pub fn row(self) -> u8 {
        self.index / GRID_SIZE
    }

    pub fn col(self) -> u8 {
        self.index % GRID_SIZE
    }

    pub fn region(self) -> u8 {
        (self.row() / REGION_SIZE) * REGION_SIZE + (self.col() / REGION_SIZE)
    }
}

impl fmt::Display for Position {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {})", self.row() + 1, self.col() + 1)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_position() {
        let position = Position::new(0, 0);
        assert_eq!(position.row(), 0);
        assert_eq!(position.col(), 0);

        let position = Position::new(8, 8);
        assert_eq!(position.row(), 8);
        assert_eq!(position.col(), 8);
    }

    #[test]
    #[should_panic(expected = "row or col out of bounds")]
    fn test_create_position_out_of_bounds_row() {
        let _position = Position::new(9, 0);
    }

    #[test]
    #[should_panic(expected = "row or col out of bounds")]
    fn test_create_position_out_of_bounds_col() {
        let _position = Position::new(0, 9);
    }

    #[test]
    fn test_region() {
        let position = Position::new(0, 0);
        assert_eq!(position.region(), 0);

        let position = Position::new(4, 5);
        assert_eq!(position.region(), 4);

        let position = Position::new(8, 8);
        assert_eq!(position.region(), 8);
    }

    #[test]
    fn test_display() {
        let position = Position::new(0, 0);
        assert_eq!(position.to_string(), "(1, 1)");

        let position = Position::new(7, 5);
        assert_eq!(position.to_string(), "(8, 6)");
    }

    #[test]
    fn test_equality() {
        let position1 = Position::new(2, 3);
        let position2 = Position::new(2, 3);
        let position3 = Position::new(3, 2);

        assert_eq!(position1, position2);
        assert_ne!(position1, position3);
    }
}
