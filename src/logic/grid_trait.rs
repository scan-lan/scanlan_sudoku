use std::{array, fmt};

use super::{grid::row_coords_to_box_coords, CELL_WIDTH, ORDER, SIZE};

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
pub struct DisplayableGrid<T>(pub [[T; SIZE]; SIZE]);

impl<T: fmt::Display> fmt::Display for DisplayableGrid<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let cell_width = CELL_WIDTH as usize;
        let box_width: usize = cell_width * ORDER + cell_width - 1;

        for (i, row) in self.0.iter().enumerate() {
            for (j, cell) in row.iter().enumerate() {
                write!(f, "{:>cell_width$}", cell)?;
                if j != SIZE - 1 && j % ORDER == ORDER - 1 {
                    write!(f, "{:>cell_width$}", "|")?;
                }
            }
            writeln!(f)?;
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
