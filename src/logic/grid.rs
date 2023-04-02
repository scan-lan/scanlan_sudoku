use super::{grid_trait::GridTrait, puzzle::Coord, Group, CELL_WIDTH, ORDER, SIZE};
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
    pub empty_cell_count: u8,
}

impl Grid {
    pub fn new() -> Grid {
        let mut c_matrix = HashSet::from(array::from_fn::<u8, SIZE, _>(|i| (i + 1) as u8));
        c_matrix.shrink_to(SIZE);

        Grid {
            rows: [[Cell::Empty; SIZE]; SIZE],
            cols: [[Cell::Empty; SIZE]; SIZE],
            boxes: [[Cell::Empty; SIZE]; SIZE],
            candidate_matrix: array::from_fn(|_| array::from_fn(|_| c_matrix.clone())),
            empty_cell_count: SIZE.pow(2) as u8,
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
        let empty_cell_count = rows.iter().fold(0u8, |mut acc, row| {
            acc += row.iter().filter(|c| c == &&Cell::Empty).count() as u8;
            acc
        });

        Grid {
            rows,
            cols,
            boxes,
            candidate_matrix,
            empty_cell_count,
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

    pub fn get_cell(&self, cell: Coord) -> &Cell {
        &self.rows[cell.row][cell.col]
    }

    fn update_candidates(&mut self, cell: Coord, val: u8) {
        self.candidate_matrix[cell.row]
            .iter_mut()
            .for_each(|candidates| {
                candidates.remove(&val);
            });

        self.candidate_matrix.iter_mut().for_each(|row| {
            row[cell.col].remove(&val);
        });

        get_box_coords_containing(cell)
            .into_iter()
            .for_each(|coord| {
                let (row, col) = coord.into();
                self.candidate_matrix[row][col].remove(&val);
            })
    }

    pub fn update(&mut self, cell: Coord, val: u8) -> Result<(), GridError> {
        if let Cell::Given(_) = self.get_cell(cell) {
            return Err(GridError::new(ErrorKind::InvalidUpdate, cell, val));
        } else if let Cell::Empty = self.get_cell(cell) {
            self.empty_cell_count -= 1;
        }
        self.rows[cell.row][cell.col] = Cell::Filled(val);
        self.cols[cell.col][cell.row] = Cell::Filled(val);
        let (box_row, box_col) = row_coords_to_box_coords(cell).into();
        self.boxes[box_row][box_col] = Cell::Filled(val);
        self.update_candidates(cell, val);
        Ok(())
    }
}

impl Default for Grid {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug)]
pub struct GridError {
    details: String,
}

impl GridError {
    fn new(kind: ErrorKind, cell: Coord, val: u8) -> Self {
        let details = match kind {
            ErrorKind::InvalidUpdate => format!(
                "failed to update cell at ({row}, {col}) with value {val} because it's a clue",
                row = cell.row,
                col = cell.col,
            ),
        };

        GridError { details }
    }
}

impl fmt::Display for GridError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "{}", self.details)
    }
}

impl std::error::Error for GridError {
    fn description(&self) -> &str {
        &self.details
    }
}

#[derive(Debug)]
enum ErrorKind {
    InvalidUpdate,
}

pub fn row_coords_to_box_coords(cell: Coord) -> Coord {
    let (row, col) = cell.into();
    (
        (row / ORDER) * ORDER + col / ORDER,
        (row % ORDER) * ORDER + col % ORDER,
    )
        .into()
}

pub fn get_box_coords_containing(cell: Coord) -> [Coord; SIZE] {
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
