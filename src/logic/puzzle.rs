use super::{get_base_solution, Grid, Group};

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct CellCoord {
    pub row: usize,
    pub col: usize,
}

impl From<(usize, usize)> for CellCoord {
    fn from((row, col): (usize, usize)) -> Self {
        CellCoord { row, col }
    }
}

impl From<CellCoord> for (usize, usize) {
    fn from(c: CellCoord) -> Self {
        (c.row, c.col)
    }
}

#[derive(Debug)]
pub struct Puzzle {
    grid: Grid,
    solution: Option<Grid>,
}

impl Puzzle {
    pub fn new() -> Puzzle {
        Puzzle {
            grid: Grid::new(),
            solution: Some(Grid::from(get_base_solution())),
        }
    }

    pub fn grid(&self) -> String {
        format!("{}", self.grid)
    }

    pub fn solution(&self) -> Option<String> {
        self.solution.as_ref().map(|s| format!("{}", s))
    }

    pub fn get_row(&self, idx: usize) -> &Group {
        self.grid.get_row(idx)
    }
}

#[cfg(test)]
mod puzzle_tests {

    use crate::logic::{grid::Cell, SIZE};

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
}
