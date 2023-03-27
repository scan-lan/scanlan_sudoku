use std::collections::HashSet;

use super::{get_base_solution, grid::Cell, Grid, Group, SIZE};

pub struct CellCoord {
    pub row: usize,
    pub col: usize,
}

#[derive(Debug)]
pub struct Puzzle {
    grid: Grid,
    solution: Option<Grid>,
    candidate_matrix: [[HashSet<u8>; SIZE]; SIZE],
}

impl Puzzle {
    pub fn new() -> Puzzle {
        Puzzle {
            grid: Grid::new(),
            solution: Some(Grid::from(get_base_solution())),
            candidate_matrix: get_base_solution().map(|row| {
                row.map(|cell| {
                    if let Cell::Given(n) = cell {
                        HashSet::from([n])
                    } else {
                        HashSet::with_capacity(SIZE)
                    }
                })
            }),
        }
    }

    pub fn grid(&self) -> String {
        format!("{}", self.grid)
    }

    pub fn solution(&self) -> Option<String> {
        match &self.solution {
            Some(s) => Some(format!("{}", s)),
            None => None,
        }
    }

    pub fn get_row(&self, idx: usize) -> &Group {
        self.grid.get_row(idx)
    }
}

#[cfg(test)]
mod puzzle_tests {
    use std::collections::HashSet;

    use crate::logic::{get_base_solution, grid::Cell, SIZE};

    use super::Puzzle;

    #[test]
    fn create_empty_puzzle() {
        let p = Puzzle::new();
        let expected_grid = [[Cell::Empty; SIZE]; SIZE];

        assert_eq!(p.grid.rows(), &expected_grid);
    }

    #[test]
    fn get_grid_as_string() {
        let expected = String::from(
            " 1 2 3 | 4 5 6 | 7 8 9
 4 5 6 | 7 8 9 | 1 2 3
 7 8 9 | 1 2 3 | 4 5 6
-------+-------+-------
 2 3 4 | 5 6 7 | 8 9 1
 5 6 7 | 8 9 1 | 2 3 4
 8 9 1 | 2 3 4 | 5 6 7
-------+-------+-------
 3 4 5 | 6 7 8 | 9 1 2
 6 7 8 | 9 1 2 | 3 4 5
 9 1 2 | 3 4 5 | 6 7 8
",
        );
        let p = Puzzle::new();

        assert_eq!(p.solution(), Some(expected));
    }

    #[test]
    fn candidate_matrix_correct_for_from() {
        let expected = get_base_solution().map(|row| {
            row.map(|cell| match cell {
                Cell::Given(n) => HashSet::from([n]),
                _ => HashSet::with_capacity(SIZE),
            })
        });
        let p = Puzzle::new();

        assert_eq!(expected, p.candidate_matrix);
    }
}
