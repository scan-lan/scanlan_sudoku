use std::io;

use crate::logic::{solve_backtracking_heuristics, Cell, Coord, DisplayableGrid, SIZE};

use crate::logic::Grid;

pub fn run() {
    let g = grid_from_input();
    println!("{}", g);

    let g_solved = solve_backtracking_heuristics(g);
    if let Some(g) = g_solved {
        println!("{}", g);
    }
}

pub fn grid_from_input() -> Grid {
    // get input etc etc
    let mut new_grid = DisplayableGrid([[Cell::Empty; SIZE]; SIZE]);
    for (i, row) in new_grid.0.iter_mut().enumerate() {
        for (j, cell) in row.iter_mut().enumerate() {
            *cell = Cell::Given(0);

            let mut display_grid = DisplayableGrid([[Cell::Empty; SIZE]; SIZE]);
            display_grid.0[i][j] = Cell::Given(0);
            println!("{}", display_grid);
            println!(
                "Please enter value for cell {}, marked with a '?'",
                Coord::from((i + 1, j + 1))
            );
            *cell = prompt_for_value();
        }
    }

    Grid::from(new_grid.0)
}

fn prompt_for_value() -> Cell {
    let mut val = Cell::Given(0);

    while val == Cell::Given(0) {
        let mut response = String::new();
        if let Err(e) = io::stdin().read_line(&mut response) {
            println!("Unexpected error: {e}\nPlease try again");
            continue;
        }
        let response = response.trim();

        if response.is_empty() {
            val = Cell::Empty;
        }

        if let Ok(n) = response.parse::<u8>() {
            if (1..=SIZE).contains(&(n as usize)) {
                val = Cell::Given(n);
                break;
            } else {
                println!("Please enter a number in the range 1-{SIZE}");
                continue;
            }
        }
    }
    val
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
