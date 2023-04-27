use crate::ui::utils::format_time;
use crate::ui::utils::HOW_TO;
use crate::ui::utils::SMALL_TITLE;
use crate::ui::utils::THANK_YOU;
use std::collections::BTreeMap;
use std::sync::mpsc;
use std::thread;
use std::time;

use crate::logic::{
    generate, solve_backtracking_heuristics, Cell, Coord, Difficulty, DisplayableGrid, SIZE,
};

use crate::logic::Grid;
use crate::ui::utils::{
    get_char_response, get_move, get_num_response, prompt_for_value, PromptResponse,
};

use self::utils::char_prompt;

/// The main entry point of the game
pub fn run() {
    loop {
        let choice = main_menu();

        match choice {
            Choice::Solve => solve(),
            Choice::Play => play(),
            Choice::Quit => break,
        }
    }
}

#[derive(Debug, Copy, Clone)]
enum Choice {
    Play,
    Solve,
    Quit,
}

/// This main menu handles which of the high-level activities a player would
/// like to do: play a puzzle or enter one to be solved.
fn main_menu() -> Choice {
    println!("{}{SMALL_TITLE}{}", "\n".repeat(4), "\n".repeat(4));
    let map = BTreeMap::from([
        ('p', Choice::Play),
        ('s', Choice::Solve),
        ('q', Choice::Quit),
    ]);
    println!(
        "[P]lay a game or enter a puzzle to be [s]olved?\n\n(Enter \"q\" at any time to quit)"
    );

    char_prompt("What would you like to do?", map, Some('p'))
}

/// The "play" sub-menu of the main menu. This obtains a player's difficulty
/// selection, as well as asking if they want a time limit.
fn play() {
    if let Some(difficulty) = difficulty_menu() {
        println!(
            "\nGenerating a puzzle with difficulty: {}",
            match difficulty {
                Difficulty::Easy => "easy",
                Difficulty::Medium => "medium",
                Difficulty::Hard => "hard",
            }
        );
        let (g, time_taken) = generate(difficulty);
        println!("Took {:?}\n", time_taken);

        let time_constraint = time_menu();
        match game_loop(g, time_constraint) {
            Game::Solved(solve_time) => {
                let solve_secs = solve_time.as_secs();
                println!(
                    "Congratulations! You solved the puzzle in {}",
                    format_time(solve_secs)
                );
            }
            Game::Quit => println!("{THANK_YOU}"),
        }
    } else {
        println!("{THANK_YOU}");
    }
}

/// A menu for asking the player what difficulty they'd like to play.
fn difficulty_menu() -> Option<Difficulty> {
    let msg = "\nSelect puzzle difficulty:\n\n- Easy [e]\n- Medium [m]\n- Hard[h]\n";
    let map = BTreeMap::from([
        ('e', Some(Difficulty::Easy)),
        ('m', Some(Difficulty::Medium)),
        ('h', Some(Difficulty::Hard)),
        ('q', None),
    ]);

    char_prompt(msg, map, Some('m'))
}

/// A menu for asking if the player want a time limit. Returns `None` if they
/// don't, or a duration based on the answer they give in minutes.
fn time_menu() -> Option<time::Duration> {
    println!("Would you like to set a time limit on your game? [y/n]");
    loop {
        let r = get_char_response("> ");
        match r {
            'y' => {
                println!("Enter the time constraint you wish to set in minutes.");
                return Some(time::Duration::from_secs(get_num_response("> ") * 60));
            }
            'n' => return None,
            _ => println!("Please enter 'y' or 'n'"),
        }
    }
}

/// Enum to model the result of a game
enum Game {
    Solved(time::Duration),
    Quit,
}

