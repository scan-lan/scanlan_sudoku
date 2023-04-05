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

pub fn solve_backtracking_heuristics(mut g: Grid) -> Grid {
    let mut history: Vec<Decision> = Vec::new();

    while g.empty_cell_count > 0 {
        let target = g.get_min_candidates_cell();
        let choice = g.collapse(target);

        match g.update(target, choice) {
            Ok(_) => {
                history.push(Decision::new(target, choice));
            }
            Err(_) => {
                g.remove_candidate(target, choice);
                continue;
            }
        }
    }
    g
}
