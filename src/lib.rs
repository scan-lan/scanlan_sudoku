#![feature(slice_flatten)]
pub mod logic;
pub mod ui;
// pub mod puzzles;

pub use ui::run;

pub const ORDER: usize = 4;
pub const SIZE: usize = ORDER.pow(2);
pub const NUM_WIDTH: u32 = SIZE.ilog10() + 1;
pub const CELL_WIDTH: u32 = NUM_WIDTH + 1;
