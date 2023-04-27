pub use cell::Cell;
pub use coord::Coord;
pub use generator::{generate, Difficulty};
pub use grid::{get_base_solution, Grid};
pub use grid_trait::DisplayableGrid;
pub use solver::solve_backtracking_heuristics;

pub use crate::{CELL_WIDTH, NUM_WIDTH, ORDER, SIZE};

pub type GridArray = [[Cell; SIZE]; SIZE];

mod candidate_matrix;
mod cell;
mod coord;
mod generator;
mod grid;
mod grid_trait;
mod solver;
