use super::{puzzle::Coord, Cell, Grid};

/// Represents a decision taken by the algorithm
pub struct Decision {
    pub cell: Coord,
    pub new: Cell,
    pub candidates_changed: Vec<Coord>,
}

impl Decision {
    fn new(cell: Coord, val: Cell, candidates_changed: Vec<Coord>) -> Self {
        Decision {
            cell,
            new: val,
            candidates_changed,
        }
    }
}

pub fn solve_backtracking_heuristics(mut g: Grid) -> Option<Grid> {
    let mut history: Vec<Decision> = Vec::new();

    'outer: while g.empty_cell_count > 0 && !g.solved {
        // Get cell with least valid candidates
        let target = g.get_min_candidates_cell();

        // Iterate over all candidates
        for val in g.candidates_at(target).iter() {
            if let Ok(candidates_changed) = g.update(target, *val) {
                // If candidate valid, push decision onto history stack; continue while loop
                history.push(Decision::new(
                    target,
                    Cell::Filled(*val),
                    candidates_changed,
                ));
                continue 'outer;
            }
        }

        // No values were accepted in the for loop, so undo last decision
        if let Some(last_decision) = history.pop() {
            g.undo(last_decision);
        }
    }
    Some(g)
}
