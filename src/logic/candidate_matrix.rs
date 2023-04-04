use std::{array, collections::HashSet};

use super::{grid::get_box_coords_containing, puzzle::Coord, Cell, SIZE};

#[derive(Debug, Clone)]
pub struct CandidateMatrix([[HashSet<u8>; SIZE]; SIZE]);

impl CandidateMatrix {
    pub fn new() -> Self {
        let mut c_matrix = HashSet::from(array::from_fn::<u8, SIZE, _>(|i| (i + 1) as u8));
        c_matrix.shrink_to(SIZE);
        CandidateMatrix(array::from_fn(|_| array::from_fn(|_| c_matrix.clone())))
    }

    pub fn get_candidates(&self, cell: Coord) -> &HashSet<u8> {
        &self.0[cell.row][cell.col]
    }

    pub fn update_candidates(&mut self, cell: Coord, val: u8) -> Result<(), ()> {
        for candidates in self.0[cell.row].iter_mut() {
            candidates.remove(&val);
            if candidates.is_empty() {
                return Err(());
            }
        }

        for row in self.0.iter_mut() {
            row[cell.col].remove(&val);
            if row[cell.col].is_empty() {
                return Err(());
            }
        }

        for coord in get_box_coords_containing(cell).into_iter() {
            self.0[coord.row][coord.col].remove(&val);
            if self.0[coord.row][coord.col].is_empty() {
                return Err(());
            }
        }

        Ok(())
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

    pub fn get_min_candidates_cell(&self) -> Coord {
        let mut coords = Coord { row: 0, col: 0 };
        let mut min = usize::MAX;

        for (i, row) in self.0.iter().enumerate() {
            for (j, candidates) in row.iter().enumerate() {
                if candidates.len() < min {
                    min = candidates.len();
                    coords = Coord { row: i, col: j };
                }
            }
        }

        coords
    }
}

impl From<[[Cell; SIZE]; SIZE]> for CandidateMatrix {
    fn from(rows: [[Cell; SIZE]; SIZE]) -> Self {
        let candidates = HashSet::from(array::from_fn::<u8, SIZE, _>(|i| (i + 1) as u8));

        let mut cm = CandidateMatrix(rows.map(|row| {
            row.map(|cell| match cell {
                Cell::Given(n) => HashSet::from([n]),
                _ => candidates.clone(),
            })
        }));

        rows.iter().enumerate().for_each(|(row_i, row)| {
            row.iter().enumerate().for_each(|(col_i, cell)| {
                if let Cell::Given(n) = cell {
                    cm.update_candidates((row_i, col_i).into(), *n)
                        .expect("invalid grid entered");
                }
            })
        });

        cm
    }
}
