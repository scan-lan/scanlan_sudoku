use std::array;

use super::{grid::row_coords_to_box_coords, SIZE};

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
