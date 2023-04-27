use std::cmp::Ordering;
use std::collections::BTreeMap;
use std::io::{self, Write};
use std::sync::mpsc;
use std::thread;
use std::time;

use crate::logic::{
    generate, solve_backtracking_heuristics, Cell, Coord, Difficulty, DisplayableGrid, SIZE,
};

use crate::logic::Grid;
use lazy_static::lazy_static;
use regex::Regex;

pub fn run() {
    let choice = main_menu();

    match choice {
        Choice::Solve => solve(),
        Choice::Play => play(),
        Choice::Quit => {}
    }
}

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

fn time_menu() -> Option<time::Duration> {
    println!("Would you like to set a time limit on your game? [y/n]");
    loop {
        let r = get_char_response("n> ");
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

fn format_time(t: u64) -> String {
    let secs = t % 60;
    let show_s_secs = if secs != 1 { "s" } else { "" };
    let mins = t % 3600 / 60;
    let show_s_mins = if mins != 1 { "s" } else { "" };
    let hrs = t / 3600;
    let show_s_hrs = if hrs != 1 { "s" } else { "" };

    match t {
        0..=59 => format!("{secs} second{show_s_secs}"),
        60..=3599 => format!("{mins} minute{show_s_mins}, {secs} second{show_s_secs}"),
        3600.. => format!(
            "{hrs} hour{show_s_hrs}, {mins} minute{show_s_mins}, {secs} second{show_s_secs}"
        ),
    }
}

enum Game {
    Solved(time::Duration),
    Quit,
}

/// The main
fn game_loop(mut g: Grid, time_constraint: Option<time::Duration>) -> Game {
    let mut undo_history: Vec<Grid> = Vec::new();
    let mut redo_history: Vec<Grid> = Vec::new();
    let now = time::Instant::now();
    let (tx, rx) = mpsc::channel();

    if let Some(duration) = time_constraint {
        thread::spawn(move || {
            let now = time::Instant::now();
            while now.elapsed() < duration {
                thread::sleep(time::Duration::from_millis(500));
            }
            let _ = tx.send(true);
        });
    }

    println!("{g}");
    println!("{HOW_TO}\n");

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

fn get_move(g: &Grid) -> PromptResponse<(Coord, Cell)> {
    loop {
        match get_coord() {
            PromptResponse::Val(user_cell) => {
                // adjust coord to match the zero-based array
                let acc_cell = Coord::from((user_cell.row - 1, user_cell.col - 1));

                // show user the grid they've chosen
                let mut display_g = DisplayableGrid(g.rows().clone());
                display_g.0[acc_cell.row][acc_cell.col] = Cell::Clue(0);
                println!("{display_g}\nCell {user_cell} marked with \"?\"");

                match prompt_for_value(&format!("Enter the value for cell {user_cell}\n> "), false)
                {
                    PromptResponse::Val(val) => return PromptResponse::Val((acc_cell, val)),
                    PromptResponse::Undo => return PromptResponse::Undo,
                    PromptResponse::Redo => return PromptResponse::Redo,
                    PromptResponse::Quit => return PromptResponse::Quit,
                }
            }
            PromptResponse::Undo => return PromptResponse::Undo,
            PromptResponse::Redo => return PromptResponse::Redo,
            PromptResponse::Quit => return PromptResponse::Quit,
        }
    }
}

fn get_coord() -> PromptResponse<Coord> {
    lazy_static! {
        static ref COORD_REGEX: Regex = Regex::new(r"^\D*(?P<row>\d)\D*(?P<col>\d)\D*$").unwrap();
    }
    loop {
        let r = get_response("Enter cell (format: \"row col\")\n> ");
        match r.as_str() {
            "u" => return PromptResponse::Undo,
            "r" => return PromptResponse::Redo,
            "q" => return PromptResponse::Quit,
            _ => {
                let r = COORD_REGEX.captures(&r);

                match r {
                    None => {
                        println!("Invalid format, please provide cell as \"<row> <col>\"");
                    }
                    Some(caps) => {
                        let (row, col) = (&caps["row"], &caps["col"]);
                        if let (Ok(r), Ok(c)) = (row.parse::<usize>(), col.parse::<usize>()) {
                            return PromptResponse::Val(Coord::from((r, c)));
                        }
                    }
                }
            }
        };
    }
}

fn solve() {
    let g = grid_from_input();

    match g {
        None => {
            println!("{THANK_YOU}");
            return;
        }
        Some(g) => {
            println!("{}", g);

            let now = time::Instant::now();
            let g_solved = solve_backtracking_heuristics(g);
            println!("Solved in {:?}", now.elapsed());

            if let Some(g) = g_solved {
                println!("{}", g);
            }
        }
    }
}

#[derive(Debug, Copy, Clone)]
enum Choice {
    Play,
    Solve,
    Quit,
}

fn main_menu() -> Choice {
    println!("{}{SMALL_TITLE}{}", "\n".repeat(4), "\n".repeat(4));
    let map = BTreeMap::from([
        ('p', Choice::Play),
        ('s', Choice::Solve),
        ('q', Choice::Quit),
    ]);
    println!("Play a game [p] or solve a puzzle [s]?\n\n(Enter \"q\" at any time to quit)");

    char_prompt("What would you like to do?", map, Some('p'))
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

#[derive(Debug)]
enum PromptResponse<T> {
    Quit,
    Undo,
    Redo,
    Val(T),
}

fn get_response(prompt: &str) -> String {
    loop {
        print!("{prompt}");
        let _ = std::io::stdout().flush();

        let mut response = String::new();
        if let Err(e) = io::stdin().read_line(&mut response) {
            println!("Unexpected error: {e}\nPlease try again");
        } else {
            return response.trim().to_lowercase();
        }
    }
}

fn get_char_response(prompt: &str) -> char {
    loop {
        let resp = get_response(prompt);
        match resp.len().cmp(&1) {
            Ordering::Greater => println!("Answer too long, enter a single character please"),
            Ordering::Equal => return resp.chars().next().unwrap_or_default(),
            Ordering::Less => return ' ',
        }
    }
}

fn format_chars<T>(map: &BTreeMap<char, T>) -> String
where
    T: Copy,
{
    let mut s = String::from("[");
    map.keys().enumerate().for_each(|(i, c)| {
        s.push_str(&format!(
            "{c}{}",
            if i == map.len() - 1 { "]" } else { ", " }
        ))
    });
    s
}

/// Print `prompt` and display the values that can be accepted from `map`.
/// This will loop until user enters a key from `map` or an empty string.
fn char_prompt<T>(prompt: &str, map: BTreeMap<char, T>, default: Option<char>) -> T
where
    T: Copy,
{
    let prompt = format!(
        "{prompt} {}\n{}> ",
        format_chars(&map),
        default.unwrap_or(char::default())
    );

    loop {
        let c = get_char_response(&prompt);
        if let Some(val) = map.get(&c) {
            return *val;
        } else if c == ' ' {
            if let Some(val) = default {
                return *map.get(&val).expect("The default should be in the map");
            }
        }

        println!("Please enter a value in {}", format_chars(&map));
    }
}

/// Prompts player for a response. Loops until a valid number has been entered.
fn get_num_response(prompt: &str) -> u64 {
    loop {
        let response = get_response(prompt);
        if let Ok(n) = response.parse() {
            return n;
        } else {
            println!("Please enter a valid number");
        }
    }
}

fn prompt_for_value(prompt: &str, is_clue: bool) -> PromptResponse<Cell> {
    loop {
        let response = get_response(prompt);
        if response.is_empty() {
            return PromptResponse::Val(Cell::Empty);
        }

        match response.as_str() {
            "u" => return PromptResponse::Undo,
            "r" => return PromptResponse::Redo,
            "q" => return PromptResponse::Quit,
            _ => {
                if let Ok(n) = response.parse::<u8>() {
                    if (1..=SIZE).contains(&(n as usize)) {
                        return if is_clue {
                            PromptResponse::Val(Cell::Clue(n))
                        } else {
                            PromptResponse::Val(Cell::Filled(n))
                        };
                    }
                }
                println!("Please enter a value between 1 and {SIZE}");
            }
        }
    }
}

const THANK_YOU: &str = "Thank you for playing.";

const HOW_TO: &str = "Enter numbers using the row/column indices to the left and below \
            the puzzle.\nFor example, the centre cell of the grid is at 5, 5.\
            The centre cell of the box to its left is 5, 2.\n\nWhen guessing,\
            enter the cell as \"row<space>column\", e.g. \"8 3\"";

const _BIG_TITLE: &str = r"____________________________________/\\\_____________________________ _______________
 ___________________________________\/\\\\_________________/\\\\__________________ _____
  ___________________________________\/\\\\________________\/\\\\________________ _______
   __/\\\\\\\\\\\\\__/\\\\____/\\\\________\/\\\\______/\\\\\\\____\/ \\\\\\\\\\\_____/\\\\____/\\\\_
    _\/\\\\//////__\/\\\\___\/\\\\___/\\\\\\\\\\\\____/\\\\///\\\\__ \/\\\\////\\\\__\/\\\\___\/\\\\_
     _\/\\\\\\\\\\\\\_\/\\\\___\/\\\\__/\\\\////\\\\___/\\\\__\// \\\\_\/\\\\\\\\\\\/___\/\\\\___\/\\\\_
      _\////////\\\\_\/\\\\___\/\\\\_\/\\\\__\/\\\\__\//\\\\__/\\\\ __\/\\\\///\\\\___\/\\\\___\/\\\\_
       __/\\\\\\\\\\\\\\_\//\\\\\\\\\\\\__\//\\\\\\\\\/\\\__\///\ \\\\\/___\/\\\\_\///\\\\_\//\\\\\\\\\\\\__
        _\//////////___\/////////____\///////\//_____\/////_____\///____\///___ \/////////___";

const SMALL_TITLE: &str = r"   _____           _       _          
  / ____|         | |     | |         
 | (___  _   _  __| | ___ | | ___   _ 
  \___ \| | | |/ _` |/ _ \| |/ / | | |
  ____) | |_| | (_| | (_) |   <| |_| |
 |_____/ \__,_|\__,_|\___/|_|\_\\\__,_| ";
