use crate::model::{CandidateCell, Position, RegionShape};
use std::fmt;
use std::fmt::{Display, Formatter};
use std::ops::{Index, IndexMut};

#[derive(Debug, Clone)]
pub struct Grid<Cell> {
    region_shape: RegionShape,
    grid: Vec<Cell>,
}

impl <Cell: Clone> Grid<Cell> {
    pub fn from_default(region_shape: RegionShape, default_value: Cell) -> Self {
        let grid_size = region_shape.region_rows as usize * region_shape.region_cols as usize;
        let cell_count = grid_size * grid_size;

        Self {
            region_shape,
            grid: vec![default_value; cell_count],
        }
    }
}

impl <Cell> Grid<Cell> {

    pub fn region_rows(&self) -> u8 {
        self.region_shape.region_rows
    }

    pub fn region_cols(&self) -> u8 {
        self.region_shape.region_cols
    }

    pub fn size(&self) -> u8 {
        self.region_shape.region_rows * self.region_shape.region_cols
    }

    pub fn map<NewCell, F>(&self, f: F) -> Grid<NewCell>
    where
        F: Fn(&Cell) -> NewCell,
    {
        Grid {
            region_shape: self.region_shape.clone(),
            grid: self.grid.iter().map(f).collect(),
        }
    }

    pub fn zip_apply<Other, F>(&mut self, other: &Grid<Other>, mut f: F)
    where
        F: FnMut(&mut Cell, &Other),
    {
        for (self_cell, other_cell) in self.grid.iter_mut().zip(other.grid.iter()) {
            f(self_cell, other_cell);
        }
    }
}

impl <Cell> Index<Position> for Grid<Cell> {
    type Output = Cell;

    fn index(&self, position: Position) -> &Self::Output {
        let i = position.row as usize * self.size() as usize + position.col as usize;
        &self.grid[i]
    }
}

impl <Cell> IndexMut<Position> for Grid<Cell> {
    fn index_mut(&mut self, position: Position) -> &mut Self::Output {
        let i = position.row as usize * self.size() as usize + position.col as usize;
        &mut self.grid[i]
    }
}

impl Display for Grid<u8> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {

        let mut separator = String::new();
        for _ in 0..self.region_rows() {
            separator.push('+');
            separator.push_str(&"-".repeat(self.region_cols() as usize * 2 + 1));
        }
        separator.push('+');
        for row in 0..self.size() {
            if row % self.region_rows() == 0 {
                writeln!(f, "{}", separator)?;
            }

            for col in 0..self.size() {
                if col % self.region_cols() == 0 {
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

        writeln!(f, "{}", separator)?;
        Ok(())
    }
}

impl Display for Grid<CandidateCell> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {

        let mut region_separator = String::new();
        let mut row_separator = String::new();
        for i in 0..self.size() {
            if i % self.region_cols() == 0 {
                region_separator.push_str("++");
                row_separator.push_str("++");
            } else {
                region_separator.push_str("+");
                row_separator.push_str("+");
            }
            region_separator.push_str(&"=".repeat(self.region_cols() as usize * 2 + 1));
            row_separator.push_str(&"-".repeat(self.region_cols() as usize * 2 + 1));
        }
        region_separator.push_str("++");
        row_separator.push_str("++");

        for r in 0..(self.size() * self.region_rows()) {
            if r % (self.region_rows() * self.region_rows()) == 0 {
                writeln!(f, "{}", region_separator)?;
            } else if r % self.region_rows() == 0 {
                writeln!(f, "{}", row_separator)?;
            }
            for c in 0..(self.size() * self.region_cols()) {
                if c % (self.region_cols() * self.region_cols()) == 0 {
                    write!(f, "|| ")?;
                } else if c % self.region_cols() == 0 {
                    write!(f, "| ")?;
                }

                let row = r / self.region_rows();
                let col = c / self.region_cols();
                let position = Position{row, col};
                let cell = &self[position];

                let sub_row = r % self.region_rows();
                let sub_col = c % self.region_cols();
                let candidate = sub_row * self.region_cols() + sub_col + 1;

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
