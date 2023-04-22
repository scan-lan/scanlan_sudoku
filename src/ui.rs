use std::io;
use std::time::Instant;

use crate::logic::{solve_backtracking_heuristics, Cell, Coord, DisplayableGrid, SIZE};

use crate::logic::Grid;
// use crate::puzzles::HARD_GRID;

pub fn run() {
    let g = grid_from_input();
    // let g = Grid::from(HARD_GRID);
    // let g = Grid::new();
    match g {
        None => {
            println!("Thank you for playing.");
            return;
        },
        Some(g) => {
            println!("{}", g);
            println!("{}", g.candidate_matrix());

            let now = Instant::now();
            let g_solved = solve_backtracking_heuristics(g);
            println!("Solved in {:?}", now.elapsed());

            if let Some(g) = g_solved {
                println!("{}", g);
            }
        }
    }
}

/// Transforms a Vec into a grid. This will panic if the Vec is too small.
fn grid_from_vec(v: Vec<Cell>) -> DisplayableGrid<Cell> {
    DisplayableGrid(std::array::from_fn(|i| std::array::from_fn(|j| v[i * SIZE + j])))
}

/// Obtains a grid from user input. Returns `None` if the user quits.
pub fn grid_from_input() -> Option<Grid> {
    let mut input: Vec<Cell> = Vec::new();

    let mut display_grid = DisplayableGrid([[Cell::Empty; SIZE]; SIZE]);
    while input.len() != SIZE.pow(2) {
        let (i, j) = (input.len() / SIZE, input.len() % SIZE);
        display_grid.0[i][j] = Cell::Clue(0);
        println!("{}", display_grid);
        println!(
            "Please enter value for cell {}, marked with a '?' ('u': undo; 'q': quit)",
            Coord::from((i + 1, j + 1))
        );

        match prompt_for_value() {
            PromptResponse::Val(c) => {
                display_grid.0[i][j] = c;
                input.push(c);
            },
            PromptResponse::Quit => {
                return None;
            }
            PromptResponse::Undo => {
                input.pop();
                display_grid.0[i][j] = Cell::Empty;
            }
        }
    }

    let new_g = grid_from_vec(input).into();
    Some(new_g)
}

#[derive(Debug)]
enum PromptResponse<T> {
    Quit,
    Undo,
    Val(T),
}

fn prompt_for_value() -> PromptResponse<Cell> {
    loop {
        let mut response = String::new();
        if let Err(e) = io::stdin().read_line(&mut response) {
            println!("Unexpected error: {e}\nPlease try again");
            continue;
        }
        let response = response.trim();

        if response.is_empty() {
            return PromptResponse::Val(Cell::Empty);
        }

        match response {
            "u" => {
                return PromptResponse::Undo;
            }
            "q" => {
                return PromptResponse::Quit;
            }
            _ => {
                if let Ok(n) = response.parse::<u8>() {
                    if (1..=SIZE).contains(&(n as usize)) {
                        return PromptResponse::Val(Cell::Clue(n));
                    }
                }
                println!("Please enter a value between 1 and {SIZE}");
            }
        }
    }
}

// fn prompt(opts: &str, default: char) -> char {
//     default
// }

// pub fn game_loop() {
//     // solve or play
//     let choice = prompt("ps", 'p');

//     if choice == 'p' {
//         // difficulty
//         let difficulty = prompt("emh", 'm');

//         // let p = Puzzle::new(difficulty);

//         loop {
//             // let coord = get_coord();
//             // let val = get_val();
//             // p.update(coord, val);
//         }
//     }
// }
