use super::{puzzle::CellCoord, Group, CELL_WIDTH, ORDER, SIZE};
use std::{array, fmt};

#[derive(Clone, Copy, Debug)]
pub enum Cell {
    Given(u8),
    Filled(u8),
    Empty,
}

impl PartialEq for Cell {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Empty, Self::Empty) => true,
            (Self::Filled(n), Self::Filled(m)) => n == m,
            (Self::Given(n), Self::Given(m)) => n == m,
            _ => false,
        }
    }
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

fn row_pos_to_box(i: usize, j: usize) -> (usize, usize) {
    (
        (i / ORDER) * ORDER + j / ORDER,
        (i % ORDER) * ORDER + j % ORDER,
    )
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
        let cols = array::from_fn(|i| array::from_fn(|j| rows[j][i]));
        let boxes = array::from_fn(|i| {
            array::from_fn(|j| {
                let (box_i, box_j) = row_pos_to_box(i, j);
                rows[box_i][box_j]
            })
        });

        Grid { rows, cols, boxes }
    }

    pub fn rows(&self) -> &[Group; SIZE] {
        &self.rows
    }

    pub fn cols(&self) -> &[Group; SIZE] {
        &self.cols
    }

    pub fn boxes(&self) -> &[Group; SIZE] {
        &self.boxes
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

    pub fn get_cell(&self, pos: CellCoord) -> &Cell {
        &self.rows[pos.row][pos.col]
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
