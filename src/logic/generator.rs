use super::{solve_backtracking_heuristics, Grid, Puzzle};

pub enum Difficulty {
    Easy,
    Medium,
    Hard,
}

pub fn generate(d: Difficulty) -> Grid {
    let g = solve_backtracking_heuristics(Grid::new()).unwrap();
    println!("{g}");
    g
}
