use std::{array, collections::HashSet};

use super::{grid::get_box_coords_containing, puzzle::Coord, Cell, SIZE};

#[derive(Debug, Clone)]
pub struct CandidateMatrix([[HashSet<u8>; SIZE]; SIZE]);

impl From<[[Cell; SIZE]; SIZE]> for CandidateMatrix {
    fn from(rows: [[Cell; SIZE]; SIZE]) -> Self {
        let candidates = HashSet::from(array::from_fn::<u8, SIZE, _>(|i| (i + 1) as u8));

        let mut cm = CandidateMatrix(rows.map(|row| {
            row.map(|cell| match cell {
                Cell::Given(_) => HashSet::from([0]),
                _ => candidates.clone(),
            })
        }));

        rows.iter().enumerate().for_each(|(row_i, row)| {
            row.iter().enumerate().for_each(|(col_i, cell)| {
                if let Cell::Given(n) = cell {
                    cm.update_around((row_i, col_i).into(), *n)
                        .expect("invalid grid entered");
                }
            })
        });

        cm
    }
}

impl CandidateMatrix {
    pub fn new() -> Self {
        let mut c_matrix = HashSet::from(array::from_fn::<u8, SIZE, _>(|i| (i + 1) as u8));
        c_matrix.shrink_to(SIZE);
        CandidateMatrix(array::from_fn(|_| array::from_fn(|_| c_matrix.clone())))
    }

    pub fn get_candidates(&self, cell: Coord) -> Vec<u8> {
        Vec::from_iter(self.0[cell.row][cell.col].clone())
    }

    /// Update the candidate sets for each group containing `cell`. Returns the
    /// coordinates of all candidate sets changed by the update.
    pub fn update_around(&mut self, cell: Coord, val: u8) -> Result<Vec<Coord>, ()> {
        let mut changed = vec![];

        for (col_i, candidates) in self.0[cell.row].iter_mut().enumerate() {
            if candidates.remove(&val) {
                changed.push((cell.row, col_i).into());
            }
            if candidates.is_empty() {
                return Err(());
            }
        }

        for (row_i, row) in self.0.iter_mut().enumerate() {
            if row[cell.col].remove(&val) {
                changed.push((row_i, cell.col).into())
            }
            if row[cell.col].is_empty() {
                return Err(());
            }
        }

        for coord in get_box_coords_containing(cell).into_iter() {
            if self.0[coord.row][coord.col].remove(&val) {
                changed.push(coord);
            }
            if self.0[coord.row][coord.col].is_empty() {
                return Err(());
            }
        }

        Ok(changed)
    }

    pub fn collapse(&mut self, cell: Coord) -> u8 {
        let val = *self.0[cell.row][cell.col]
            .iter()
            .next()
            .expect("don't call collapse on a cell with 0 candidates");

        val
    }

    pub fn remove_candidate(&mut self, cell: Coord, val: u8) -> bool {
        self.0[cell.row][cell.col].remove(&val)
    }

    pub fn add_candidate(&mut self, cell: Coord, val: u8) -> bool {
        self.0[cell.row][cell.col].insert(val)
    }

    /// Gets the coordinates of the cell with the lowest possibilities in the
    /// grid. Excludes cells with only 1 candidate as these must have values
    pub fn get_min_candidates_cell(&self) -> Coord {
        let mut coords = Coord { row: 0, col: 0 };
        let mut min = usize::MAX;

        for (i, row) in self.0.iter().enumerate() {
            for (j, candidates) in row.iter().enumerate() {
                if !candidates.contains(&0) && (2..min).contains(&candidates.len()) {
                    min = candidates.len();
                    coords = Coord { row: i, col: j };
                }
            }
        }

        coords
    }

    pub fn undo_changed(&mut self, val: Cell, changed: Vec<Coord>) {
        if let Cell::Given(n) = val {
            changed.iter().for_each(|cell| {
                self.0[cell.row][cell.col].insert(n);
            });
        }
    }
}
