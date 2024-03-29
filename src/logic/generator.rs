use std::{
    array,
    time::{Duration, Instant},
};

use crate::logic::{Coord, SIZE};

use super::{solve_backtracking_heuristics, Grid};

use rand::seq::{IteratorRandom, SliceRandom};

#[derive(Debug, Copy, Clone)]
pub enum Difficulty {
    Easy,
    Medium,
    Hard,
}

pub fn generate(d: Difficulty) -> (Grid, Duration) {
    let now = Instant::now();
    let mut g = solve_backtracking_heuristics(Grid::new()).unwrap();
    let coords: [[Coord; SIZE]; SIZE] = array::from_fn(|i| array::from_fn(|j| (i, j).into()));
    let coords = coords.flatten();

    let to_delete = match d {
        Difficulty::Easy => (40..45)
            .choose(&mut rand::thread_rng())
            .unwrap_or(43),
        Difficulty::Medium => (46..50)
            .choose(&mut rand::thread_rng())
            .unwrap_or(47),
        Difficulty::Hard => (51..56)
            .choose(&mut rand::thread_rng())
            .unwrap_or(53),
    };

    coords
        .choose_multiple(&mut rand::thread_rng(), to_delete)
        .for_each(|coord| {
            let _ = g.clear(*coord);
        });
    g.set_clues();
    g.reset_candidate_matrix();

    (g, now.elapsed())
}
