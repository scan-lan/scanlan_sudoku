pub const ORDER: usize = 3;
pub const SIZE: usize = ORDER.pow(2);
const NUM_WIDTH: u32 = SIZE.ilog10() + 1;
const CELL_WIDTH: u32 = NUM_WIDTH + 1;

pub use grid::{get_base_solution, Cell, Grid};
pub use grid_trait::DisplayableGrid;
pub use puzzle::{Coord, Puzzle};
pub use solver::solve_backtracking_heuristics;
pub use types::Move;

type Group = [grid::Cell; SIZE];

mod candidate_matrix;
mod grid;
mod grid_trait;
mod puzzle;
mod solver;
mod types;
