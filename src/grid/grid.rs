use std::fmt;
use std::fmt::{Display, Formatter};
use std::ops::{Index, IndexMut};
use crate::constants::{GRID_SIZE, REGION_SIZE};
use crate::grid::{CandidateCell, Position};

#[derive(Debug, Clone)]
pub struct Grid<Cell> {
    grid: [[Cell; GRID_SIZE as usize]; GRID_SIZE as usize],
}

impl <Cell: Clone> Grid<Cell> {
    pub fn from_default(default_value: Cell) -> Self {
        Self {
            grid: std::array::from_fn(|_| std::array::from_fn(|_| default_value.clone())),
        }
    }
}

impl <Cell> Index<Position> for Grid<Cell> {
    type Output = Cell;

    fn index(&self, position: Position) -> &Self::Output {
        &self.grid[position.row() as usize][position.col() as usize]
    }
}

impl <Cell> IndexMut<Position> for Grid<Cell> {
    fn index_mut(&mut self, position: Position) -> &mut Self::Output {
        &mut self.grid[position.row() as usize][position.col() as usize]
    }
}

impl Grid<Option<u8>> {
    pub fn grid_from_yaml(raw: &serde_yaml::Value) -> Result<Grid<Option<u8>>, String> {
        // Expect raw as a 2D array (Vec<Vec<u8>>)
        let rows: Vec<Vec<u8>> = serde_yaml::from_value(raw.clone())
            .map_err(|e| format!("invalid grid format: {}", e))?;

        if rows.len() != GRID_SIZE as usize {
            return Err(format!("grid must have {} rows, got {}", GRID_SIZE, rows.len()));
        }

        let mut grid = Grid::from_default(None);

        for (row_idx, row) in rows.into_iter().enumerate() {
            if row.len() != GRID_SIZE as usize {
                return Err(format!(
                    "row {} must have {} columns, got {}",
                    row_idx, GRID_SIZE, row.len()
                ));
            }

            for (col_idx, val) in row.into_iter().enumerate() {
                let pos = Position::new(row_idx as u8, col_idx as u8);
                grid[pos] = match val {
                    0 => None,
                    1..=GRID_SIZE => Some(val),
                    other => return Err(format!("invalid value {} at {}", other, pos)),
                };
            }
        }

        Ok(grid)
    }
}

impl Display for Grid<CandidateCell> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {

        for r in 0..(GRID_SIZE * REGION_SIZE) {
            if r % 9 == 0 {
                writeln!(f, "++=======+=======+=======++=======+=======+=======++=======+=======+=======++")?;
            } else if r % REGION_SIZE == 0 {
                writeln!(f, "++-------+-------+-------++-------+-------+-------++-------+-------+-------++")?;
            }

            for c in 0..(GRID_SIZE * REGION_SIZE) {
                if c % 9 == 0 {
                    write!(f, "|| ")?;
                } else if c % REGION_SIZE == 0 {
                    write!(f, "| ")?;
                }

                let cell_row = r / REGION_SIZE;
                let cell_col = c / REGION_SIZE;
                let position = Position::new(cell_row, cell_col);
                let cell = &self[position];

                let sub_row = r % REGION_SIZE;
                let sub_col = c % REGION_SIZE;
                let candidate = sub_row * REGION_SIZE + sub_col + 1;

                if cell.is_valid(candidate) {
                    write!(f, "{} ", candidate)?;
                } else {
                    write!(f, "  ")?;
                }
            }

            writeln!(f, "||")?;
        }

        writeln!(f, "++=======+=======+=======++=======+=======+=======++=======+=======+=======++")?;
        Ok(())
    }
}

impl Display for Grid<u8> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {

        for r in 0..GRID_SIZE {
            if r % REGION_SIZE == 0 {
                writeln!(f, "+-------+-------+-------+")?;
            }

            for c in 0..GRID_SIZE {
                if c % REGION_SIZE == 0 {
                    write!(f, "| ")?;
                }

                let pos = Position::new(r, c);
                let val = self[pos];

                if val != 0 {
                    write!(f, "{} ", val)?;
                } else {
                    write!(f, "  ")?;
                }
            }

            writeln!(f, "|")?;
        }

        writeln!(f, "+-------+-------+-------+")?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_grid_init() {
        let grid : Grid<u8> = Grid::from_default(0);
        let position = Position::new(4, 5);

        assert_eq!(grid[position], 0);
    }

    #[test]
    fn test_grid_setter_getter() {
        let mut grid : Grid<u8> = Grid::from_default(0);
        let position = Position::new(4, 5);

        grid[position] = 5;
        assert_eq!(grid[position], 5);
    }
}
