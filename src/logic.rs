mod sudoku {
    use std::fmt;

    use crate::{CELL_WIDTH, ORDER, SIZE};

    type Grid = [[Cell; SIZE]; SIZE];

    #[derive(Clone, Copy, Debug, PartialEq)]
    pub enum Cell {
        Given(u32),
        Filled(u32),
        Empty,
    }

    impl fmt::Display for Cell {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            let width = usize::try_from(CELL_WIDTH).expect("cell width should always be small");
            match self {
                Self::Given(n) => write!(f, "{:width$}", n),
                Self::Filled(n) => write!(f, "{:width$}", n),
                Self::Empty => write!(f, "{:width$}", " "),
            }
        }
    }

    #[derive(Debug)]
    pub struct Puzzle {
        grid: Grid,
        solution: [[u8; SIZE]; SIZE],
    }

    impl Puzzle {
        pub fn new() -> Puzzle {
            let mut base_solution = [[0u8; SIZE]; SIZE];
            for i in 0..SIZE {
                for j in 0..SIZE {
                    base_solution[i][j] =
                        (1 + (j + (i / ORDER) + (i % ORDER) * ORDER) % SIZE) as u8;
                }
            }

            Puzzle {
                grid: [[Cell::Empty; SIZE]; SIZE],
                solution: base_solution,
            }
        }

        fn grid(&self) -> &Grid {
            &self.grid
        }

        pub fn solution(&self) -> &[[u8; SIZE]; SIZE] {
            &self.solution
        }
    }

    impl fmt::Display for Puzzle {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            let cell_width =
                usize::try_from(CELL_WIDTH).expect("cell width should always be small");
            let box_width: usize = cell_width * ORDER + cell_width - 1;

            for (i, row) in self.solution.iter().enumerate() {
                for (j, cell) in row.iter().enumerate() {
                    write!(f, "{:>cell_width$}", cell)?;
                    if j != SIZE - 1 && j % ORDER == ORDER - 1 {
                        write!(f, "{:>cell_width$}", "|")?;
                    }
                }
                write!(f, "\n")?;
                if i != SIZE - 1 && i % ORDER == ORDER - 1 {
                    let line = format!("{:->box_width$}", "-");
                    for _ in 1..ORDER {
                        write!(f, "{line}+")?;
                    }
                    writeln!(f, "{line}")?;
                }
            }
            Ok(())
        }
    }

    #[cfg(test)]
    mod grid_tests {
        use crate::SIZE;

        use super::{Cell, Puzzle};

        #[test]
        fn create_empty_puzzle() {
            let p = Puzzle::new();
            let expected_grid = [[Cell::Empty; SIZE]; SIZE];

            assert_eq!(p.grid, expected_grid);
        }

        #[test]
        fn fill_cell() {
            todo!()
        }
    }
}

pub use sudoku::Puzzle;

pub fn run() {
    let mut p = Puzzle::new();
    println!("{p}");
}
