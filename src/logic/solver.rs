use std::cmp::Ordering;

use super::{Coord, Grid};

/// Struct to model a decision point in the algorithm.
struct Decision {
    state: Grid,
    val: u8,
    target: Coord,
}

/// Solve grid `g` using a backtracking algorithm with heurisitics
pub fn solve_backtracking_heuristics(mut g: Grid) -> Option<Grid> {
    let mut history: Vec<Decision> = Vec::new();

    'outer: while !g.solved {
        // Get cell with least valid candidates
        let target = g.get_min_candidates_cell();
        let candidates = g.candidates_at(target);

        match candidates.len().cmp(&1) {
            // If the cell has more than one candidate, the choice we make
            // may be wrong, so we store the current state of the grid.
            Ordering::Greater => {
                let backtrack = g.clone();

                for val in candidates.clone().into_iter() {
                    if g.update(target, val).is_ok() {
                        // If candidate valid, push decision onto history stack; continue while loop
                        history.push(Decision {
                            state: backtrack,
                            val,
                            target,
                        });

                        continue 'outer;
                    }
                }
            }
            // If the cell has exactly one candidate, that's not recorded as
            // a decision point, because we're choosing the only option.
            // However, if updating the grid with that option fails,
            // we know a previous decision was incorrect, so we backtrack.
            Ordering::Equal => {
                if g.update(target, candidates[0]).is_err() {
                    let dec = history.pop().expect("Shouldn't run out of history");

                    // Set grid to the state at the last decision point, then
                    // remove the value that was chosen for the target as a
                    // candidate, because it led to an error.
                    g = dec.state;
                    g.remove_candidate(dec.target, dec.val);
                }
            }
            Ordering::Less => panic!("This should never happen"),
        }
    }

    Some(g)
}
