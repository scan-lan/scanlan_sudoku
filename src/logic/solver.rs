use std::{cmp::Ordering, time::Duration};

use super::{puzzle::Coord, Grid};
const DEBUG_MODE: bool = false;

pub fn solve_backtracking_heuristics(mut g: Grid) -> Option<Grid> {
    let mut history: Vec<(Grid, u8, Coord)> = Vec::new();
    let mut i = 0;

    'outer: while !g.solved {
        if DEBUG_MODE {
            std::thread::sleep(Duration::from_millis(200));
            i += 1;
            println!("{:=^79}", format!("Iteration {i}"),);
        }

        // Get cell with least valid candidates
        let target = g.get_min_candidates_cell();
        let candidates = g.candidates_at(target);

        if DEBUG_MODE {
            println!("Grid:\n{g}");
            println!("Candidate Matrix:\n{}", g.candidate_matrix());
            println!("Solving {target}");
            println!("Candidates: {:?}", candidates);
            // println!("History: {:?}", history);
        }

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
            },
            Ordering::Less => panic!("This should never happen"),
        }
    }

    Some(g)
}
