use super::{Group, CELL_WIDTH, ORDER, SIZE};
use std::fmt;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Cell {
    Given(u8),
    Filled(u8),
    Empty,
}

impl fmt::Display for Cell {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let width = usize::try_from(CELL_WIDTH).expect("cell width should always be small");
        match self {
            Self::Given(n) => write!(f, "{:width$}", n),
            Self::Filled(n) => write!(f, "{:width$}", n),
            Self::Empty => write!(f, "{:width$}", " "),
        }
    }
}

#[derive(Debug)]
pub struct Grid {
    rows: [Group; SIZE],
    cols: [Group; SIZE],
    boxes: [Group; SIZE],
}

impl Grid {
    pub fn new() -> Grid {
        Grid {
            rows: [[Cell::Empty; SIZE]; SIZE],
            cols: [[Cell::Empty; SIZE]; SIZE],
            boxes: [[Cell::Empty; SIZE]; SIZE],
        }
    }

    pub fn from(rows: [Group; SIZE]) -> Grid {
        Grid {
            rows,
            cols: rows,
            boxes: rows,
        }
    }

    pub fn rows(&self) -> &[Group; SIZE] {
        &self.rows
    }

    pub fn get_row(&self, idx: usize) -> &Group {
        &self.rows[idx]
    }

    pub fn get_col(&self, idx: usize) -> &Group {
        &self.cols[idx]
    }

    pub fn get_box(&self, idx: usize) -> &Group {
        &self.boxes[idx]
    }
}

pub fn get_base_solution() -> [Group; SIZE] {
    let mut base_solution = [[Cell::Empty; SIZE]; SIZE];
    for i in 0..SIZE {
        for j in 0..SIZE {
            base_solution[i][j] =
                Cell::Given((1 + (j + (i / ORDER) + (i % ORDER) * ORDER) % SIZE) as u8);
        }
    }

    base_solution
}

impl fmt::Display for Grid {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let cell_width = usize::try_from(CELL_WIDTH).expect("cell width should always be small");
        let box_width: usize = cell_width * ORDER + cell_width - 1;

        for (i, row) in self.rows.iter().enumerate() {
            for (j, cell) in row.iter().enumerate() {
                write!(f, "{:>cell_width$}", cell)?;
                if j != SIZE - 1 && j % ORDER == ORDER - 1 {
                    write!(f, "{:>cell_width$}", "|")?;
                }
            }
            write!(f, "\n")?;
            if i != SIZE - 1 && i % ORDER == ORDER - 1 {
                let line = format!("{:->box_width$}", "-");
                for _ in 1..ORDER {
                    write!(f, "{line}+")?;
                }
                writeln!(f, "{line}")?;
            }
        }
        Ok(())
    }
}

mod grid_tests;
