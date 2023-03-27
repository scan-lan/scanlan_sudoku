// use crate::logic::{Cell, SIZE};

use super::logic::Puzzle;

pub fn run() {
    let p = Puzzle::new();
    println!("{}", p.solution().unwrap());
}

// // pub fn take_input(input: [[Cell; SIZE]; SIZE]) -> Result<Puzzle, _> {
// //     // get input etc etc
// //     Puzzle::from(input)?;
// // }

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
