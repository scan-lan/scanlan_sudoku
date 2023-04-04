use super::{puzzle::Coord, Cell};

pub struct Move {
    pub cell: Coord,
    pub old: Cell,
    pub new: Cell,
}

impl Move {
    pub fn new(cell: Coord, old: Cell, new: Cell) -> Self {
        Move { cell, old, new }
    }
}
