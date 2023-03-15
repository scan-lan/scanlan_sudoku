mod sudoku {
    use std::fmt;

    type Grid = [[Cell; 9]; 9];

    #[derive(Clone, Copy, Debug, PartialEq)]
    pub enum Cell {
        Given(u32),
        Filled(u32),
        Empty,
    }

    impl fmt::Display for Cell {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            match self {
                Self::Given(n) => write!(f, "{n}"),
                Self::Filled(n) => write!(f, "{n}"),
                Self::Empty => write!(f, " "),
            }
        }
    }

    #[derive(Debug)]
    pub struct Puzzle {
        grid: Grid,
        solution: [[u8; 9]; 9],
    }

    impl Puzzle {
        pub fn new() -> Puzzle {
            Puzzle {
                grid: [[Cell::Empty; 9]; 9],
                solution: [[0; 9]; 9],
            }
        }

        fn grid(&self) -> &Grid {
            &self.grid
        }

        // for (let i = 0; i < 9; i++) {
        //   for (let j = 0; j < 9; j++) {
        //     const value = (j + (Math.floor(i / 3) + 1) + (i % 3) * 3) % 9;
        fn solution() -> [[u8; 9]; 9] {
            let mut base_solution = [[0u8; 9]; 9];
            for i in 0..9 {
                for j in 0..9 {
                    base_solution[i][j] = ((j + (i / 3 + 1) + (i % 3) * 3) % 9) as u8;
                }
            }

            base_solution
        }
    }

    #[cfg(test)]
    mod grid_tests {
        use super::{Cell, Puzzle};

        #[test]
        fn create_empty_puzzle() {
            let p = Puzzle::new();
            let expected_grid = [[Cell::Empty; 9]; 9];

            assert_eq!(p.grid, expected_grid);
        }

        #[test]
        fn build_puzzle() {
            todo!()
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
    println!("{:?}", &p);
}
