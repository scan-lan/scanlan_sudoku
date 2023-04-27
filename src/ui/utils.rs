use std::{
    cmp::Ordering,
    collections::BTreeMap,
    io::{self, Write},
};

use crate::{
    logic::{Cell, Coord, DisplayableGrid, Grid},
    SIZE,
};
use lazy_static::lazy_static;
use regex::Regex;

/// Get a coord from the player. The parser uses a very forgiving regex.
pub fn get_coord() -> PromptResponse<Coord> {
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

/// Formats time in seconds into "x hours, x minutes, x seconds", omitting
/// units appropriately and accounting for correct pluralisation, e.g. "25
/// minutes, 1 second".
pub fn format_time(t: u64) -> String {
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

/// Obtains a coordinate and a cell value from the player, accepting undo/redo
/// and quit.
pub fn get_move(g: &Grid) -> PromptResponse<(Coord, Cell)> {
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

#[derive(Debug)]
/// Models a player response where undo, redo, and quit are acceptable in
/// addition to the generic value `T`.
pub enum PromptResponse<T> {
    Quit,
    Undo,
    Redo,
    Val(T),
}

/// Get a string response from the player, accounting for any unexepected
/// read errors.
pub fn get_response(prompt: &str) -> String {
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

/// Gets a single-character response from the player.
pub fn get_char_response(prompt: &str) -> char {
    loop {
        let resp = get_response(prompt);
        match resp.len().cmp(&1) {
            Ordering::Greater => println!("Answer too long, enter a single character please"),
            Ordering::Equal => return resp.chars().next().unwrap_or_default(),
            Ordering::Less => return ' ',
        }
    }
}

/// Helper function to format a list of chars e.g. "[a, b, c]".
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
pub fn char_prompt<T>(prompt: &str, map: BTreeMap<char, T>, default: Option<char>) -> T
where
    T: Copy,
{
    let prompt = format!(
        "{prompt}{}\n{}> ",
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
pub fn get_num_response(prompt: &str) -> u64 {
    loop {
        let response = get_response(prompt);
        if let Ok(n) = response.parse() {
            return n;
        } else {
            println!("Please enter a valid number");
        }
    }
}

/// Prompts player for a cell value. Any empty string, including whitespace,
/// is taken as an empty cell. Validates that any number is within the
/// acceptable bounds. Can also return undo, redo, or quit.
pub fn prompt_for_value(prompt: &str, is_clue: bool) -> PromptResponse<Cell> {
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

pub const THANK_YOU: &str = "Thank you for playing.";

pub const HOW_TO: &str = "Enter numbers using the row/column indices to the left and below \
            the puzzle.\nFor example, the centre cell of the grid is at 5, 5.\
            The centre cell of the box to its left is 5, 2.\n\nWhen guessing,\
            enter the cell as \"row<space>column\", e.g. \"8 3\"";

pub const SMALL_TITLE: &str = r"   _____           _       _          
  / ____|         | |     | |         
 | (___  _   _  __| | ___ | | ___   _ 
  \___ \| | | |/ _` |/ _ \| |/ / | | |
  ____) | |_| | (_| | (_) |   <| |_| |
 |_____/ \__,_|\__,_|\___/|_|\_\\__,_| ";
