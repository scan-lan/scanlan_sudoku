pub const ORDER: usize = 2;
pub const SIZE: usize = ORDER.pow(2);
const CELL_WIDTH: u32 = SIZE.ilog10() + 2;

pub mod logic;
pub mod ui;

pub use logic::run;
