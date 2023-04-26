pub use generator::{generate, Difficulty};
pub use grid::{get_base_solution, Cell, Grid};
pub use grid_trait::DisplayableGrid;
pub use puzzle::{Coord, Puzzle};
pub use solver::solve_backtracking_heuristics;
pub use types::Move;

pub use crate::{CELL_WIDTH, NUM_WIDTH, ORDER, SIZE};

type Group = [grid::Cell; SIZE];

mod candidate_matrix;
mod generator;
mod grid;
mod grid_trait;
mod puzzle;
mod solver;
mod types;