/// The main game loop for when a player is solving a puzzle. Includes
/// undo/redo support, time constraints, and sophisticated input validation.
fn game_loop(mut g: Grid, time_constraint: Option<time::Duration>) -> Game {
    let mut undo_history: Vec<Grid> = Vec::new();
    let mut redo_history: Vec<Grid> = Vec::new();
    let now = time::Instant::now();
    let (tx, rx) = mpsc::channel();

    // spawn a thread to track the elapsed time in the background
    if let Some(duration) = time_constraint {
        thread::spawn(move || {
            let now = time::Instant::now();
            while now.elapsed() < duration {
                thread::sleep(time::Duration::from_millis(500));
            }
            let _ = tx.send(true);
        });
    }

    println!("{g}\n\n{HOW_TO}\n");

    while !g.solved {
        match get_move(&g) {
            PromptResponse::Val((cell, val)) => {
                if let Cell::Filled(n) = val {
                    let cur_g = g.clone();
                    if let Err(e) = g.update(cell, n) {
                        println!("{e}");
                    } else {
                        undo_history.push(cur_g);
                        redo_history.clear();
                        println!("\n{g}\n");
                    }
                }
            }

            PromptResponse::Undo => match undo_history.pop() {
                Some(last_g) => {
                    redo_history.push(g.clone());
                    g = last_g;
                    println!("Move undone:\n\n{g}");
                }
                None => {
                    println!("No more moves to undo");
                }
            },

            PromptResponse::Redo => match redo_history.pop() {
                Some(next_g) => {
                    undo_history.push(g);
                    g = next_g;
                    println!("Move redone:\n\n{g}");
                }
                None => println!("No more moves to redo"),
            },

            PromptResponse::Quit => return Game::Quit,
        }

        if time_constraint.is_some() {
            if let Ok(_) = rx.try_recv() {
                println!("Bad luck, you're out of time!");
                return Game::Quit;
            }
        }
    }
    Game::Solved(now.elapsed())
}

/// The "solve" sub-menu of the main menu. This obtains a grid from the player
/// and proceeds to solve it.
fn solve() {
    match grid_from_input() {
        Some(g) => {
            println!("{}", g);

            let now = time::Instant::now();
            let g_solved = solve_backtracking_heuristics(g);
            println!("Solved in {:?}", now.elapsed());

            if let Some(g) = g_solved {
                println!("{}", g);
            }
        }
        None => {
            println!("{THANK_YOU}");
        }
    }
}

/// Transforms a Vec into a grid. This will panic if the Vec is too small.
fn grid_from_vec(v: Vec<Cell>) -> DisplayableGrid<Cell> {
    DisplayableGrid(std::array::from_fn(|i| {
        std::array::from_fn(|j| v[i * SIZE + j])
    }))
}

/// Obtains a grid from user input. Returns `None` if the user quits.
pub fn grid_from_input() -> Option<Grid> {
    let mut input: Vec<Cell> = Vec::new();
    let mut redo_stack = Vec::new();

    let mut display_grid = DisplayableGrid([[Cell::Empty; SIZE]; SIZE]);
    while input.len() != SIZE.pow(2) {
        let (i, j) = (input.len() / SIZE, input.len() % SIZE);
        display_grid.0[i][j] = Cell::Clue(0);
        println!("\n{}\n", display_grid);
        let prompt = format!(
            "Please enter value for cell {}, marked with a '?' ([u]ndo, [r]edo, [q]uit)\n> ",
            Coord::from((i + 1, j + 1))
        );

        match prompt_for_value(&prompt, true) {
            PromptResponse::Val(c) => {
                display_grid.0[i][j] = c;
                input.push(c);
                redo_stack.clear();
            }
            PromptResponse::Quit => {
                return None;
            }
            PromptResponse::Undo => match input.pop() {
                None => println!("No more moves to undo"),
                Some(last_move) => {
                    redo_stack.push(last_move);
                    display_grid.0[i][j] = Cell::Empty;
                }
            },
            PromptResponse::Redo => match redo_stack.pop() {
                None => println!("No more moves to redo"),
                Some(next_move) => {
                    input.push(next_move);
                    display_grid.0[i][j] = next_move;
                }
            },
        }
    }

    let new_g = grid_from_vec(input).into();
    Some(new_g)
}

mod utils;
