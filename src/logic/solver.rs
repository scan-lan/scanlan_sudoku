use super::{puzzle::Coord, Grid};

/// Represents a decision taken by the algorithm
struct Decision {
    pub cell: Coord,
    pub val: u8,
}

impl Decision {
    fn new(cell: Coord, val: u8) -> Self {
        Decision { cell, val }
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
