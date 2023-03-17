pub const ORDER: usize = 3;
pub const SIZE: usize = ORDER.pow(2);
const CELL_WIDTH: u32 = SIZE.ilog10() + 2;

pub use grid::{get_base_solution, Grid};
pub use puzzle::Puzzle;

type Group = [grid::Cell; SIZE];

pub fn run() {
    let p = Puzzle::new();
    println!("{}", p.solution());
}

mod grid;
mod puzzle;
