use std::cmp::Ordering;

use super::{puzzle::Coord, Grid};

/// Solve grid `g` using a backtracking algorithm with heurisitics
pub fn solve_backtracking_heuristics(mut g: Grid) -> Option<Grid> {
    let mut history: Vec<(Grid, u8, Coord)> = Vec::new();

    'outer: while !g.solved {
        // Get cell with least valid candidates
        let target = g.get_min_candidates_cell();
        let candidates = g.candidates_at(target);

        match candidates.len().cmp(&1) {
            Ordering::Greater => {
                let backtrack = g.clone();

                for val in candidates.clone().into_iter() {
                    if g.update(target, val).is_ok() {
                        // If candidate valid, push decision onto history stack; continue while loop
                        history.push((backtrack, val, target));
                        continue 'outer;
                    }
                }
            }
            Ordering::Equal => {
                if g.update(target, candidates[0]).is_err() {
                    // println!("{e}");
                    let dec = history.pop().expect("Shouldn't run out of history");
                    g = dec.0;

                    g.remove_candidate(dec.2, dec.1);
                }
            }
            Ordering::Less => panic!("This should never happen"),
        }
    }

    Some(g)
}
