use std::{array, fmt};

use super::{grid::row_coords_to_box_coords, CELL_WIDTH, ORDER, SIZE};

/// Trait that allows transforming from a representation of a Sudoku grid with
/// inner arrays as rows into one where they're columns or boxes.
pub trait GridTrait<T> {
    fn cols(&self) -> [[T; SIZE]; SIZE];
    fn boxes(&self) -> [[T; SIZE]; SIZE];
}

impl<T: Clone> GridTrait<T> for [[T; SIZE]; SIZE] {
    fn cols(&self) -> [[T; SIZE]; SIZE] {
        array::from_fn(|i| array::from_fn(|j| self[j][i].clone()))
    }

    fn boxes(&self) -> [[T; SIZE]; SIZE] {
        array::from_fn(|i| {
            array::from_fn(|j| {
                let (box_i, box_j) = row_coords_to_box_coords((i, j).into()).into();
                self[box_i][box_j].clone()
            })
        })
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
/// Wrapper around the standard array type that enables printing it as a
/// Sudoku grid.
pub struct DisplayableGrid<T>(pub [[T; SIZE]; SIZE]);

impl<T: fmt::Display> fmt::Display for DisplayableGrid<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let cell_width = CELL_WIDTH as usize;
        let box_width: usize = cell_width * ORDER + cell_width - 1;
        let gutter_width = cell_width * 2;

        for (i, row) in self.0.iter().enumerate() {
            write!(f, "{:^gutter_width$}", i + 1)?;
            for (j, cell) in row.iter().enumerate() {
                write!(f, "{:>cell_width$}", cell)?;
                if j != SIZE - 1 && j % ORDER == ORDER - 1 {
                    write!(f, "{:>cell_width$}", "|")?;
                }
            }

            writeln!(f)?;
            if i == SIZE - 1 {
                // offset by cell width to fit the numbers down the side
                write!(f, "\n{:^gutter_width$}", " ")?;
                for col_index in 1..=SIZE {
                    write!(f, "{:>cell_width$}", col_index)?;
                    if col_index != SIZE && (col_index - 1) % ORDER == ORDER - 1 {
                        write!(f, "{:>cell_width$}", "|")?;
                    }
                }
            } else if i % ORDER == ORDER - 1 {
                // offset to fit the numbers down the side
                write!(f, "{:^gutter_width$}", " ")?;
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
