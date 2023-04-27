use std::fmt::Write;
use std::{array, collections::HashSet, fmt};

use rand::seq::SliceRandom;

use super::{grid::get_box_coords_containing, Cell, Coord, NUM_WIDTH, ORDER, SIZE};

#[derive(Debug, Clone)]
/// Struct representing the possible values for cells in a `Grid`. If a value is
/// in a set, that value is a valid possibility for the cell at the
/// corresponding position in the `Grid`.
/// If a set contains `0`, this indicates it can't be changed and shouldn't be
/// considered by methods such as `get_min_candidates`.
pub struct CandidateMatrix([[HashSet<u8>; SIZE]; SIZE]);

/// Create the candidate matrix for the grid passed as rows.
impl From<[[Cell; SIZE]; SIZE]> for CandidateMatrix {
    fn from(rows: [[Cell; SIZE]; SIZE]) -> Self {
        let candidates = HashSet::from(array::from_fn::<u8, SIZE, _>(|i| (i + 1) as u8));

        let mut cm = CandidateMatrix(rows.map(|row| {
            row.map(|cell| match cell {
                Cell::Clue(_) => HashSet::from([0]),
                _ => candidates.clone(),
            })
        }));

        rows.iter().enumerate().for_each(|(row_i, row)| {
            row.iter().enumerate().for_each(|(col_i, cell)| {
                if let Cell::Clue(n) = cell {
                    cm.update_around((row_i, col_i).into(), *n)
                        .expect("invalid grid entered");
                }
            })
        });

        cm
    }
}

impl CandidateMatrix {
    /// Create a new candidate matrix
    pub fn new() -> Self {
        let mut c_matrix = HashSet::from(array::from_fn::<u8, SIZE, _>(|i| (i + 1) as u8));
        c_matrix.shrink_to(SIZE);
        CandidateMatrix(array::from_fn(|_| array::from_fn(|_| c_matrix.clone())))
    }

    /// Get the canidates for the cell at `cell`.
    pub fn get_candidates(&self, cell: Coord) -> Vec<u8> {
        Vec::from_iter(self.0[cell.row][cell.col].clone())
    }

    /// Update the candidate sets for each group containing `cell`. This removes
    /// `val` from all cells in these groups if it's present.
    pub fn update_around(&mut self, cell: Coord, val: u8) -> Result<(), ()> {
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

    /// Remove candiate `val` from `cell`.
    pub fn remove_candidate(&mut self, cell: Coord, val: u8) -> bool {
        self.0[cell.row][cell.col].remove(&val)
    }

    /// Gets the coordinates of the cell with the lowest possibilities in the
    /// grid. Excludes cells with already-set values (i.e. contain `0`).
    pub fn get_min_candidates_cell(&self) -> Coord {
        let rows_to_lens = |row: [HashSet<u8>; SIZE]| {
            row.into_iter()
                .map(|c| if c.contains(&0) { usize::MAX } else { c.len() })
                .collect()
        };
        let lens: Vec<Vec<_>> = self.0.clone().into_iter().map(rows_to_lens).collect();

        let mut coords: Vec<Coord> = Vec::new();

        let min = lens.clone().into_iter().flatten().min();
        lens.into_iter().enumerate().for_each(|(i, row)| {
            row.into_iter().enumerate().for_each(|(j, c)| {
                if Some(c) == min {
                    coords.push((i, j).into());
                }
            })
        });

        *coords
            .choose(&mut rand::thread_rng())
            .expect("There has to be a minimum")
    }

    /// This marks a cell as fixed, so it won't be included in
    /// `get_min_candidates` results.
    pub fn set_fixed(&mut self, cell: Coord) {
        self.0[cell.row][cell.col] = HashSet::from([0]);
    }
}

/// Enables displaying the candidate matrix for debugging
impl fmt::Display for CandidateMatrix {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let num_width = NUM_WIDTH as usize;
        let line_width = SIZE * (num_width * ORDER + 1) + (2 * (ORDER - 1)) + 1;
        let box_width = line_width / ORDER + 1;

        for (i, row) in self.0.iter().enumerate() {
            let row_str = fmt_row(row).expect("Shouldn't fail");
            write!(f, "{row_str}")?;
            for _ in 0..(ORDER - 1) {
                write!(f, "{:>box_width$}", "|")?;
            }
            writeln!(f)?;

            if i != SIZE - 1 && i % ORDER == ORDER - 1 {
                writeln!(f, "{:->line_width$}", "-")?;
            }
        }
        Ok(())
    }
}

/// Helper function for displaying a candidate matrix for debugging purposes
fn fmt_row(row: &[HashSet<u8>; SIZE]) -> Result<String, fmt::Error> {
    let width = NUM_WIDTH as usize;
    let mut s = String::new();

    for outer_idx in 0..ORDER {
        for (candidate_idx, candidates) in row.iter().enumerate().take(SIZE) {
            for inner_idx in 1..=ORDER {
                if inner_idx == 1 {
                    write!(s, " ")?;
                }
                let candidate = inner_idx + outer_idx * ORDER;
                if candidates.contains(&0) {
                    write!(s, "{:>width$}", "X")?;
                } else if candidates.contains(&(candidate as u8)) {
                    write!(s, "{:>width$}", candidate)?;
                } else {
                    write!(s, "{:>width$}", " ")?;
                }
            }
            if candidate_idx != SIZE - 1 && candidate_idx % ORDER == ORDER - 1 {
                write!(s, "{:>2}", "|")?;
            }
        }
        writeln!(s)?;
    }
    Ok(s)
}
