use super::{grid_trait::GridTrait, puzzle::CellCoord, Group, CELL_WIDTH, ORDER, SIZE};
use std::{array, collections::HashSet, fmt};

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Cell {
    Given(u8),
    Filled(u8),
    Empty,
}

impl fmt::Display for Cell {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let width = usize::try_from(CELL_WIDTH).expect("cell width should always be small");
        match self {
            Self::Given(n) => write!(f, "{:width$}", n),
            Self::Filled(n) => write!(f, "{:width$}", n),
            Self::Empty => write!(f, "{:width$}", " "),
        }
    }
}

#[derive(Debug)]
pub struct Grid {
    rows: [Group; SIZE],
    cols: [Group; SIZE],
    boxes: [Group; SIZE],
    candidate_matrix: [[HashSet<u8>; SIZE]; SIZE],
}

impl Grid {
    pub fn new() -> Grid {
        Grid {
            rows: [[Cell::Empty; SIZE]; SIZE],
            cols: [[Cell::Empty; SIZE]; SIZE],
            boxes: [[Cell::Empty; SIZE]; SIZE],
            candidate_matrix: array::from_fn(|_| array::from_fn(|_| HashSet::with_capacity(SIZE))),
        }
    }

    pub fn from(rows: [Group; SIZE]) -> Grid {
        let cols = rows.cols();
        let boxes = rows.boxes();
        let candidate_matrix = rows.map(|row| {
            row.map(|cell| {
                if let Cell::Given(n) = cell {
                    HashSet::from([n])
                } else {
                    HashSet::with_capacity(SIZE)
                }
            })
        });

        Grid {
            rows,
            cols,
            boxes,
            candidate_matrix,
        }
    }

    pub fn rows(&self) -> &[Group; SIZE] {
        &self.rows
    }

    pub fn cols(&self) -> &[Group; SIZE] {
        &self.cols
    }

    pub fn boxes(&self) -> &[Group; SIZE] {
        &self.boxes
    }

    pub fn candidate_matrix(&self) -> &[[HashSet<u8>; SIZE]; SIZE] {
        &self.candidate_matrix
    }

    pub fn get_row(&self, idx: usize) -> &Group {
        &self.rows[idx]
    }

    pub fn get_col(&self, idx: usize) -> &Group {
        &self.cols[idx]
    }

    pub fn get_box(&self, idx: usize) -> &Group {
        &self.boxes[idx]
    }

    pub fn get_cell(&self, pos: CellCoord) -> &Cell {
        &self.rows[pos.row][pos.col]
    }

    pub fn update(&mut self, pos: CellCoord, val: u8) {
        self.rows[pos.row][pos.col] = Cell::Filled(val);
        self.cols[pos.col][pos.row] = Cell::Filled(val);
        let (box_row, box_col) = row_coords_to_box_coords(pos).into();
        self.boxes[box_row][box_col] = Cell::Filled(val);
    }
}

impl Default for Grid {
    fn default() -> Self {
        Self::new()
    }
}

pub fn row_coords_to_box_coords(cell: CellCoord) -> CellCoord {
    let (row, col) = cell.into();
    (
        (row / ORDER) * ORDER + col / ORDER,
        (row % ORDER) * ORDER + col % ORDER,
    )
        .into()
}

pub fn get_box_containing(cell: CellCoord) -> [CellCoord; SIZE] {
    let (row, col) = cell.into();
    let (row_offset, col_offset) = ((row / ORDER) * ORDER, (col / ORDER) * ORDER);
    array::from_fn(|i| (row_offset + i / ORDER, col_offset + i % ORDER).into())
}

pub fn get_base_solution() -> [Group; SIZE] {
    array::from_fn(|i| {
        array::from_fn(|j| Cell::Given((1 + (j + (i / ORDER) + (i % ORDER) * ORDER) % SIZE) as u8))
    })
}

impl fmt::Display for Grid {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let cell_width = usize::try_from(CELL_WIDTH).expect("cell width should always be small");
        let box_width: usize = cell_width * ORDER + cell_width - 1;

        for (i, row) in self.rows.iter().enumerate() {
            for (j, cell) in row.iter().enumerate() {
                write!(f, "{:>cell_width$}", cell)?;
                if j != SIZE - 1 && j % ORDER == ORDER - 1 {
                    write!(f, "{:>cell_width$}", "|")?;
                }
            }
            writeln!(f)?;
            if i != SIZE - 1 && i % ORDER == ORDER - 1 {
                let line = format!("{:->box_width$}", "-");
                for _ in 1..ORDER {
                    write!(f, "{line}+")?;
                }
                writeln!(f, "{line}")?;
            }
        }
        Ok(())
    }
}

#[cfg(test)]
mod grid_tests;
