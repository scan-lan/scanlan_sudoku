# Sudoku

This is my Sudoku game and automatic solver, built using Rust.

## Requirements

- Rust
- Cargo

Both can be installed by following the instructions for your platform on
<https://rustup.rs>.

## Installation

The game can be built using `cargo build --release`, then an executable for
your OS should be available at `<sudoku path>/target/release/scanlan_sudoku`.

Alternatively, the game can be run directly with the command `cargo run
--release`.

## Features

- [x] Main menu
  - [x] Play game
  - [x] Enter puzzle to be solved
- [x] Playing interface
  - [x] Input validation
  - [x] Mistake warnings
  - [x] Undo/redo
  - [x] Setting time constraints
  - [x] Timing solves
  - [x] Quit to menu
- [x] Solving interface
  - [x] Solves order 3 puzzles in microseconds
  - [x] Undo/redo
  - [x] Quit
- [x] Puzzle generation
  - [x] Difficulty settings
- [x] Order-n puzzles (code modification required)

## Changing to order-n puzzles

In its current state, the game requires code modification to change the order
of the generated puzzles. It has been made as simple as possible.

1. Open <./src/lib.rs> in a text editor of your choice.
2. Modify `const ORDER: usize = 3;` to the value of your choice, e.g `const
ORDER: usize = 4;` for a 16 x 16 puzzle.

