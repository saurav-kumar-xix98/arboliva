use std::fmt;
use std::fmt::{Display, Formatter};
use std::ops::{Index, IndexMut};
use crate::constants::{GRID_SIZE, REGION_SIZE};
use crate::grid::{CandidateCell, Position};

#[derive(Debug, Clone)]
pub struct Grid<Cell> {
    grid: [[Cell; GRID_SIZE]; GRID_SIZE],
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
        &self.grid[position.row][position.col]
    }
}

impl <Cell> IndexMut<Position> for Grid<Cell> {
    fn index_mut(&mut self, position: Position) -> &mut Self::Output {
        &mut self.grid[position.row][position.col]
    }
}

impl Grid<Option<usize>> {
    pub fn grid_from_yaml(raw: &serde_yaml::Value) -> Result<Grid<Option<usize>>, String> {
        let mut grid = Grid::from_default(None);

        // Expect raw as a 2D array (Vec<Vec<usize>>)
        let grid_raw: Vec<Vec<usize>> = serde_yaml::from_value(raw.clone())
            .map_err(|e| format!("invalid grid format: {}", e))?;

        if grid_raw.len() != GRID_SIZE {
            return Err(format!("grid must have {} rows, got {}", GRID_SIZE, grid_raw.len()));
        }

        for (row_idx, row) in grid_raw.into_iter().enumerate() {
            if row.len() != GRID_SIZE {
                return Err(format!(
                    "row {} must have {} columns, got {}",
                    row_idx, GRID_SIZE, row.len()
                ));
            }

            for (col_idx, val) in row.into_iter().enumerate() {
                let pos = Position {row: row_idx, col: col_idx };
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

                let row = r / REGION_SIZE;
                let col = c / REGION_SIZE;
                let position = Position{row, col};
                let cell = &self[position];

                let sub_row = r % REGION_SIZE;
                let sub_col = c % REGION_SIZE;
                let candidate = sub_row * REGION_SIZE + sub_col + 1;

                if cell.contains(candidate) {
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

impl Display for Grid<usize> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {

        for row in 0..GRID_SIZE {
            if row % REGION_SIZE == 0 {
                writeln!(f, "+-------+-------+-------+")?;
            }

            for col in 0..GRID_SIZE {
                if col % REGION_SIZE == 0 {
                    write!(f, "| ")?;
                }

                let pos = Position{row, col};
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
