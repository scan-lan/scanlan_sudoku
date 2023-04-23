use super::{
    candidate_matrix::CandidateMatrix,
    grid_trait::{DisplayableGrid, GridTrait},
    puzzle::Coord,
    Group, CELL_WIDTH, ORDER, SIZE,
};

use colored::Colorize;
use std::{array, collections::HashSet, fmt};

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum Cell {
    Clue(u8),
    Filled(u8),
    Empty,
}

impl fmt::Display for Cell {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let width = usize::try_from(CELL_WIDTH).expect("cell width should always be small");
        match self {
            Self::Clue(n) => {
                if n == &0 {
                    write!(f, "{:>width$}", '?')
                } else {
                    write!(f, "{:>width$}", n.to_string().bold())
                }
            }
            Self::Filled(n) => write!(f, "{:width$}", n),
            Self::Empty => write!(f, "{:width$}", " "),
        }
    }
}

#[derive(Clone, Debug)]
pub struct Grid {
    rows: [Group; SIZE],
    cols: [Group; SIZE],
    boxes: [Group; SIZE],
    candidate_matrix: CandidateMatrix,
    pub empty_cell_count: u8,
    pub solved: bool,
}

impl Grid {
    pub fn new() -> Grid {
        Grid {
            rows: [[Cell::Empty; SIZE]; SIZE],
            cols: [[Cell::Empty; SIZE]; SIZE],
            boxes: [[Cell::Empty; SIZE]; SIZE],
            candidate_matrix: CandidateMatrix::new(),
            empty_cell_count: SIZE.pow(2) as u8,
            solved: false,
        }
    }

    fn check_solved(&mut self) {
        let different = |&group| HashSet::from(group).len() == SIZE;

        self.solved = self.empty_cell_count == 0
            && self.rows.iter().all(different)
            && self.cols.iter().all(different)
            && self.boxes.iter().all(different);
    }

    pub fn rows(&self) -> &[Group; SIZE] {
        &self.rows
    }

    pub fn candidate_matrix(&self) -> &CandidateMatrix {
        &self.candidate_matrix
    }

    pub fn get_cell(&self, cell: Coord) -> &Cell {
        &self.rows[cell.row][cell.col]
    }

    /// Set `cell` to empty. Returns an error if called on a clue.
    pub fn clear(&mut self, cell: Coord) -> Result<(), GridError> {
        let (row, col) = cell.into();
        match self.rows[row][col] {
            Cell::Empty => Ok(()),
            Cell::Clue(_) => Err(GridError::new(ErrorKind::ClearedClue, cell, 0)),
            Cell::Filled(_) => {
                let (box_row, box_col) = row_coords_to_box_coords(cell).into();
                self.rows[row][col] = Cell::Empty;
                self.cols[col][row] = Cell::Empty;
                self.boxes[box_row][box_col] = Cell::Empty;
                self.empty_cell_count += 1;

                Ok(())
            }
        }
    }

    /// Update the value at `cell` to `val`. Returns an error if `cell` is a
    /// clue, or the update would result in another cell having zero valid
    /// candidates. Changes to candidates are rolled back if an error occurs.
    /// Returns the coordinates of all cells whose candidates were changed by
    /// the update.
    pub fn update(&mut self, cell: Coord, val: u8) -> Result<Vec<Coord>, GridError> {
        if let Cell::Clue(_) = self.get_cell(cell) {
            return Err(GridError::new(ErrorKind::UpdatedClue, cell, val));
        } else if let Cell::Empty = self.get_cell(cell) {
            self.empty_cell_count -= 1;
        }

        let (box_row, box_col) = row_coords_to_box_coords(cell).into();
        self.rows[cell.row][cell.col] = Cell::Filled(val);
        self.cols[cell.col][cell.row] = Cell::Filled(val);
        self.boxes[box_row][box_col] = Cell::Filled(val);

        // Creates a copy of candidate matrix in case the update is invalid
        let cm_backup = self.candidate_matrix.clone();
        self.candidate_matrix.set_fixed(cell);
        let result = self
            .candidate_matrix
            .update_around(cell, val)
            .map_err(|_| GridError::new(ErrorKind::ZeroCandidates, cell, val));

        if result.is_err() {
            // Revert to the copied version
            self.candidate_matrix = cm_backup;
        } else {
            self.check_solved();
        }

        result
    }

    pub fn collapse(&mut self, cell: Coord) -> u8 {
        self.candidate_matrix.collapse(cell)
    }

    pub fn remove_candidate(&mut self, cell: Coord, val: u8) -> bool {
        self.candidate_matrix.remove_candidate(cell, val)
    }

    pub fn get_min_candidates_cell(&self) -> Coord {
        self.candidate_matrix.get_min_candidates_cell()
    }

    pub fn candidates_at(&self, cell: Coord) -> Vec<u8> {
        self.candidate_matrix.get_candidates(cell)
    }

    pub fn from_rows(rows: [[Cell; SIZE]; SIZE]) -> Self {
        let cols = rows.cols();
        let boxes = rows.boxes();
        let empty_cell_count = rows.iter().fold(0u8, |mut acc, row| {
            acc += row.iter().filter(|c| c == &&Cell::Empty).count() as u8;
            acc
        });

        let mut g = Grid {
            rows,
            cols,
            boxes,
            candidate_matrix: rows.into(),
            empty_cell_count,
            solved: false,
        };

        g.check_solved();
        g
    }
}

impl From<[[Cell; SIZE]; SIZE]> for Grid {
    fn from(rows: [Group; SIZE]) -> Grid {
        Self::from_rows(rows)
    }
}

impl From<DisplayableGrid<Cell>> for Grid {
    fn from(dg: DisplayableGrid<Cell>) -> Self {
        Self::from_rows(dg.0)
    }
}

impl Default for Grid {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Clone)]
pub struct GridError {
    details: String,
    pub kind: ErrorKind,
}

impl GridError {
    fn new(kind: ErrorKind, cell: Coord, val: u8) -> Self {
        let details = match kind {
            ErrorKind::ClearedClue => format!(
                "failed to clear cell at {cell} because it's a clue",
            ),
            ErrorKind::UpdatedClue => format!(
                "failed to update cell at {cell} with value `{val}` because it's a clue",
            ),
            ErrorKind::ZeroCandidates => format!(
                "updating cell {cell} with value `{val}` resulted in a cell having zero candidate values"
            )
        };

        GridError { details, kind }
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

#[derive(Clone, Copy, Debug)]
pub enum ErrorKind {
    ClearedClue,
    UpdatedClue,
    ZeroCandidates,
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
        array::from_fn(|j| Cell::Clue((1 + (j + (i / ORDER) + (i % ORDER) * ORDER) % SIZE) as u8))
    })
}

impl fmt::Display for Grid {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let g = DisplayableGrid(self.rows);
        write!(f, "{g}")?;
        Ok(())
    }
}

#[cfg(test)]
mod grid_tests;
