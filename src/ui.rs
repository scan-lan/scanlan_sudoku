use std::cmp::Ordering;
use std::collections::BTreeMap;
use std::io::{self, Write};
use std::time::Instant;

use crate::logic::{
    generate, solve_backtracking_heuristics, Cell, Coord, Difficulty, DisplayableGrid, SIZE,
};

use crate::logic::Grid;
use crate::puzzles::HARD_GRID;

pub fn run() {
    let choice = main_menu();

    match choice {
        Choice::Solve => solve(),
        Choice::Play => play(),
        Choice::Quit => {}
    }
}

fn difficulty_menu() -> Option<Difficulty> {
    let msg = "Select puzzle difficulty:\n\n- Easy [e]\n- Medium [m]\n- Hard[h]";
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
        let _g = generate(difficulty);
    } else {
        println!("{THANK_YOU}");
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
            }
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

fn get_response() -> String {
    loop {
        let mut response = String::new();
        if let Err(e) = io::stdin().read_line(&mut response) {
            println!("Unexpected error: {e}\nPlease try again");
        } else {
            return response.trim().to_lowercase();
        }
    }
}

fn get_char_response() -> char {
    loop {
        let resp = get_response();
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
        print!("{}", prompt);
        let _ = std::io::stdout().flush();

        let c = get_char_response();
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

fn prompt_for_value() -> PromptResponse<Cell> {
    loop {
        let response = get_response();
        if response.is_empty() {
            return PromptResponse::Val(Cell::Empty);
        }

        match response.as_str() {
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

const THANK_YOU: &str = "Thank you for playing.";

const _BIG_TITLE: &str = "\
____________________________________/\\\\\\_____________________________\
_______________
 ___________________________________\\/\\\\\\_________________/\\\\\\__________________\
_____
  ___________________________________\\/\\\\\\________________\\/\\\\\\________________\
_______
   __/\\\\\\\\\\\\\\\\\\\\__/\\\\\\____/\\\\\\________\\/\\\\\\______/\\\\\\\\\\____\\/\
       \\\\\\\\\\\\\\\\_____/\\\\\\____/\\\\\\_
    _\\/\\\\\\//////__\\/\\\\\\___\\/\\\\\\___/\\\\\\\\\\\\\\\\\\____/\\\\\\///\\\\\\__\
        \\/\\\\\\////\\\\\\__\\/\\\\\\___\\/\\\\\\_
     _\\/\\\\\\\\\\\\\\\\\\\\_\\/\\\\\\___\\/\\\\\\__/\\\\\\////\\\\\\___/\\\\\\__\\//\
         \\\\\\_\\/\\\\\\\\\\\\\\\\/___\\/\\\\\\___\\/\\\\\\_
      _\\////////\\\\\\_\\/\\\\\\___\\/\\\\\\_\\/\\\\\\__\\/\\\\\\__\\//\\\\\\__/\\\\\\\
__\\/\\\\\\///\\\\\\___\\/\\\\\\___\\/\\\\\\_
       __/\\\\\\\\\\\\\\\\\\\\_\\//\\\\\\\\\\\\\\\\\\__\\//\\\\\\\\\\\\\\/\\\\__\\///\\\
           \\\\\\\\/___\\/\\\\\\_\\///\\\\\\_\\//\\\\\\\\\\\\\\\\\\__
        _\\//////////___\\/////////____\\///////\\//_____\\/////_____\\///____\\///___\
            \\/////////___";

const SMALL_TITLE: &str = "   _____           _       _          
  / ____|         | |     | |         
 | (___  _   _  __| | ___ | | ___   _ 
  \\___ \\| | | |/ _` |/ _ \\| |/ / | | |
  ____) | |_| | (_| | (_) |   <| |_| |
 |_____/ \\__,_|\\__,_|\\___/|_|\\_\\\\__,_| ";
