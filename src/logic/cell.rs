use std::fmt;

use colored::Colorize;

use crate::CELL_WIDTH;

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
