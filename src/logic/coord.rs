use std::fmt;

#[derive(PartialEq, Eq, Clone, Copy, PartialOrd, Ord)]
/// Struct to represent a coordinate which would be used to get a value from
/// a Sudoku grid.
pub struct Coord {
    pub row: usize,
    pub col: usize,
}

impl fmt::Debug for Coord {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})", self.row, self.col)
    }
}

impl fmt::Display for Coord {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})", self.row, self.col)
    }
}

/// Allow constructing a coord from a tuple and vice-versa
impl From<(usize, usize)> for Coord {
    fn from((row, col): (usize, usize)) -> Self {
        Coord { row, col }
    }
}

impl From<Coord> for (usize, usize) {
    fn from(c: Coord) -> Self {
        (c.row, c.col)
    }
}
