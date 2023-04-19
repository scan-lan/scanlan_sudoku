use super::{puzzle::Coord, Cell, Grid};

/// Represents a decision taken by the algorithm
pub struct Decision {
    pub cell: Coord,
    pub val: Cell,
    pub candidates_changed: Vec<Coord>,
    pub prev_cell_candidates: Vec<u8>,
}

impl Decision {
    fn new(
        cell: Coord,
        val: Cell,
        candidates_changed: Vec<Coord>,
        prev_cell_candidates: Vec<u8>,
    ) -> Self {
        Decision {
            cell,
            val,
            candidates_changed,
            prev_cell_candidates,
        }
    }
}

impl std::fmt::Debug for Decision {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "<({}, {}), {}>", self.cell.row, self.cell.col, self.val)?;
        Ok(())
    }
}

pub fn solve_backtracking_heuristics(mut g: Grid) -> Option<Grid> {
    let mut history: Vec<Decision> = Vec::new();

    'outer: while !g.solved {
        // Get cell with least valid candidates
        let target = g.get_min_candidates_cell();
        let prev_cell_candidates = g.candidates_at(target);

        // Iterate over all candidates
        for val in g.candidates_at(target).iter() {
            if let Ok(candidates_changed) = g.update(target, *val) {
                // If candidate valid, push decision onto history stack; continue while loop
                history.push(Decision::new(
                    target,
                    Cell::Filled(*val),
                    candidates_changed,
                    prev_cell_candidates,
                ));
                continue 'outer;
            }
        }

        // No values were accepted in the for loop, so undo last decision
        if let Some(last_decision) = history.pop() {
            g.undo(last_decision);
        }
        println!("{g}");
    }
    Some(g)
}
