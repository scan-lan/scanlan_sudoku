use std::fmt::Write;
use std::{array, collections::HashSet, fmt};

use super::{
    grid::get_box_coords_containing, puzzle::Coord, Cell, CELL_WIDTH, NUM_WIDTH, ORDER, SIZE,
};

#[derive(Debug, Clone)]
pub struct CandidateMatrix([[HashSet<u8>; SIZE]; SIZE]);

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
    pub fn new() -> Self {
        let mut c_matrix = HashSet::from(array::from_fn::<u8, SIZE, _>(|i| (i + 1) as u8));
        c_matrix.shrink_to(SIZE);
        CandidateMatrix(array::from_fn(|_| array::from_fn(|_| c_matrix.clone())))
    }

    pub fn get_candidates(&self, cell: Coord) -> Vec<u8> {
        Vec::from_iter(self.0[cell.row][cell.col].clone())
    }

    /// Update the candidate sets for each group containing `cell`. Returns a
    /// deduplicated vector of coordinates of all candidate sets changed by the
    /// update.
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

        changed.sort();
        changed.dedup();

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
                if !candidates.contains(&0) && candidates.len() < min {
                    min = candidates.len();
                    coords = Coord { row: i, col: j };
                }
            }
        }

        coords
    }

    pub fn undo_changed(&mut self, val: Cell, changed: Vec<Coord>) {
        if let Cell::Clue(n) = val {
            changed.iter().for_each(|cell| {
                self.0[cell.row][cell.col].insert(n);
            });
        }
    }

    /// This marks a 'Filled' cell as fixed, so it won't be included in
    /// `get_min_candidates` results.
    pub fn set_fixed(&mut self, cell: Coord) {
        self.0[cell.row][cell.col] = HashSet::from([0]);
    }
}

fn fmt_row(row: &[HashSet<u8>; SIZE]) -> Result<String, fmt::Error> {
    let width = NUM_WIDTH as usize;
    let mut s = String::new();

    for outer_idx in 0..ORDER {
        for candidate_idx in 0..SIZE {
            for inner_idx in 1..=ORDER {
                if inner_idx == 1 {
                    write!(s, " ")?;
                }
                let candidate = inner_idx + outer_idx * ORDER;
                if row[candidate_idx].contains(&(candidate as u8)) {
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
