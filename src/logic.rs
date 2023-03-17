pub use sudoku::Puzzle;

pub fn run() {
    let p = Puzzle::new();
    println!("{}", p.solution());
}

mod sudoku {
    use std::fmt;

    use crate::{CELL_WIDTH, ORDER, SIZE};

    #[derive(Clone, Copy, Debug, PartialEq)]
    pub enum Cell {
        Given(u8),
        Filled(u8),
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

    type Group = [Cell; SIZE];

    #[derive(Debug)]
    struct Grid([Group; SIZE]);

    impl fmt::Display for Grid {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            let cell_width =
                usize::try_from(CELL_WIDTH).expect("cell width should always be small");
            let box_width: usize = cell_width * ORDER + cell_width - 1;

            for (i, row) in self.0.iter().enumerate() {
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

    impl Grid {
        pub fn get_row(&self, idx: usize) -> &Group {
            &self.0[idx]
        }
    }

    #[derive(Debug)]
    pub struct Puzzle {
        grid: Grid,
        solution: Grid,
    }

    impl Puzzle {
        pub fn new() -> Puzzle {
            Puzzle {
                grid: Grid([[Cell::Empty; SIZE]; SIZE]),
                solution: get_base_solution(),
            }
        }

        pub fn grid(&self) -> String {
            format!("{}", self.grid)
        }

        pub fn solution(&self) -> String {
            format!("{}", self.solution)
        }

        pub fn get_row(&self, idx: usize) -> &Group {
            self.grid.get_row(idx)
        }
    }

    fn get_base_solution() -> Grid {
        let mut base_solution = [[Cell::Empty; SIZE]; SIZE];
        for i in 0..SIZE {
            for j in 0..SIZE {
                base_solution[i][j] =
                    Cell::Given((1 + (j + (i / ORDER) + (i % ORDER) * ORDER) % SIZE) as u8);
            }
        }

        Grid(base_solution)
    }

    #[cfg(test)]
    mod puzzle_tests {
        use crate::SIZE;

        use super::{Cell, Puzzle};

        #[test]
        fn create_empty_puzzle() {
            let p = Puzzle::new();
            let expected_grid = [[Cell::Empty; SIZE]; SIZE];

            assert_eq!(p.grid.0, expected_grid);
        }

        #[test]
        fn get_grid_as_string() {
            let expected = String::from(
                " 1 2 3 | 4 5 6 | 7 8 9
 4 5 6 | 7 8 9 | 1 2 3
 7 8 9 | 1 2 3 | 4 5 6
-------+-------+-------
 2 3 4 | 5 6 7 | 8 9 1
 5 6 7 | 8 9 1 | 2 3 4
 8 9 1 | 2 3 4 | 5 6 7
-------+-------+-------
 3 4 5 | 6 7 8 | 9 1 2
 6 7 8 | 9 1 2 | 3 4 5
 9 1 2 | 3 4 5 | 6 7 8
",
            );
            let p = Puzzle::new();

            assert_eq!(p.solution(), expected);
        }
    }

    #[cfg(test)]
    mod grid_tests {
        use super::{get_base_solution, Cell};

        #[test]
        fn get_row() {
            let expected: Group =
                core::array::from_fn(|i| Cell::Given((i + 1).try_into().unwrap()));
            let p = get_base_solution();

            assert_eq!(p.get_row(0), &expected);
        }
    }
}
