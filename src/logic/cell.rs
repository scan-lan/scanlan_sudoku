use std::fmt;

use colored::Colorize;

use crate::CELL_WIDTH;

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
/// Enum representing possible states of a cell. `Clue` is when a number is
/// fixed, represented in bold, `Filled` is when a cell contains a player's
/// input, and `Empty` is an empty cell.
pub enum Cell {
    Clue(u8),
    Filled(u8),
    Empty,
}

impl fmt::Display for Cell {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let width = CELL_WIDTH as usize;
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
