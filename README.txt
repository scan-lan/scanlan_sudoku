These instructions are available at https://github.com/scan-lan/scanlan_sudoku
where they're clearly formatted and easier to read.

This program was developed on MacOS. I've tried to cross-compile a windows
executable for the upload, but if it doesn't work, please follow the
instructions below or on GitHub.

# Sudoku

This is my Sudoku game and automatic solver, built using Rust.

## Requirements

- Rust
- Cargo

Both can be installed by following the instructions for your platform on
https://rustup.rs.

## Installation

### Via Cargo

This program has been published to [Crates.io](https://crates.io), so installing
and running it is as easy as

cargo install scanlan_sudoku

in your terminal, then running

scanlan_sudoku

### Manually building

The game can be built using `cargo build --release` in the terminal, then an
executable for your OS should be available at
`<sudoku path>/target/release/scanlan_sudoku`.

Alternatively, the game can be run directly with the command `cargo run
--release`.

## Features

- Main menu
  - Play game
  - Enter puzzle to be solved
- Playing interface
  - Input validation
  - Mistake warnings
  - Undo/redo
  - Setting time constraints
  - Timing solves
  - Quit to menu
- Solving interface
  - Solves order 3 puzzles in milliseconds (often microseconds)
  - Undo/redo
  - Quit
- Puzzle generation
  - Difficulty settings
- Order-n puzzles (code modification required)

## Changing to order-n puzzles

In its current state, the game requires code modification to change the order
of the generated puzzles. It has been made as simple as possible.

1. Open ./src/lib.rs in a text editor of your choice.
2. Modify `const ORDER: usize = 3;` to the value of your choice, e.g `const
ORDER: usize = 4;` for a 16 x 16 puzzle.

