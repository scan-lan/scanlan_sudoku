use super::{Cell, Coord};

/// Represents a decision taken by the algorithm
pub struct Decision {
    pub cell: Coord,
    pub val: Cell,
    pub candidates_changed: Vec<Coord>,
    pub prev_cell_candidates: Vec<u8>,
    pub forced: bool,
}

impl Decision {
    fn new(
        cell: Coord,
        val: Cell,
        candidates_changed: Vec<Coord>,
        prev_cell_candidates: Vec<u8>,
        forced: bool,
    ) -> Self {
        Decision {
            cell,
            val,
            candidates_changed,
            prev_cell_candidates,
            forced,
        }
    }
}

impl std::fmt::Debug for Decision {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "<({}, {}), {}>", self.cell.row, self.cell.col, self.val)?;
        Ok(())
    }
}
