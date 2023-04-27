use super::{
    candidate_matrix::CandidateMatrix,
    grid_trait::{DisplayableGrid, GridTrait},
    Cell, Coord, GridArray, ORDER, SIZE,
};

use std::{array, collections::HashSet, fmt};

#[derive(Clone, Debug)]
/// Struct to represent a Sudoku grid, with fields for the representation as
/// rows, columns and boxes, as well as the candidate matrix, an empty cell
/// count field, and a boolean representing whether the puzzle has been solved.
pub struct Grid {
    rows: GridArray,
    cols: GridArray,
    boxes: GridArray,
    candidate_matrix: CandidateMatrix,
    pub empty_cell_count: usize,
    pub solved: bool,
}

impl Grid {
    /// Construct a new grid.
    pub fn new() -> Grid {
        Grid {
            rows: [[Cell::Empty; SIZE]; SIZE],
            cols: [[Cell::Empty; SIZE]; SIZE],
            boxes: [[Cell::Empty; SIZE]; SIZE],
            candidate_matrix: CandidateMatrix::new(),
            empty_cell_count: SIZE.pow(2),
            solved: false,
        }
    }

    /// Check if the puzzle is solved by constructing a `HashSet` from all
    /// groups and checking its length is equal to SIZE.
    fn check_solved(&mut self) {
        let different = |&group| HashSet::from(group).len() == SIZE;

        self.solved = self.empty_cell_count == 0
            && self.rows.iter().all(different)
            && self.cols.iter().all(different)
            && self.boxes.iter().all(different);
    }

    /// Return a reference to the `rows` field.
    pub fn rows(&self) -> &GridArray {
        &self.rows
    }

    /// Return a reference to the `candidate_matrix` field.
    pub fn candidate_matrix(&self) -> &CandidateMatrix {
        &self.candidate_matrix
    }

    /// Return a reference to the cell at `cell`.
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
                self.check_solved();

                Ok(())
            }
        }
    }

    /// Update the value at `cell` to `val`. Returns an error if `cell` is a
    /// clue, or the update would result in another cell having zero valid
    /// candidates.
    pub fn update(&mut self, cell: Coord, val: u8) -> Result<(), GridError> {
        if let Cell::Clue(_) = self.get_cell(cell) {
            return Err(GridError::new(ErrorKind::UpdatedClue, cell, val));
        } else if !self.candidate_matrix.get_candidates(cell).contains(&val) {
            return Err(GridError::new(ErrorKind::NotInCandidates, cell, val));
        } else if let Cell::Empty = self.get_cell(cell) {
            self.empty_cell_count -= 1;
        }

        let (box_row, box_col) = row_coords_to_box_coords(cell).into();
        self.rows[cell.row][cell.col] = Cell::Filled(val);
        self.cols[cell.col][cell.row] = Cell::Filled(val);
        self.boxes[box_row][box_col] = Cell::Filled(val);

        self.candidate_matrix.set_fixed(cell);
        let result = self
            .candidate_matrix
            .update_around(cell, val)
            .map_err(|_| GridError::new(ErrorKind::ZeroCandidates, cell, val));

        self.check_solved();

        result
    }

    /// Remove candidate `val` from the set at `cell`.
    pub fn remove_candidate(&mut self, cell: Coord, val: u8) -> bool {
        self.candidate_matrix.remove_candidate(cell, val)
    }

    /// Returns the coordinates of the cell with the least valid candidates.
    pub fn get_min_candidates_cell(&self) -> Coord {
        self.candidate_matrix.get_min_candidates_cell()
    }

    /// Gets the candidates at `cell` as a vector.
    pub fn candidates_at(&self, cell: Coord) -> Vec<u8> {
        self.candidate_matrix.get_candidates(cell)
    }

    /// Constructs a `Grid` from a 2D array of `Cell`s.
    pub fn from_rows(rows: GridArray) -> Self {
        let cols = rows.cols();
        let boxes = rows.boxes();
        let empty_cell_count = rows.iter().fold(0, |mut acc, row| {
            acc += row.iter().filter(|c| c == &&Cell::Empty).count();
            acc
        });

        let mut g = Grid {
            rows,
            cols,
            boxes,
            candidate_matrix: CandidateMatrix::from(&rows),
            empty_cell_count,
            solved: false,
        };

        g.check_solved();
        g
    }

    /// Set all `Filled` cells to `Clue`s of the same value. Used in puzzle
    /// generation.
    pub fn set_clues(&mut self) {
        for row in 0..SIZE {
            for col in 0..SIZE {
                if let Cell::Filled(val) = self.rows[row][col] {
                    let box_coords = row_coords_to_box_coords((row, col).into());
                    self.rows[row][col] = Cell::Clue(val);
                    self.cols[col][row] = Cell::Clue(val);
                    self.boxes[box_coords.row][box_coords.col] = Cell::Clue(val);
                }
            }
        }
    }

    pub fn reset_candidate_matrix(&mut self) {
        self.candidate_matrix = CandidateMatrix::from(&self.rows);
    }
}

/// Enable conversion from 2D array of cells into `Grid`.
impl From<GridArray> for Grid {
    fn from(rows: GridArray) -> Grid {
        Self::from_rows(rows)
    }
}

/// Enable conversion from `DisplayableGrid` into `Grid`.
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
/// Struct representing possible errors that could arise from grid operations
pub struct GridError {
    details: String,
    pub kind: ErrorKind,
}

impl GridError {
    fn new(kind: ErrorKind, cell: Coord, val: u8) -> Self {
        let cell = Coord::from((cell.row + 1, cell.col + 1));
        let details = match kind {
            ErrorKind::ClearedClue => format!(
                "Failed to clear cell at {cell} because it's a clue",
            ),
            ErrorKind::UpdatedClue => format!(
                "Failed to update cell at {cell} with value `{val}` because it's a clue",
            ),
            ErrorKind::ZeroCandidates => format!(
                "Updating cell {cell} with value `{val}` resulted in a cell having zero candidate values"
            ),
            ErrorKind::NotInCandidates => format!(
                "Updating cell {cell} with value {val} is a mistake. Check again"
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
    NotInCandidates,
    UpdatedClue,
    ZeroCandidates,
}

/// Helper function to convert row coordinates to box coordinates.
pub fn row_coords_to_box_coords(cell: Coord) -> Coord {
    let (row, col) = cell.into();
    (
        (row / ORDER) * ORDER + col / ORDER,
        (row % ORDER) * ORDER + col % ORDER,
    )
        .into()
}

/// Helper function to get the coordinates of all cells in the box containing
/// `cell`.
pub fn get_box_coords_containing(cell: Coord) -> [Coord; SIZE] {
    let (row, col) = cell.into();
    let (row_offset, col_offset) = ((row / ORDER) * ORDER, (col / ORDER) * ORDER);
    array::from_fn(|i| (row_offset + i / ORDER, col_offset + i % ORDER).into())
}

/// Helper function to return a valid grid for testing purposes.
pub fn get_base_solution() -> GridArray {
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
